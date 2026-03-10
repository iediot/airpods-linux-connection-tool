use tauri::{Manager, LogicalSize};

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())

        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            window.set_size(LogicalSize::new(300.0, 160.0))?;
            window.set_resizable(false)?;
            window.set_decorations(false)?;
            window.set_always_on_top(true)?;

            Ok(())
        })

        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}