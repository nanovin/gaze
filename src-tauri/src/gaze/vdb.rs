use arrow_array::types::Float32Type;
use arrow_array::{
    Array, FixedSizeListArray, Float32Array, Int32Array, Int64Array, RecordBatch,
    RecordBatchIterator, StringArray,
};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use lancedb::arrow::arrow_schema::{DataType, Field, Schema};
use lancedb::connect;
use lancedb::query::{ExecutableQuery, QueryBase};

use super::{
    state::{Gaze, GazeState},
    utils::create_id,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ScreenshotRow {
    id: Option<i32>,
    embedding: Vec<f32>,
    ocr_text: String,
    timestamp: Option<i64>,
    focused_window_title: String,
}

impl ScreenshotRow {
    pub fn from_record_batch(batch: RecordBatch) -> Vec<Self> {
        let ids = batch
            .column(0)
            .as_any()
            .downcast_ref::<Int32Array>()
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let embeddings: Vec<Vec<f32>> = batch
            .column(1)
            .as_any()
            .downcast_ref::<FixedSizeListArray>()
            .unwrap()
            .iter()
            .map(|list_opt| {
                list_opt
                    .map(|list| {
                        let float_array = list.as_any().downcast_ref::<Float32Array>().unwrap();
                        (0..list.len())
                            .map(|i| float_array.value(i as usize))
                            .collect::<Vec<f32>>()
                    })
                    .unwrap()
            })
            .collect();

        let ocr_texts = batch
            .column(2)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let timestamps = batch
            .column(3)
            .as_any()
            .downcast_ref::<Int64Array>()
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        let focused_window_titles = batch
            .column(4)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap()
            .into_iter()
            .map(|x| x.unwrap())
            .collect::<Vec<_>>();

        ids.into_iter()
            .zip(embeddings.into_iter())
            .zip(ocr_texts.into_iter())
            .zip(timestamps.into_iter())
            .zip(focused_window_titles.into_iter())
            .map(
                |((((id, embedding), ocr_text), timestamp), focused_window_title)| Self {
                    id: Some(id),
                    embedding,
                    ocr_text: ocr_text.to_string(),
                    timestamp: Some(timestamp),
                    focused_window_title: focused_window_title.to_string(),
                },
            )
            .collect::<Vec<_>>()
    }

    pub fn _to_record_batch(&self, schema: Arc<Schema>) -> RecordBatch {
        RecordBatch::try_new(
            schema,
            vec![
                Arc::new(Int32Array::from(vec![self.id])),
                Arc::new(
                    FixedSizeListArray::from_iter_primitive::<Float32Type, _, _>(
                        vec![Some(
                            self.embedding
                                .clone()
                                .into_iter()
                                .map(|x| Some(x))
                                .collect::<Vec<_>>(),
                        )],
                        384,
                    ),
                ),
                Arc::new(StringArray::from(vec![Some(self.ocr_text.clone())])),
                Arc::new(Int64Array::from(vec![self.timestamp])),
                Arc::new(StringArray::from(vec![Some(
                    self.focused_window_title.clone(),
                )])),
            ],
        )
        .unwrap()
    }
}

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

    pub async fn search_embeddings(
        &self,
        embedded_query: Vec<f32>,
        k: usize,
    ) -> Result<Vec<ScreenshotRow>, Box<dyn std::error::Error>> {
        let tbl = self.tbl.as_ref().unwrap();

        let record = tbl
            .query()
            .limit(k)
            .nearest_to(embedded_query)?
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        Ok(record
            .into_iter()
            .map(|x| ScreenshotRow::from_record_batch(x))
            .flatten()
            .collect::<Vec<_>>())
    }

    pub async fn get_rows(
        &self,
        after: Option<i64>,
        before: Option<i64>,
        limit: usize,
    ) -> Result<Vec<ScreenshotRow>, Box<dyn std::error::Error>> {
        let tbl = self.tbl.as_ref().unwrap();

        let records = tbl
            .query()
            .only_if(if let Some(after) = after {
                format!("timestamp >= {}", after)
            } else {
                "timestamp >= 0".to_string()
            })
            .only_if(if let Some(before) = before {
                format!("timestamp <= {}", before)
            } else {
                "timestamp >= 0".to_string()
            })
            .limit(limit)
            .execute()
            .await?
            .try_collect::<Vec<_>>()
            .await?;

        Ok(records
            .into_iter()
            .map(|x| ScreenshotRow::from_record_batch(x))
            .flatten()
            .collect::<Vec<_>>())
    }
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
