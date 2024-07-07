use std::sync::Arc;
use tokio::sync::Mutex;

use keybinds::setup_keybinds;
use screenshot::init_screenshot_worker;
use state::{Gaze, GazeState};
use tauri::Manager;
use vdb::init_vdb;
use window_appearance::setup_window;

pub mod embed;
pub mod keybinds;
pub mod ocr;
pub mod screenshot;
pub mod state;
pub mod utils;
pub mod vdb;
pub mod window_appearance;

// #[tauri::command]
// async fn command_name(state: tauri::State<'_, GazeAppState>) -> Result<(), String> {
//     Ok(())
// }

pub async fn init_search() {
    let app_state: GazeState = Arc::new(Mutex::new(Gaze::default()));

    init_vdb(app_state.clone()).await;
    init_screenshot_worker(app_state.clone());

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_window("main").unwrap();
            setup_window(&window);
            setup_keybinds(app);
            Ok(())
        })
        .manage(app_state)
        // .invoke_handler(tauri::generate_handler![command_name])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}