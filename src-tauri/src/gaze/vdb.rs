use std::sync::Arc;

use arrow_array::types::Float32Type;
use arrow_array::{
    FixedSizeListArray, Int32Array, Int64Array, RecordBatch, RecordBatchIterator, StringArray,
};

use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::connect;

use super::{
    state::{Gaze, GazeState},
    utils::create_id,
};

pub fn create_record(
    schema: Arc<Schema>,
    id: i32,
    embeddings: Vec<f32>,
    ocr_text: &str,
    focused_window_title: &str,
) -> RecordBatch {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    RecordBatch::try_new(
        schema,
        vec![
            Arc::new(Int32Array::from(vec![id])),
            Arc::new(
                FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                    vec![Some(
                        embeddings.into_iter().map(|x| Some(x)).collect::<Vec<_>>(),
                    )],
                    384,
                ),
            ),
            Arc::new(StringArray::from(vec![Some(ocr_text.to_string())])),
            Arc::new(Int64Array::from(vec![timestamp])),
            Arc::new(StringArray::from(vec![Some(
                focused_window_title.to_string(),
            )])),
        ],
    )
    .unwrap()
}

impl Gaze {
    pub async fn store_embeddings(
        &self,
        embeddings: Vec<f32>,
        ocr_text: &str,
        focused_window_title: &str,
    ) -> Result<i32, lancedb::Error> {
        let tbl = self.tbl.as_ref().unwrap();
        let schema = self.schema.as_ref().unwrap();
        let id = create_id();

        let batch = create_record(
            schema.clone(),
            id,
            embeddings,
            ocr_text,
            focused_window_title,
        );

        tbl.add(Box::new(RecordBatchIterator::new(
            vec![batch].into_iter().map(Ok),
            schema.clone(),
        )))
        .execute()
        .await?;

        Ok(id)
    }

    // pub async fn search_embeddings(
    //     &self,
    //     query: &str,
    //     k: i64,
    // ) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    //     Ok(vec![])
    // }
}

pub async fn init_vdb(app_state: GazeState) {
    let db = connect("../data/gazedb").execute().await.unwrap();

    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, true),
        Field::new(
            "embedding",
            DataType::FixedSizeList(Arc::new(Field::new("item", DataType::Float32, true)), 384),
            true,
        ),
        Field::new("ocr_text", DataType::Utf8, true),
        Field::new("timestamp", DataType::Int64, true),
        Field::new("focused_window_title", DataType::Utf8, true),
    ]));

    let tbl = match db.open_table("screenshots").execute().await {
        Ok(tbl) => tbl,
        Err(_) => db
            .create_empty_table("screenshots", schema.clone())
            .execute()
            .await
            .unwrap(),
    };

    let mut gaze = app_state.lock().await;

    gaze.schema = Some(schema);
    gaze.tbl = Some(tbl);
    gaze.vdb = Some(db);
    println!("loaded vdb, ready to search uwu~~~");
}
