// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use serde::Deserialize;
use std::path::PathBuf;
use tauri::{Manager, WebviewUrl, WebviewWindowBuilder};
use tauri_plugin_shell::ShellExt;

/// Memos startup parameters, loaded from external config.yaml
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
struct MemosConfig {
    /// Address the Memos server listens on
    host: String,
    /// Port the Memos server listens on
    port: u16,
    /// Data directory (defaults to executable directory if not set)
    data: Option<String>,
    /// Window width
    width: f64,
    /// Window height
    height: f64,
    /// Extra arguments passed through to the memos server
    extra_args: Vec<String>
}

impl Default for MemosConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 5230,
            data: None,
            width: 1920.0,
            height: 1080.0,
            extra_args: Vec::new(),
        }
    }
}

impl MemosConfig {
    /// URL of the Memos Web UI to load in the window
    fn url(&self) -> String {
        format!("http://{}:{}", self.host, self.port)
    }

    /// Resolve data directory: use configured value, or default to executable directory
    fn data_dir(&self) -> PathBuf {
        match &self.data {
            Some(d) if !d.trim().is_empty() => PathBuf::from(d),
            _ => std::env::current_exe()
                .ok()
                .and_then(|p| p.parent().map(|d| d.to_path_buf()))
                .unwrap_or_else(|| PathBuf::from(".")),
        }
    }
}

/// Config file lookup order:
/// 1. config.yaml next to the executable (src-tauri/target/debug/config.yaml in dev)
/// 2. System app config directory (Windows: %APPDATA%/memos-desktop/config.yaml)
fn find_config_path(app: &tauri::AppHandle) -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            let p = dir.join("config.yaml");
            if p.is_file() {
                return Some(p);
            }
        }
    }
    app.path()
        .app_config_dir()
        .ok()
        .map(|dir| dir.join("config.yaml")) 
}

fn load_config(app: &tauri::AppHandle) -> MemosConfig {
    let Some(path) = find_config_path(app) else {
        return MemosConfig::default();
    };
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read {}: {}, using default config", path.display(), e);
            return MemosConfig::default();
        }
    };
    match serde_yaml::from_str::<MemosConfig>(&content) {
        Ok(cfg) => cfg,
        Err(e) => {
            eprintln!("Failed to parse {}: {}, using default config", path.display(), e);
            MemosConfig::default()
        }
    }
}

/// Convert shell plugin error to tauri error
fn shell_err(e: tauri_plugin_shell::Error) -> tauri::Error {
    tauri::Error::from(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))
}

/// Spawn the memos sidecar process
fn spawn_memos(app: &tauri::AppHandle, cfg: &MemosConfig) -> tauri::Result<()> {
    let mut args: Vec<String> = vec![
        "--port".to_string(),
        cfg.port.to_string(),
        "--data".to_string(),
        cfg.data_dir().to_string_lossy().to_string(),
    ];
    args.extend(cfg.extra_args.clone());

    let command = app.shell().sidecar("memos").map_err(shell_err)?.args(args);
    let (_rx, _child) = command.spawn().map_err(shell_err)?;
    // To kill the process on exit, store child in app.state()
    Ok(())
}

/// Poll until the memos port is ready, so the window doesn't open before the server is up
fn wait_for_port(host: &str, port: u16, timeout: std::time::Duration) -> bool {
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if std::net::TcpStream::connect((host, port)).is_ok() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(200));
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // Read host/port from config.yaml, create window dynamically
            let handle = app.handle().clone();
            let cfg = load_config(&handle);

            // Start memos server first, wait for port to be ready before opening window
            spawn_memos(&handle, &cfg)?;
            if !wait_for_port(&cfg.host, cfg.port, std::time::Duration::from_secs(15)) {
                eprintln!("Memos server not ready within 15s; window may fail to load, try refreshing later");
            }

            let url: url::Url = cfg
                .url()
                .parse()
                .map_err(|e: url::ParseError| {
                    tauri::Error::from(std::io::Error::new(std::io::ErrorKind::InvalidInput, e.to_string()))
                })?;
            WebviewWindowBuilder::new(app, "main", WebviewUrl::External(url))
                .title("Memos")
                .inner_size(cfg.width, cfg.height)
                .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
