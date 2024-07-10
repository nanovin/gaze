use tauri::Window;
use window_vibrancy::*;

#[cfg(target_os = "macos")]
pub fn apply_transparency(window: &Window) {
    apply_vibrancy(window, NSVisualEffectMaterial::HudWindow, None, None)
        .expect("Unsupported platform! 'apply_vibrancy' is only supported on macOS");
}

#[cfg(target_os = "windows")]
pub fn apply_transparency(window: &Window) {
    apply_mica(window, Some(true))
        .expect("Unsupported platform! 'apply_blur' is only supported on Windows");
}

pub fn setup_window(window: &Window) {
    window.set_title("Search").unwrap();
    window.hide().unwrap();
    // apply_transparency(window);

    // if its unfocused, hide the window
    // let window_copy = window.clone();
    // window.on_window_event(move |event| {
    //     if let tauri::WindowEvent::Focused(false) = event {
    //         window_copy.hide().unwrap();
    //     }
    // });
}
