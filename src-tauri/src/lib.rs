use tauri::Manager;

mod bluetooth;

#[tauri::command]
async fn connect_airpods(app_handle: tauri::AppHandle) -> Result<String, String> {
    let result = bluetooth::connect_airpods()
        .await
        .map_err(|e: Box<dyn std::error::Error + Send + Sync>| e.to_string())?;

    app_handle.exit(0);
    Ok(result)
}

#[tauri::command]
async fn ignore_airpods(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                match bluetooth::wait_for_airpods().await {
                    Ok(name) => {
                        println!("Found AirPods: {}", name);

                        if let Some(window) = app_handle.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    Err(e) => {
                        eprintln!("Bluetooth scan error: {}", e);
                        app_handle.exit(1);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            connect_airpods,
            ignore_airpods,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}