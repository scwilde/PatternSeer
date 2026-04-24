use std::{path::Path, str::FromStr};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;
use crate::app::forms::FormDraft;

#[derive(Debug, Clone)]
pub struct PatternDraft {
    pub width: i16,
    pub height: i16,
    pub path: Option<String>
}
impl PatternDraft {
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 30,
            path: None,
        }
    }
}
impl FormDraft for PatternDraft {
    type Complete = Pattern;
    type Error = PatternError;

    fn finish(&self) -> Result<Self::Complete, Self::Error>{
        Pattern::from_draft(self.clone()).map_err(|e| {
            PatternError::SQLError(e)
        })
    }
}
#[derive(Debug)]
pub enum PatternError {
    SQLError(sqlx::Error)
}

#[derive(Debug)]
pub struct PatternMeta {
    pub width: i16,
    pub height: i16,
}

#[derive(Debug)]
pub struct Pattern {
    pub metadata: PatternMeta,
    pub db_pool: SqlitePool,
}
impl Pattern {
    pub fn from_draft(draft: PatternDraft) -> sqlx::Result<Self> {
        let path = draft.path.as_ref().unwrap().as_str();
        let path_obj = Path::new(path);
        if path_obj.exists() {
            match std::fs::remove_file(path_obj) {
                Ok(_) => {},
                Err(_e) => { todo!() }
            }
        }

        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let db_options = SqliteConnectOptions::from_str(path)?
                .create_if_missing(true);
            let db_pool = SqlitePool::connect_with(db_options).await?;

            sqlx::query(
                r#"
                CREATE TABLE metadata (
                    id INTEGER PRIMARY KEY CHECK (id = 0),
                    width INTEGER NOT NULL CHECK (0 < width AND width <= 16384),
                    height INTEGER NOT NULL CHECK (0 < height AND height <= 16384)
                )
                "#
            )
            .execute(&db_pool)
            .await?;

            sqlx::query!(
                "INSERT INTO metadata (id, width, height) VALUES (0, ?, ?)",
                draft.width,
                draft.height,
            )
            .execute(&db_pool)
            .await?;

            Ok(Self {
                metadata: PatternMeta { width: draft.width, height: draft.height },
                db_pool,
            })
        })
    }

    pub fn open_sync(path: &str) -> sqlx::Result<Self> {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let db_pool = SqlitePool::connect(path).await?;

            let metadata = sqlx::query_as!(
                PatternMeta,
                // * `AS i16` is safe here as `width` and `height` are CHECKed to be {0 < x <= 16,384}
                "SELECT width AS 'width: i16', height AS 'height: i16' FROM metadata",
            )
            .fetch_one(&db_pool)
            .await?;

            Ok(Self {metadata, db_pool})
        })
    }
}
