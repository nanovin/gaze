use fastembed::{EmbeddingModel, InitOptions, TextEmbedding};

pub fn embed_text(text: &str) -> Result<Vec<f32>, Box<dyn std::error::Error>> {
    // With custom InitOptions
    let model = TextEmbedding::try_new(InitOptions {
        model_name: EmbeddingModel::AllMiniLML6V2, // 384-dimensional embeddings
        show_download_progress: true,
        ..Default::default()
    })?;
    match model.embed(Vec::from([text]), None) {
        Ok(embeddings) => Ok(embeddings[0].clone()),
        Err(e) => Err(e.into()),
    }
}
