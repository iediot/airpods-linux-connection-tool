use tauri::{Manager, Emitter};
use std::sync::Arc;
use tokio::sync::Mutex;

mod bluetooth;

#[derive(Debug, PartialEq)]
enum ScanState {
    Scanning,
    Ready,
    Cooldown(u64),
}

struct AppState(Arc<Mutex<ScanState>>);

#[tauri::command]
async fn connect_airpods(app_handle: tauri::AppHandle) -> Result<String, String> {
    let result = bluetooth::connect_airpods()
        .await
        .map_err(|e: Box<dyn std::error::Error + Send + Sync>| e.to_string());

    if result.is_ok() {
        let app_state = app_handle.state::<AppState>();
        let mut state = app_state.0.lock().await;
        *state = ScanState::Cooldown(60);
        drop(state);
        if let Some(window) = app_handle.get_webview_window("main") {
            let _ = window.hide();
        }
    }

    result
}

#[tauri::command]
async fn ignore_airpods(app_handle: tauri::AppHandle) {
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.hide();
    }

    println!("Ignored — will wait 30s before next scan.");
    let app_state = app_handle.state::<AppState>();
    let mut state = app_state.0.lock().await;
    *state = ScanState::Cooldown(30);
}

fn show_popup(window: &tauri::WebviewWindow) {
    use tauri::LogicalSize;
    use tauri::LogicalPosition;

    let width: f64 = 300.0;
    let height: f64 = 200.0;

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
    let app_state = Arc::new(Mutex::new(ScanState::Ready));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState(app_state.clone()))
        .setup(move |app| {
            let app_handle = app.handle().clone();
            let state = app_state.clone();

            tauri::async_runtime::spawn(async move {
                loop {
                    let current = {
                        let s = state.lock().await;
                        match *s {
                            ScanState::Ready => ScanState::Ready,
                            ScanState::Scanning => ScanState::Scanning,
                            ScanState::Cooldown(secs) => ScanState::Cooldown(secs),
                        }
                    };

                    match current {
                        ScanState::Cooldown(secs) => {
                            tokio::time::sleep(std::time::Duration::from_secs(secs)).await;
                            let mut s = state.lock().await;
                            *s = ScanState::Ready;
                            continue;
                        }
                        ScanState::Scanning => {
                            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                            continue;
                        }
                        ScanState::Ready => {}
                    }

                    {
                        let mut s = state.lock().await;
                        *s = ScanState::Scanning;
                    }

                    match bluetooth::wait_for_airpods().await {
                        Ok(name) => {
                            println!("Found AirPods: {} — showing popup", name);
                            if let Some(window) = app_handle.get_webview_window("main") {
                                let _ = window.emit("airpods-found", &name);
                                show_popup(&window);
                            }
                        }
                        Err(e) => {
                            eprintln!("Scan error: {}, retrying in 10s...", e);
                            let mut s = state.lock().await;
                            *s = ScanState::Ready;
                            drop(s);
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
