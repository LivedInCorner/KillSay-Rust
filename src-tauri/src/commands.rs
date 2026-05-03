use tauri::{Emitter, Manager, State};
use crate::config::{Config, ConfigInfo, ConfigManager, AppSettings};
use crate::monitor::{MonitorParams, MonitorState};
use std::sync::Mutex;

#[tauri::command]
pub fn get_config_list(config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<Vec<ConfigInfo>, String> {
    let mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_config_list())
}

#[tauri::command]
pub fn load_config(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<Config, String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.load_config(&path)
}

#[tauri::command]
pub fn save_config(path: String, config: Config, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.save_config(&path, &config)
}

#[tauri::command]
pub fn create_config(name: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<Config, String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.create_config(&name)
}

#[tauri::command]
pub fn delete_config(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.delete_config(&path)
}

#[tauri::command]
pub fn set_default_config(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.set_default_config(&path)
}

#[tauri::command]
pub fn get_current_config(config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<Option<Config>, String> {
    let mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_current_config())
}

#[tauri::command]
pub fn get_current_config_path(config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<Option<String>, String> {
    let mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_current_config_path())
}

#[tauri::command]
pub fn get_settings(config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<AppSettings, String> {
    let mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_settings())
}

#[tauri::command]
pub fn add_user_history(user: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.add_user_history(&user);
    Ok(())
}

#[tauri::command]
pub fn set_default_user(user: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.set_default_user(&user)
}

#[tauri::command]
pub fn remove_user_history(user: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.remove_user_history(&user);
    Ok(())
}

#[tauri::command]
pub fn add_path_history(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.add_path_history(&path);
    Ok(())
}

#[tauri::command]
pub fn set_default_path(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.set_default_path(&path)
}

#[tauri::command]
pub fn remove_path_history(path: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.remove_path_history(&path);
    Ok(())
}

#[tauri::command]
pub fn get_theme_color(config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<String, String> {
    let mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    Ok(mgr.get_theme_color())
}

#[tauri::command]
pub fn set_theme_color(color: String, config_mgr: State<'_, Mutex<ConfigManager>>) -> Result<(), String> {
    let mut mgr = config_mgr.lock().map_err(|e| e.to_string())?;
    mgr.set_theme_color(&color);
    Ok(())
}

#[tauri::command]
pub fn start_monitoring(
    params: MonitorParams,
    monitor_state: State<'_, Mutex<MonitorState>>,
    app_handle: tauri::AppHandle,
) -> Result<(), String> {
    let mut state = monitor_state.lock().map_err(|e| e.to_string())?;
    
    let handle = app_handle.clone();
    state.start(params, move |event| {
        // 通过Tauri事件系统发送事件到前端
        handle.emit("monitor-event", event).ok();
    })
}

#[tauri::command]
pub fn stop_monitoring(monitor_state: State<'_, Mutex<MonitorState>>) -> Result<(), String> {
    let mut state = monitor_state.lock().map_err(|e| e.to_string())?;
    state.stop();
    Ok(())
}

#[tauri::command]
pub fn get_monitor_status(monitor_state: State<'_, Mutex<MonitorState>>) -> Result<crate::monitor::MonitorStatus, String> {
    let state = monitor_state.lock().map_err(|e| e.to_string())?;
    Ok(state.get_status())
}

#[tauri::command]
pub fn toggle_stats_window(app_handle: tauri::AppHandle) -> Result<(), String> {
    use tauri::{WebviewUrl, WebviewWindowBuilder};
    
    let label = "stats";
    
    // Check if window already exists
    if let Some(window) = app_handle.get_webview_window(label) {
        if let Ok(is_visible) = window.is_visible() {
            if is_visible {
                let _ = window.hide();
            } else {
                let _ = window.show();
            }
        }
        return Ok(());
    }
    
    // Create new stats window using main app with hash
    let _window = WebviewWindowBuilder::new(
        &app_handle,
        label,
        WebviewUrl::App("index.html#/stats".into()),
    )
    .title("KILLSAY Stats")
    .inner_size(300.0, 90.0)
    .decorations(false)
    .always_on_top(true)
    .resizable(false)
    .skip_taskbar(true)
    .focused(false)
    .visible(true)
    .build()
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
