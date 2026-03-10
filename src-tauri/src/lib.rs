use tauri::{Manager, Emitter};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

mod bluetooth;

struct ScanReady(Arc<AtomicBool>);

#[tauri::command]
async fn connect_airpods(app_handle: tauri::AppHandle) -> Result<String, String> {
    let result = bluetooth::connect_airpods()
        .await
        .map_err(|e: Box<dyn std::error::Error + Send + Sync>| e.to_string())?;

    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }

    let flag = app_handle.state::<ScanReady>().0.clone();
    flag.store(true, Ordering::SeqCst);

    Ok(result)
}

#[tauri::command]
async fn ignore_airpods(app_handle: tauri::AppHandle) {
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let scan_ready = Arc::new(AtomicBool::new(true));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(ScanReady(scan_ready.clone()))
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let ready = scan_ready.clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    while !ready.load(Ordering::SeqCst) {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    }

                    match bluetooth::wait_for_airpods(0).await {
                        Ok(name) => {
                            println!("Found AirPods: {} — showing popup", name);

                            ready.store(false, Ordering::SeqCst);

                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.emit("airpods-found", &name);
                                show_popup(&window);
                            }
                        }
                        Err(e) => {
                            eprintln!("Scan error: {}, retrying in 10s...", e);
                            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
                        }
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