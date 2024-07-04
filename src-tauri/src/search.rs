use device_query::{DeviceQuery, DeviceState};
use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use tauri::Manager;
use window_vibrancy::*;

#[derive(Default)]
struct MyState {
    s: std::sync::Mutex<String>,
    t: std::sync::Mutex<std::collections::HashMap<String, String>>,
}
// remember to call `.manage(MyState::default())`
#[tauri::command]
async fn command_name(state: tauri::State<'_, MyState>) -> Result<(), String> {
    *state.s.lock().unwrap() = "new string".into();
    state.t.lock().unwrap().insert("key".into(), "value".into());
    Ok(())
}

pub fn init_search() {
    let setup = tauri::Builder::default().setup(move |app| {
        let window = app.get_window("main").unwrap();
        window.set_title("Search").unwrap();
        window.hide().unwrap();

        // if its unfocused, hide the window
        let window_copy = window.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Focused(false) = event {
                window_copy.hide().unwrap();
            }
        });

        #[cfg(target_os = "macos")]
        apply_vibrancy(&window, NSVisualEffectMaterial::HudWindow, None, None)
            .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");

        #[cfg(target_os = "windows")]
        apply_acrylic(&window, Some((18, 18, 18, 125)))
            .expect("Unsupported platform! 'apply_blur' is only supported on Windows");

        let app_handle = Arc::new(Mutex::new(app.app_handle().clone()));
        let app_handle_thread = Arc::clone(&app_handle);
        let _keypress_thread = std::thread::spawn(move || {
            let device_state = DeviceState::new();
            let mut triggered = false;
            loop {
                // check for keybind
                let keys = device_state.get_keys();
                if keys.contains(&device_query::Keycode::LMeta)
                    && keys.contains(&device_query::Keycode::Space)
                {
                    if !triggered {
                        let app_handle = app_handle_thread.lock().unwrap();
                        // set tauri window to be visible and focused or hidden
                        let window = app_handle.get_window("main").unwrap();
                        if window.is_visible().unwrap_or(false) {
                            window.hide().unwrap();
                        } else {
                            window.show().unwrap();
                            window.set_focus().unwrap();
                        }
                    }

                    triggered = true;
                } else {
                    triggered = false;
                }

                thread::sleep(Duration::from_micros(100));
            }
        });

        Ok(())
    });

    setup
        .manage(MyState::default())
        .invoke_handler(tauri::generate_handler![command_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
