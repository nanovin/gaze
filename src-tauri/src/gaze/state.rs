use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;

#[derive(Default)]
pub struct Gaze {
    pub vdb: Option<lancedb::Connection>,
    pub schema: Option<Arc<arrow::datatypes::Schema>>,
    pub tbl: Option<lancedb::Table>,
    pub last_screenshot_phash: Option<Vec<u8>>,
    pub app_data_dir: Option<PathBuf>,
}

pub type GazeState = Arc<Mutex<Gaze>>;

impl Gaze {
    pub fn set_app_data_dir(state: GazeState, path: PathBuf) {
        tokio::spawn(async move {
            let mut locked_internal = state.lock().await;
            locked_internal.app_data_dir = Some(path);
        });
    }
}
