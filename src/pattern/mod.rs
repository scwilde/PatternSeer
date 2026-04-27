use std::{fmt, io, path::{Path, PathBuf}, str::FromStr};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use tokio::runtime::Runtime;


pub mod pattern_file;


#[derive(Debug, Clone)]
pub struct PatternDraft {
    pub width: u16,
    pub height: u16,
}
impl PatternDraft {
    pub fn new() -> Self {
        Self {
            width: 30,
            height: 30,
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
pub struct Pattern {
    pub width: u16,
    pub height: u16,
    pub path: Option<PathBuf>,
}
impl Pattern {
    pub fn from_draft(draft: &PatternDraft) -> Self {
        Self {
            width: draft.width,
            height: draft.height,
            path: None,
        }
    }

    // pub fn open_sync(path: &str) -> Result<Self, PatternError> {
    //     let rt = Runtime::new().expect("Failed to create Tokio runtime");
    //     rt.block_on(async {
    //         let db_pool = SqlitePool::connect(path).await
    //             .map_err(|e| PatternError::SQLError(e))?;

    //         let metadata = sqlx::query_as!(
    //             PatternMeta,
    //             // * `AS i16` is safe here as `width` and `height` are CHECKed to be {0 < x <= 16,384}
    //             "SELECT width AS 'width: u16', height AS 'height: u16' FROM metadata",
    //         )
    //         .fetch_one(&db_pool)
    //         .await
    //         .map_err(|e| PatternError::SQLError(e))?;

    //         Ok(Self {
    //             width: metadata.width,
    //             height: metadata.height,
    //         })
    //     })
    // }
}
