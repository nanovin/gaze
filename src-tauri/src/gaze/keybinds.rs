use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use device_query::{DeviceQuery, DeviceState};
use tauri::Manager;

pub fn setup_keybinds(app: &mut tauri::App) {
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
}
