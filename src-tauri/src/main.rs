// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::WebviewWindowBuilder;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app|{
            let mut window_builder = WebviewWindowBuilder::new(app, "proxy", tauri::WebviewUrl::External("https://duckduckgo.com".parse().unwrap()));
            #[cfg(desktop)]
            {
                window_builder = window_builder
                .title("Proxy Example")
                .inner_size(1000., 800.)
                .min_inner_size(600., 400.)
                .content_protected(true)
                .proxy_url("socks5://localhost:1337".parse().unwrap());
            }

            let window = window_builder.build().unwrap();
            window.show().unwrap();
          Ok(())  
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
