use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}};
use std::thread;
use std::time::Duration;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::Path;
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitorParams {
    pub user: String,
    pub window_name: String,
    pub log_file_path: String,
    pub sniper_list: Vec<String>,
    pub join_patterns: Vec<String>,
    pub anti_snipe_messages: Vec<String>,
    pub kill_patterns: Vec<String>,
    pub kill_messages: Vec<String>,
    pub anti_snipe_enabled: bool,
    #[serde(default)]
    pub message_prefix: String,
    #[serde(default = "default_killsay_format")]
    pub killsay_format: String,
}

fn default_killsay_format() -> String {
    "%t%m".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct MonitorStatus {
    pub is_running: bool,
    pub kills: u32,
    pub deaths: u32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum MonitorEvent {
    Kill {
        killer: String,
        victim: String,
        message: String,
        kills: u32,
        deaths: u32,
    },
    Death {
        victim: String,
        kills: u32,
        deaths: u32,
    },
    AntiSnipe {
        player: String,
    },
    Error {
        message: String,
    },
    StatusChanged {
        is_running: bool,
    },
}

pub struct MonitorState {
    is_running: Arc<AtomicBool>,
    kills: Arc<Mutex<u32>>,
    deaths: Arc<Mutex<u32>>,
    handle: Option<thread::JoinHandle<()>>,
}

impl MonitorState {
    pub fn new() -> Self {
        Self {
            is_running: Arc::new(AtomicBool::new(false)),
            kills: Arc::new(Mutex::new(0)),
            deaths: Arc::new(Mutex::new(0)),
            handle: None,
        }
    }
    
    pub fn start(&mut self, params: MonitorParams, event_sender: impl Fn(MonitorEvent) + Send + 'static) -> Result<(), String> {
        if self.is_running.load(Ordering::Relaxed) {
            return Err("监控已在运行中".to_string());
        }
        
        // 验证日志文件
        if !Path::new(&params.log_file_path).exists() {
            return Err("日志文件不存在".to_string());
        }
        
        self.is_running.store(true, Ordering::Relaxed);
        *self.kills.lock().unwrap() = 0;
        *self.deaths.lock().unwrap() = 0;
        
        let is_running = self.is_running.clone();
        let kills = self.kills.clone();
        let deaths = self.deaths.clone();
        
        let handle = thread::spawn(move || {
            monitor_loop(params, is_running, kills, deaths, event_sender);
        });
        
        self.handle = Some(handle);
        
        Ok(())
    }
    
    pub fn stop(&mut self) {
        self.is_running.store(false, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            handle.join().ok();
        }
    }
    
    pub fn get_status(&self) -> MonitorStatus {
        MonitorStatus {
            is_running: self.is_running.load(Ordering::Relaxed),
            kills: *self.kills.lock().unwrap(),
            deaths: *self.deaths.lock().unwrap(),
        }
    }
}

fn monitor_loop(
    params: MonitorParams,
    is_running: Arc<AtomicBool>,
    kills: Arc<Mutex<u32>>,
    deaths: Arc<Mutex<u32>>,
    event_sender: impl Fn(MonitorEvent),
) {
    let file = match File::open(&params.log_file_path) {
        Ok(f) => f,
        Err(e) => {
            event_sender(MonitorEvent::Error {
                message: format!("无法打开日志文件: {}", e),
            });
            is_running.store(false, Ordering::Relaxed);
            return;
        }
    };
    
    let mut reader = BufReader::new(file);
    reader.seek(SeekFrom::End(0)).ok();
    
    // 编译正则表达式
    let join_regexes: Vec<Regex> = params.join_patterns
        .iter()
        .filter_map(|p| {
            let escaped = regex::escape(p).replace(r"%t", "(.+?)");
            Regex::new(&escaped).ok()
        })
        .collect();
    
    let kill_regexes: Vec<(Regex, usize, usize)> = params.kill_patterns
        .iter()
        .filter_map(|p| {
            let o_idx = p.find("%o")?;
            let u_idx = p.find("%u")?;
            let escaped = regex::escape(p)
                .replace(r"%o", "(.*)")
                .replace(r"%u", "(.*)")
                .replace(r"%n", r"\d+");
            Regex::new(&escaped).ok().map(|r| (r, o_idx, u_idx))
        })
        .collect();
    
    event_sender(MonitorEvent::StatusChanged { is_running: true });
    
    let mut line = String::new();
    
    while is_running.load(Ordering::Relaxed) {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                // 没有新行，等待
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            Ok(_) => {
                let line = line.trim();
                if line.contains("[CHAT]") {
                    if let Some(chat_part) = line.split("[CHAT]").nth(1) {
                        let chat = chat_part.trim();
                        
                        // 处理击杀
                        process_kill(
                            chat,
                            &params.user,
                            &kill_regexes,
                            &params.kill_messages,
                            &kills,
                            &deaths,
                            &params.message_prefix,
                            &params.killsay_format,
                            &event_sender,
                        );
                        
                        // 处理反狙击
                        if params.anti_snipe_enabled {
                            process_anti_snipe(
                                chat,
                                &params.sniper_list,
                                &join_regexes,
                                &event_sender,
                            );
                        }
                    }
                } else if params.anti_snipe_enabled {
                    // 检查加入消息
                    if line.contains("加入了游戏") || line.contains("joined the game") || line.contains("加入游戏") {
                        process_anti_snipe(
                            line,
                            &params.sniper_list,
                            &join_regexes,
                            &event_sender,
                        );
                    }
                }
            }
            Err(_) => {
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
    
    event_sender(MonitorEvent::StatusChanged { is_running: false });
}

fn process_kill(
    msg: &str,
    user: &str,
    kill_regexes: &[(Regex, usize, usize)],
    kill_messages: &[String],
    kills: &Arc<Mutex<u32>>,
    deaths: &Arc<Mutex<u32>>,
    message_prefix: &str,
    killsay_format: &str,
    event_sender: &impl Fn(MonitorEvent),
) {
    for (regex, o_idx, u_idx) in kill_regexes {
        if let Some(captures) = regex.captures(msg) {
            if captures.len() >= 3 {
                let (d, k) = if o_idx < u_idx {
                    (captures[1].to_string(), captures[2].to_string())
                } else {
                    (captures[2].to_string(), captures[1].to_string())
                };
                
                let d = d.trim().to_string();
                let k = k.trim().to_string();
                
                if k == user {
                    let mut kills_count = kills.lock().unwrap();
                    *kills_count += 1;
                    
                    let message = get_random_kill_message(kill_messages);
                    let formatted = format_message(&message, &k, &d, *kills_count, message_prefix, killsay_format);
                    
                    event_sender(MonitorEvent::Kill {
                        killer: k,
                        victim: d.clone(),
                        message: formatted,
                        kills: *kills_count,
                        deaths: *deaths.lock().unwrap(),
                    });
                }
                
                if d == user {
                    let mut deaths_count = deaths.lock().unwrap();
                    *deaths_count += 1;
                    
                    event_sender(MonitorEvent::Death {
                        victim: d,
                        kills: *kills.lock().unwrap(),
                        deaths: *deaths_count,
                    });
                }
            }
            break;
        }
    }
}

fn process_anti_snipe(
    msg: &str,
    sniper_list: &[String],
    join_regexes: &[Regex],
    event_sender: &impl Fn(MonitorEvent),
) {
    for regex in join_regexes {
        if let Some(captures) = regex.captures(msg) {
            if let Some(player_match) = captures.get(1) {
                let player = player_match.as_str().trim().to_string();
                
                for sniper in sniper_list {
                    if sniper.is_empty() {
                        continue;
                    }
                    if player == *sniper || player.contains(sniper) || sniper.contains(&player) {
                        event_sender(MonitorEvent::AntiSnipe {
                            player: player.clone(),
                        });
                        return;
                    }
                }
            }
            break;
        }
    }
}

fn get_random_kill_message(messages: &[String]) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    if messages.is_empty() {
        return String::new();
    }
    
    let mut hasher = DefaultHasher::new();
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .hash(&mut hasher);
    let index = hasher.finish() as usize % messages.len();
    
    messages[index].clone()
}

fn format_message(template: &str, killer: &str, victim: &str, kills: u32, prefix: &str, format: &str) -> String {
    let message = template
        .replace("%u", killer)
        .replace("%o", victim);
    
    format
        .replace("%t", prefix)
        .replace("%m", &message)
        .replace("%u", killer)
        .replace("%o", victim)
        .replace("%c", &kills.to_string())
}
