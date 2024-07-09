use super::{embed::async_embed_text, state::GazeState, vdb::ScreenshotRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearcResults(Vec<ScreenshotRow>);

// search vector DB for similar strings to the query and return 10 most similar
#[tauri::command]
pub async fn vector_search(
    state: tauri::State<'_, GazeState>,
    query: String,
) -> Result<SearcResults, String> {
    let state = state.lock().await;
    let embedded_query = async_embed_text(query).await.unwrap();
    let results = state
        .search_embeddings(embedded_query, 10)
        .await
        .map_err(|e| e.to_string())?;
    Ok(SearcResults(results))
}

// search vector DB for rows between the given timestamps
#[tauri::command]
pub async fn get_rows(
    state: tauri::State<'_, GazeState>,
    after: Option<i64>,
    before: Option<i64>,
    limit: usize,
) -> Result<SearcResults, String> {
    let state = state.lock().await;
    let results = state
        .get_rows(after, before, limit)
        .await
        .map_err(|e| e.to_string())?;
    Ok(SearcResults(results))
}
