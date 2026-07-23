// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_shell::ShellExt;

#[derive(Serialize, Deserialize, Clone)]
struct Settings {
    ask_every_time: bool,
    thread_count: u32,
    trim_none: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ask_every_time: true,
            // Matches iso2god-rs's own CLI default (-j defaults to 1).
            thread_count: 1,
            trim_none: false,
        }
    }
}

#[derive(Serialize)]
struct SettingsResponse {
    configured: bool,
    ask_every_time: bool,
    thread_count: u32,
    trim_none: bool,
    cpu_count: u32,
}

fn cpu_count() -> u32 {
    std::thread::available_parallelism()
        .map(|n| n.get() as u32)
        .unwrap_or(4)
}

fn settings_path(app: &AppHandle) -> Result<PathBuf, String> {
    let dir = app
        .path()
        .app_config_dir()
        .map_err(|e| e.to_string())?;
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    Ok(dir.join("settings.json"))
}

#[tauri::command]
fn get_settings(app: AppHandle) -> Result<SettingsResponse, String> {
    let path = settings_path(&app)?;
    let (configured, settings) = match fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str::<Settings>(&text) {
            Ok(s) => (true, s),
            Err(_) => (false, Settings::default()),
        },
        Err(_) => (false, Settings::default()),
    };

    Ok(SettingsResponse {
        configured,
        ask_every_time: settings.ask_every_time,
        thread_count: settings.thread_count.max(1),
        trim_none: settings.trim_none,
        cpu_count: cpu_count(),
    })
}

#[tauri::command]
fn save_settings(
    app: AppHandle,
    ask_every_time: bool,
    thread_count: u32,
    trim_none: bool,
) -> Result<(), String> {
    let path = settings_path(&app)?;
    let settings = Settings {
        ask_every_time,
        thread_count: thread_count.max(1),
        trim_none,
    };
    let text = serde_json::to_string_pretty(&settings).map_err(|e| e.to_string())?;
    fs::write(path, text).map_err(|e| e.to_string())
}

// Uses the callback-based pick_files/pick_folder API rather than
// blocking_pick_files/blocking_pick_folder: the blocking variants are
// documented as unsafe to call from the main thread (they can deadlock the
// event loop), and Tauri commands may run there. The oneshot channel just
// lets us await the callback's result from an async command.
#[tauri::command]
async fn pick_iso_files(app: AppHandle) -> Option<Vec<String>> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog()
        .file()
        .add_filter("ISO image", &["iso"])
        .set_title("Select one or more ISO files to convert")
        .pick_files(move |files| {
            let _ = tx.send(files);
        });
    let files = rx.await.ok().flatten()?;
    Some(
        files
            .into_iter()
            .filter_map(|f| f.into_path().ok())
            .map(|p| p.to_string_lossy().to_string())
            .collect(),
    )
}

#[tauri::command]
async fn pick_dest_folder(app: AppHandle) -> Option<String> {
    let (tx, rx) = tokio::sync::oneshot::channel();
    app.dialog()
        .file()
        .set_title("Choose a destination folder for the converted GOD files")
        .pick_folder(move |folder| {
            let _ = tx.send(folder);
        });
    let folder = rx.await.ok().flatten()?;
    folder
        .into_path()
        .ok()
        .map(|p| p.to_string_lossy().to_string())
}

#[derive(Serialize, Clone)]
struct ConvertProgress {
    index: u32,
    total: u32,
    name: String,
    status: String,
    message: Option<String>,
}

#[derive(Serialize)]
struct ConvertSummary {
    success_count: u32,
    total_count: u32,
    errors: Vec<String>,
}

#[tauri::command]
async fn convert(
    app: AppHandle,
    files: Vec<String>,
    dest: String,
    thread_count: u32,
    trim_none: bool,
) -> Result<ConvertSummary, String> {
    let total = files.len() as u32;
    let mut success_count = 0u32;
    let mut errors = Vec::new();
    let thread_count = thread_count.max(1);

    for (i, file) in files.iter().enumerate() {
        let index = i as u32 + 1;
        let name = PathBuf::from(file)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| file.clone());

        let _ = app.emit(
            "convert-progress",
            ConvertProgress {
                index,
                total,
                name: name.clone(),
                status: "running".into(),
                message: None,
            },
        );

        let mut args = vec!["-j".to_string(), thread_count.to_string()];
        if trim_none {
            args.push("--trim=none".to_string());
        }
        args.push(file.clone());
        args.push(dest.clone());

        let sidecar = app.shell().sidecar("iso2god").map_err(|e| e.to_string())?;
        let output = sidecar.args(args).output().await.map_err(|e| e.to_string())?;

        if output.status.success() {
            success_count += 1;
            let _ = app.emit(
                "convert-progress",
                ConvertProgress {
                    index,
                    total,
                    name: name.clone(),
                    status: "done".into(),
                    message: None,
                },
            );
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
            errors.push(format!("{}: {}", name, stderr));
            let _ = app.emit(
                "convert-progress",
                ConvertProgress {
                    index,
                    total,
                    name: name.clone(),
                    status: "error".into(),
                    message: Some(stderr),
                },
            );
        }
    }

    Ok(ConvertSummary {
        success_count,
        total_count: total,
        errors,
    })
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            save_settings,
            pick_iso_files,
            pick_dest_folder,
            convert
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
