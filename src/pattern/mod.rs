use std::{fmt, io, path::{Path, PathBuf}, str::FromStr};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;


// mod pattern_file;


#[derive(Debug, Clone)]
pub struct PatternDraft {
    pub width: u16,
    pub height: u16,
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

#[derive(Debug)]
pub enum PatternError {
    SQLError(sqlx::Error),
    IOError(io::Error),
}
impl fmt::Display for PatternError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SQLError(e) => write!(f, "SQL Error: {}", e),
            Self::IOError(e) => write!(f, "File IO Error: {}", e),
        }
    }
}


#[derive(Debug)]
pub struct PatternMeta {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug)]
pub struct Pattern {
    pub metadata: PatternMeta,
    pub db_pool: SqlitePool,
}
impl Pattern {
    pub fn from_draft(draft: &PatternDraft) -> Result<Self, PatternError> {
        let path = draft.path.as_ref().unwrap().as_str();
        let path_obj = Path::new(path);
        if path_obj.exists() {
            std::fs::remove_file(path_obj).map_err(|e| PatternError::IOError(e))?
        }

        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let db_options = SqliteConnectOptions::from_str(path)
                .map_err(|e| PatternError::SQLError(e))?
                .create_if_missing(true);
            let db_pool = SqlitePool::connect_with(db_options).await
                .map_err(|e| PatternError::SQLError(e))?;

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
            .await
            .map_err(|e| PatternError::SQLError(e))?;

            sqlx::query!(
                "INSERT INTO metadata (id, width, height) VALUES (0, ?, ?)",
                draft.width,
                draft.height,
            )
            .execute(&db_pool)
            .await
            .map_err(|e| PatternError::SQLError(e))?;

            Ok(Self {
                metadata: PatternMeta { width: draft.width, height: draft.height },
                db_pool,
            })
        })
    }

    pub fn open_sync(path: &str) -> Result<Self, PatternError> {
        let rt = Runtime::new().expect("Failed to create Tokio runtime");
        rt.block_on(async {
            let db_pool = SqlitePool::connect(path).await
                .map_err(|e| PatternError::SQLError(e))?;

            let metadata = sqlx::query_as!(
                PatternMeta,
                // * `AS i16` is safe here as `width` and `height` are CHECKed to be {0 < x <= 16,384}
                "SELECT width AS 'width: u16', height AS 'height: u16' FROM metadata",
            )
            .fetch_one(&db_pool)
            .await
            .map_err(|e| PatternError::SQLError(e))?;

            Ok(Self {metadata, db_pool})
        })
    }
}
