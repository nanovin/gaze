use std::sync::Arc;
use tokio::sync::Mutex;

use keybinds::setup_keybinds;
use screenshot::init_screenshot_worker;
use state::{Gaze, GazeState};
use tauri::Manager;
use vdb::init_vdb;
use window_appearance::setup_window;

pub mod embed;
pub mod imghash;
pub mod keybinds;
pub mod ocr;
pub mod screenshot;
pub mod search;
pub mod state;
pub mod utils;
pub mod vdb;
pub mod window_appearance;

pub async fn init_gaze() {
    let app_state: GazeState = Arc::new(Mutex::new(Gaze::default()));

    init_screenshot_worker(app_state.clone());

    let app_state_ref = app_state.clone();
    tauri::Builder::default()
        .setup(move |app| {
            let window = app.get_window("main").unwrap();
            setup_window(&window);
            setup_keybinds(app);

            match app.path_resolver().app_data_dir() {
                Some(path) => {
                    Gaze::set_app_data_dir(app_state_ref.clone(), path);
                }
                None => {
                    println!("Could not get app data dir");
                }
            };

            // this might be a race condition im not sure... though the lock should save it?
            // theres 100% a better way to be doing this i just want it to work for right now
            tokio::spawn(init_vdb(app_state_ref));

            Ok(())
        })
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            search::vector_search,
            search::get_rows
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
