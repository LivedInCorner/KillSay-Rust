use std::fs;
use std::io::Write;
use std::process::Command;

use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
    pub debug_info: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub downloaded: u64,
    pub total: u64,
    pub percentage: u32,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    body: String,
    html_url: String,
    prerelease: bool,
    assets: Vec<GitHubAsset>,
}

#[derive(Debug, Deserialize)]
struct GitHubAsset {
    name: String,
    browser_download_url: String,
    size: u64,
    content_type: String,
}

const GITHUB_API: &str = "https://api.github.com/repos/LivedInCorner/KillSay-Rust/releases";

fn strip_v(s: &str) -> String {
    s.trim().strip_prefix('v').or_else(|| s.trim().strip_prefix('V')).unwrap_or(s.trim()).to_string()
}

fn version_greater(a: &str, b: &str) -> bool {
    let pa: Vec<u32> = a.split('.').filter_map(|s| s.parse().ok()).collect();
    let pb: Vec<u32> = b.split('.').filter_map(|s| s.parse().ok()).collect();
    let len = pa.len().max(pb.len());
    for i in 0..len {
        let va = pa.get(i).copied().unwrap_or(0);
        let vb = pb.get(i).copied().unwrap_or(0);
        if va > vb { return true; }
        if va < vb { return false; }
    }
    false
}

fn current_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

async fn fetch_releases() -> Result<Vec<GitHubRelease>, String> {
    let client = reqwest::Client::new();
    let resp = client
        .get(GITHUB_API)
        .query(&[("per_page", "10")])
        .header(USER_AGENT, "KillSay-Updater/1.0")
        .header(ACCEPT, "application/vnd.github+json")
        .send()
        .await
        .map_err(|e| format!("网络请求失败: {}", e))?;

    if !resp.status().is_success() {
        return Err(format!("GitHub API 返回错误: {}", resp.status()));
    }

    resp.json::<Vec<GitHubRelease>>()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))
}

fn find_msi_asset(release: &GitHubRelease) -> Option<&GitHubAsset> {
    release.assets.iter().find(|a| a.name.ends_with(".msi"))
}

#[tauri::command]
pub async fn check_for_update() -> Result<UpdateInfo, String> {
    let releases = fetch_releases().await?;
    let current = current_version();
    let mut debug_lines = Vec::new();
    debug_lines.push(format!("当前版本: {}", current));
    debug_lines.push(format!("获取到 {} 个 releases", releases.len()));
    
    for r in &releases {
        let has_msi = find_msi_asset(r).is_some();
        debug_lines.push(format!("  tag: {} (prerelease: {}, msi: {})", r.tag_name, r.prerelease, has_msi));
    }
    
    let latest = releases
        .iter()
        .filter(|r| !r.prerelease && find_msi_asset(r).is_some())
        .max_by(|a, b| {
            let va = strip_v(&a.tag_name);
            let vb = strip_v(&b.tag_name);
            if version_greater(&va, &vb) {
                std::cmp::Ordering::Greater
            } else if version_greater(&vb, &va) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Equal
            }
        });

    match latest {
        Some(release) => {
            let latest_version = strip_v(&release.tag_name);
            let has_update = version_greater(&latest_version, &current);
            debug_lines.push(format!("最新版本: {} (有更新: {})", latest_version, has_update));
            
            Ok(UpdateInfo {
                has_update,
                current_version: current,
                latest_version,
                release_url: release.html_url.clone(),
                release_notes: release.body.clone(),
                debug_info: debug_lines.join("\n"),
            })
        }
        None => {
            debug_lines.push("没有找到包含 MSI 的 release".to_string());
            Ok(UpdateInfo {
                has_update: false,
                current_version: current.clone(),
                latest_version: current,
                release_url: String::new(),
                release_notes: String::new(),
                debug_info: debug_lines.join("\n"),
            })
        }
    }
}

#[tauri::command]
pub async fn download_update(app: AppHandle, version: String) -> Result<String, String> {
    let releases = fetch_releases().await?;
    
    let release = releases
        .iter()
        .find(|r| strip_v(&r.tag_name) == version)
        .ok_or_else(|| format!("未找到版本 {} 的 release", version))?;
    
    let asset = find_msi_asset(release)
        .ok_or_else(|| "未找到 MSI 安装包".to_string())?;
    
    let temp_dir = std::env::temp_dir().join("killsay_update");
    fs::create_dir_all(&temp_dir)
        .map_err(|e| format!("创建临时目录失败: {}", e))?;
    
    let file_path = temp_dir.join(&asset.name);
    let file_path_str = file_path.to_string_lossy().to_string();
    
    let client = reqwest::Client::new();
    let mut resp = client
        .get(&asset.browser_download_url)
        .header(USER_AGENT, "KillSay-Updater/1.0")
        .send()
        .await
        .map_err(|e| format!("下载失败: {}", e))?;
    
    let total = resp.content_length().unwrap_or(asset.size);
    let mut downloaded: u64 = 0;
    let mut last_emit: u64 = 0;
    let mut file = fs::File::create(&file_path)
        .map_err(|e| format!("创建文件失败: {}", e))?;
    
    while let Some(chunk) = resp.chunk().await
        .map_err(|e| format!("下载读取失败: {}", e))?
    {
        file.write_all(&chunk)
            .map_err(|e| format!("写入文件失败: {}", e))?;
        downloaded += chunk.len() as u64;
        
        let percentage = if total > 0 { (downloaded * 100 / total) as u32 } else { 0 };
        
        if downloaded.saturating_sub(last_emit) >= 256 * 1024 || downloaded >= total {
            let _ = app.emit("update-download-progress", DownloadProgress {
                downloaded,
                total,
                percentage,
            });
            last_emit = downloaded;
        }
    }
    
    file.flush().map_err(|e| format!("刷新文件失败: {}", e))?;
    
    let _ = app.emit("update-download-progress", DownloadProgress {
        downloaded: total,
        total,
        percentage: 100,
    });
    
    Ok(file_path_str)
}

#[tauri::command]
pub fn install_update(installer_path: String) -> Result<(), String> {
    let path = std::path::Path::new(&installer_path);
    if !path.exists() {
        return Err("安装文件不存在".to_string());
    }
    
    // 启动 MSI 安装器
    Command::new("msiexec")
        .args(["/i", &installer_path])
        .spawn()
        .map_err(|e| format!("启动安装器失败: {}", e))?;
    
    // 退出应用
    std::process::exit(0);
}
