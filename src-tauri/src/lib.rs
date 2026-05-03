use tauri::Manager;
use std::sync::Mutex;

// 配置管理模块
mod config;
// 监控模块
mod monitor;
// Tauri命令模块
mod commands;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            // 初始化配置管理器
            let config_dir = app
                .path()
                .app_config_dir()
                .expect("failed to get app config dir")
                .join("configs");
            
            let config_mgr = config::ConfigManager::new(config_dir);
            app.manage(Mutex::new(config_mgr));
            
            // 初始化监控状态
            let monitor_state = monitor::MonitorState::new();
            app.manage(Mutex::new(monitor_state));
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config_list,
            commands::load_config,
            commands::save_config,
            commands::create_config,
            commands::delete_config,
            commands::set_default_config,
            commands::get_current_config,
            commands::get_current_config_path,
            commands::get_settings,
            commands::add_user_history,
            commands::set_default_user,
            commands::remove_user_history,
            commands::add_path_history,
            commands::set_default_path,
            commands::remove_path_history,
            commands::get_theme_color,
            commands::set_theme_color,
            commands::toggle_stats_window,
            commands::start_monitoring,
            commands::stop_monitoring,
            commands::get_monitor_status,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
