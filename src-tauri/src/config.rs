use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub sniper_list: Vec<String>,
    #[serde(default)]
    pub join_patterns: Vec<String>,
    #[serde(default)]
    pub anti_snipe_messages: Vec<String>,
    #[serde(default)]
    pub kill_patterns: Vec<String>,
    #[serde(default)]
    pub kill_messages: Vec<String>,
    #[serde(default)]
    pub message_prefix: String,
    #[serde(default = "default_killsay_format")]
    pub killsay_format: String,
}

fn default_killsay_format() -> String {
    "%t%m".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigInfo {
    pub name: String,
    pub path: String,
    pub modified: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub value: String,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserHistory {
    pub items: Vec<HistoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathHistory {
    pub items: Vec<HistoryItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub default_config: Option<String>,
    pub theme_color: Option<String>,
    pub user_history: UserHistory,
    pub path_history: PathHistory,
}

    
    impl Default for Config {
    fn default() -> Self {
        Self {
            sniper_list: vec!["".to_string()],
            join_patterns: vec![
                "%t加入了游戏".to_string(),
                "%t 加入了游戏".to_string(),
                "%t joined the game".to_string(),
                "%t 加入游戏".to_string(),
                "%t 进入了游戏".to_string(),
            ],
            anti_snipe_messages: vec!["/PLAY SWRSOLO".to_string()],
            kill_patterns: vec![
                "%o被%u击败".to_string(),
                "%o被炸成了粉尘, 幕后黑手是%u!".to_string(),
                "%o消逝了, 幕后黑手是%u!".to_string(),
                "%o被%u用弓箭射穿了".to_string(),
                "%o跑得很快, 但是他还是摔了一跤, 最终被%u击败了".to_string(),
                "%u击败了%o".to_string(),
            ],
            kill_messages: vec![
                "黄沙百战穿金甲，不破楼兰终不还。".to_string(),
                "十步杀一人，千里不留行。".to_string(),
                "会挽雕弓如满月，西北望，射天狼。".to_string(),
                "壮志饥餐胡虏肉，笑谈渴饮匈奴血。".to_string(),
                "谈笑间，樯橹灰飞烟灭。".to_string(),
                "金戈铁马，气吞万里如虎。".to_string(),
            ],
            message_prefix: String::new(),
            killsay_format: "%t%m".to_string(),
        }
    }
}

impl Default for UserHistory {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Default for PathHistory {
    fn default() -> Self {
        Self { items: Vec::new() }
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            default_config: None,
            theme_color: Some("#F9D342".to_string()),
            user_history: UserHistory::default(),
            path_history: PathHistory::default(),
        }
    }
}

pub struct ConfigManager {
    config_dir: PathBuf,
    settings_path: PathBuf,
    current_config: Option<Config>,
    current_config_path: Option<PathBuf>,
    settings: AppSettings,
}

impl ConfigManager {
    pub fn new(config_dir: PathBuf) -> Self {
        let settings_path = config_dir.join("settings.json");
        let mut mgr = Self {
            config_dir,
            settings_path,
            current_config: None,
            current_config_path: None,
            settings: AppSettings::default(),
        };
        
        // 创建配置目录
        fs::create_dir_all(&mgr.config_dir).ok();
        
        // 加载设置
        mgr.load_settings();
        
        // 加载上次使用的配置或默认配置
        mgr.load_last_used();
        
        mgr
    }
    
    fn load_settings(&mut self) {
        if self.settings_path.exists() {
            if let Ok(content) = fs::read_to_string(&self.settings_path) {
                if let Ok(settings) = serde_json::from_str::<AppSettings>(&content) {
                    self.settings = settings;
                }
            }
        }
    }
    
    fn save_settings(&self) {
        if let Ok(content) = serde_json::to_string_pretty(&self.settings) {
            fs::write(&self.settings_path, content).ok();
        }
    }
    
    fn load_last_used(&mut self) {
        // 优先使用默认配置
        if let Some(ref default_path) = self.settings.default_config {
            let path_buf = PathBuf::from(default_path);
            if path_buf.exists() {
                self.load_config_from_path(&path_buf);
                return;
            }
        }
        
        // 创建并加载默认配置
        let default_path = self.config_dir.join("Default.json");
        if !default_path.exists() {
            let default_config = Config::default();
            self.save_config_to_path(&default_path, &default_config);
        }
        self.load_config_from_path(&default_path);
    }
    
    fn load_config_from_path(&mut self, path: &PathBuf) {
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(mut config) = serde_json::from_str::<Config>(&content) {
                // 确保配置包含所有必要字段
                let default = Config::default();
                if config.sniper_list.is_empty() {
                    config.sniper_list = default.sniper_list;
                }
                if config.join_patterns.is_empty() {
                    config.join_patterns = default.join_patterns;
                }
                if config.kill_patterns.is_empty() {
                    config.kill_patterns = default.kill_patterns;
                }
                if config.kill_messages.is_empty() {
                    config.kill_messages = default.kill_messages;
                }
                
                self.current_config = Some(config);
                self.current_config_path = Some(path.clone());
            }
        }
    }
    
    fn save_config_to_path(&self, path: &PathBuf, config: &Config) {
        if let Ok(content) = serde_json::to_string_pretty(config) {
            fs::write(path, content).ok();
        }
    }
    
    pub fn get_config_list(&self) -> Vec<ConfigInfo> {
        let mut configs = Vec::new();
        let default_config = self.settings.default_config.clone();
        
        if let Ok(entries) = fs::read_dir(&self.config_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "json") {
                    let file_name = path.file_name().map_or(false, |name| {
                        let name_str = name.to_string_lossy();
                        name_str != "last_used.json" && name_str != "settings.json"
                    });
                    
                    if file_name {
                        let name = path.file_stem()
                            .map(|n| n.to_string_lossy().to_string())
                            .unwrap_or_default();
                        
                        let path_str = path.to_string_lossy().to_string();
                        let is_default = default_config.as_ref() == Some(&path_str);
                        
                        let modified = path.metadata()
                            .and_then(|m| m.modified())
                            .map(|t| {
                                let datetime: chrono::DateTime<chrono::Local> = t.into();
                                datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                            })
                            .unwrap_or_default();
                        
                        configs.push(ConfigInfo {
                            name,
                            path: path_str,
                            modified,
                            is_default,
                        });
                    }
                }
            }
        }
        
        configs.sort_by(|a, b| {
            if a.is_default && !b.is_default {
                std::cmp::Ordering::Less
            } else if !a.is_default && b.is_default {
                std::cmp::Ordering::Greater
            } else {
                b.modified.cmp(&a.modified)
            }
        });
        configs
    }
    
    pub fn load_config(&mut self, path: &str) -> Result<Config, String> {
        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            return Err("配置文件不存在".to_string());
        }
        
        self.load_config_from_path(&path_buf);
        self.current_config.clone().ok_or_else(|| "加载配置失败".to_string())
    }
    
    pub fn save_config(&mut self, path: &str, config: &Config) -> Result<(), String> {
        let path_buf = PathBuf::from(path);
        self.save_config_to_path(&path_buf, config);
        
        // 如果是当前配置，更新内存中的配置
        if self.current_config_path.as_ref() == Some(&path_buf) {
            self.current_config = Some(config.clone());
        }
        
        Ok(())
    }
    
    pub fn create_config(&mut self, name: &str) -> Result<Config, String> {
        // 禁止使用保留名称
        let reserved_names = ["settings", "last_used"];
        let name_lower = name.to_lowercase();
        if reserved_names.contains(&name_lower.as_str()) {
            return Err("该名称为系统保留，请使用其他名称".to_string());
        }
        
        let path = self.config_dir.join(format!("{}.json", name));
        if path.exists() {
            return Err("配置文件已存在".to_string());
        }
        
        let config = Config::default();
        self.save_config_to_path(&path, &config);
        self.load_config_from_path(&path);
        
        Ok(config)
    }
    
    pub fn delete_config(&mut self, path: &str) -> Result<(), String> {
        let path_buf = PathBuf::from(path);
        
        // 不能删除当前正在使用的配置
        if self.current_config_path.as_ref() == Some(&path_buf) {
            return Err("不能删除当前正在使用的配置".to_string());
        }
        
        // 如果是默认配置，清除默认设置
        if self.settings.default_config.as_ref() == Some(&path.to_string()) {
            self.settings.default_config = None;
            self.save_settings();
        }
        
        if path_buf.exists() {
            fs::remove_file(&path_buf).map_err(|e| e.to_string())?;
            Ok(())
        } else {
            Err("配置文件不存在".to_string())
        }
    }
    
    pub fn set_default_config(&mut self, path: &str) -> Result<(), String> {
        let path_buf = PathBuf::from(path);
        if !path_buf.exists() {
            return Err("配置文件不存在".to_string());
        }
        
        self.settings.default_config = Some(path.to_string());
        self.save_settings();
        Ok(())
    }
    
    pub fn get_current_config(&self) -> Option<Config> {
        self.current_config.clone()
    }
    
    pub fn get_current_config_path(&self) -> Option<String> {
        self.current_config_path.as_ref().map(|p| p.to_string_lossy().to_string())
    }
    
    pub fn get_settings(&self) -> AppSettings {
        self.settings.clone()
    }
    
    pub fn add_user_history(&mut self, user: &str) {
        if user.is_empty() {
            return;
        }
        
        // 移除已存在的相同项
        self.settings.user_history.items.retain(|item| item.value != user);
        
        // 添加到开头
        self.settings.user_history.items.insert(0, HistoryItem {
            value: user.to_string(),
            is_default: false,
        });
        
        // 限制历史记录数量
        if self.settings.user_history.items.len() > 20 {
            self.settings.user_history.items.truncate(20);
        }
        
        self.save_settings();
    }
    
    pub fn set_default_user(&mut self, user: &str) -> Result<(), String> {
        for item in &mut self.settings.user_history.items {
            item.is_default = item.value == user;
        }
        self.save_settings();
        Ok(())
    }
    
    pub fn remove_user_history(&mut self, user: &str) {
        self.settings.user_history.items.retain(|item| item.value != user);
        self.save_settings();
    }
    
    pub fn add_path_history(&mut self, path: &str) {
        if path.is_empty() {
            return;
        }
        
        // 移除已存在的相同项
        self.settings.path_history.items.retain(|item| item.value != path);
        
        // 添加到开头
        self.settings.path_history.items.insert(0, HistoryItem {
            value: path.to_string(),
            is_default: false,
        });
        
        // 限制历史记录数量
        if self.settings.path_history.items.len() > 20 {
            self.settings.path_history.items.truncate(20);
        }
        
        self.save_settings();
    }
    
    pub fn set_default_path(&mut self, path: &str) -> Result<(), String> {
        for item in &mut self.settings.path_history.items {
            item.is_default = item.value == path;
        }
        self.save_settings();
        Ok(())
    }
    
    pub fn remove_path_history(&mut self, path: &str) {
        self.settings.path_history.items.retain(|item| item.value != path);
        self.save_settings();
    }
    
    pub fn get_theme_color(&self) -> String {
        self.settings.theme_color.clone().unwrap_or_else(|| "#F9D342".to_string())
    }
    
    pub fn set_theme_color(&mut self, color: &str) {
        self.settings.theme_color = Some(color.to_string());
        self.save_settings();
    }
}
