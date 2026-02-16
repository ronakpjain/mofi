use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ColorConfig {
    pub background: String,
    pub border: String,
    pub text: String,
    pub selected_bg: String,
    pub selected_text: String,
}

impl Default for ColorConfig {
    fn default() -> Self {
        Self {
            background: "#1e1e2e".to_string(),
            border: "#fab387".to_string(),
            text: "#fab387".to_string(),
            selected_bg: "#fab387".to_string(),
            selected_text: "#1e1e2e".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct MofiConfig {
    pub colors: Option<ColorConfig>,
    pub aliases: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppInfo {
    pub name: String,
    pub path: String,
}

/// Recursively search for .app bundles in a directory
fn find_apps_recursive(dir: &Path, apps: &mut Vec<AppInfo>) -> std::io::Result<()> {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();

            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // If this is an .app bundle, add it
                if name.ends_with(".app") {
                    let app_name = name.trim_end_matches(".app").to_string();
                    let full_path = path.to_string_lossy().to_string();
                    apps.push(AppInfo {
                        name: app_name,
                        path: full_path,
                    });
                }
                // Recursively search subdirectories (but not inside .app bundles)
                else if path.is_dir() && !name.starts_with('.') {
                    // Avoid descending too deep into system directories
                    // but allow one level of nesting (e.g., Applications/Folder/App.app)
                    if let Ok(subdirs) = fs::read_dir(&path) {
                        let mut has_app = false;
                        for subentry in subdirs.flatten() {
                            let subpath = subentry.path();
                            if let Some(subname) = subpath.file_name().and_then(|n| n.to_str()) {
                                if subname.ends_with(".app") {
                                    has_app = true;
                                    let app_name = subname.trim_end_matches(".app").to_string();
                                    let full_path = subpath.to_string_lossy().to_string();
                                    apps.push(AppInfo {
                                        name: app_name,
                                        path: full_path,
                                    });
                                }
                            }
                        }
                        // Only recursively search if this directory didn't contain apps
                        if !has_app {
                            let _ = find_apps_recursive(&path, apps);
                        }
                    }
                }
            }
        }
    }
    Ok(())
}

#[tauri::command]
fn list_apps() -> Vec<AppInfo> {
    let mut apps = Vec::new();

    let app_dirs = vec![
        PathBuf::from("/Applications"),
        PathBuf::from("/System/Applications"),
        PathBuf::from("/System/Library/CoreServices"),
    ];

    for dir in app_dirs {
        let _ = find_apps_recursive(&dir, &mut apps);
    }

    // Sort by app name for consistent ordering
    apps.sort_by(|a, b| a.name.cmp(&b.name));
    apps
}

#[tauri::command]
fn launch_app(app_path: String, app_name: String) -> Result<String, String> {
    let config = load_mofi_config();
    if let Some(aliases) = config.aliases {
        if let Some((_, command)) = aliases
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(&app_name))
        {
            return run_alias_command(command);
        }
    }

    if !Path::new(&app_path).exists() {
        return Err(format!("App not found at '{}'", app_path));
    }

    let status = Command::new("open")
        .arg(&app_path)
        .status()
        .map_err(|e| format!("Failed to launch app: {}", e))?;

    if status.success() {
        Ok("App opened successfully".to_string())
    } else {
        Err("Failed to open app: exited with non-zero code".to_string())
    }
}

#[tauri::command]
fn load_color_config() -> ColorConfig {
    let config = load_mofi_config();
    config.colors.unwrap_or_default()
}

fn load_mofi_config() -> MofiConfig {
    let home_dir = match std::env::var("HOME") {
        Ok(home) => home,
        Err(_) => return MofiConfig::default(),
    };

    let mut config_path = PathBuf::from(home_dir.clone());
    config_path.push(".config/mofi/mofi.toml");

    let mut config = if let Ok(contents) = fs::read_to_string(&config_path) {
        toml::from_str::<MofiConfig>(&contents).unwrap_or_default()
    } else {
        MofiConfig::default()
    };

    if config.colors.is_none() {
        let mut colors_path = PathBuf::from(home_dir.clone());
        colors_path.push(".config/mofi/colors.toml");
        if let Ok(contents) = fs::read_to_string(&colors_path) {
            config.colors = toml::from_str::<ColorConfig>(&contents).ok();
        }
    }

    config
}

fn run_alias_command(command: &str) -> Result<String, String> {
    let status = Command::new("sh")
        .arg("-c")
        .arg(command)
        .status()
        .map_err(|e| format!("Failed to run alias command: {}", e))?;

    if status.success() {
        Ok("Command executed successfully".to_string())
    } else {
        Err("Alias command failed: exited with non-zero code".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            launch_app,
            list_apps,
            load_color_config
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
