use reqwest::header::{ACCEPT, USER_AGENT};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub has_update: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_url: String,
    pub release_notes: String,
    pub debug_info: String,
}

#[derive(Debug, Deserialize)]
struct GitHubRelease {
    tag_name: String,
    name: String,
    body: String,
    html_url: String,
    prerelease: bool,
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

#[tauri::command]
pub async fn check_for_update() -> Result<UpdateInfo, String> {
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

    let releases: Vec<GitHubRelease> = resp
        .json()
        .await
        .map_err(|e| format!("解析响应失败: {}", e))?;

    let current = current_version();
    let mut debug_lines = Vec::new();
    debug_lines.push(format!("当前版本: {}", current));
    debug_lines.push(format!("获取到 {} 个 releases", releases.len()));
    
    // Log all tags
    for r in &releases {
        debug_lines.push(format!("  tag: {} (prerelease: {})", r.tag_name, r.prerelease));
    }
    
    // Find the latest non-prerelease
    let latest = releases
        .iter()
        .filter(|r| !r.prerelease)
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
            debug_lines.push("没有找到非 prerelease 的 release".to_string());
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
