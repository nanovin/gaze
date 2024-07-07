use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct Gaze {
    pub vdb: Option<lancedb::Connection>,
    pub schema: Option<Arc<arrow::datatypes::Schema>>,
    pub tbl: Option<lancedb::Table>,
}

pub type GazeState = Arc<Mutex<Gaze>>;

unsafe impl Send for Gaze {}
unsafe impl Sync for Gaze {}
