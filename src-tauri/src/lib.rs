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
<<<<<<< HEAD
    app_handle.exit(0);
=======
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }

    println!("Ignored — waiting 30s before next scan...");
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;

    let flag = app_handle.state::<ScanReady>().0.clone();
    flag.store(true, Ordering::SeqCst);
}

fn show_popup(window: &tauri::WebviewWindow) {
    use tauri::LogicalSize;
    use tauri::LogicalPosition;

    let width: f64 = 300.0;
    let height: f64 = 160.0;

    let _ = window.set_min_size(Some(LogicalSize::new(width, height)));
    let _ = window.set_size(LogicalSize::new(width, height));

    if let Ok(Some(monitor)) = window.current_monitor() {
        let screen = monitor.size();
        let scale = monitor.scale_factor();
        let screen_w = screen.width as f64 / scale;
        let screen_h = screen.height as f64 / scale;
        let x = (screen_w - width) / 2.0;
        let y = (screen_h - height) / 2.0;
        let _ = window.set_position(LogicalPosition::new(x, y));
    }

    let _ = window.show();
    let _ = window.set_focus();
    let _ = window.set_size(LogicalSize::new(width, height));
>>>>>>> 8acb368 (final)
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