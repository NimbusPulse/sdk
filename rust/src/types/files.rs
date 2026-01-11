use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct FileInfo {
    pub name: String,
    pub path: String,
    pub is_directory: bool,
    pub size: Option<u64>,
    pub modified: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct FileListResponse {
    pub files: Vec<FileInfo>,
    pub current_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct FileUploadRequest {
    pub path: String,
    pub content: Vec<u8>,
    pub overwrite: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[cfg_attr(test, derive(ts_rs::TS))]
#[cfg_attr(test, ts(export, export_to = "../../javascript/lib/types/"))]
pub struct FileDownloadResponse {
    pub content: Vec<u8>,
    pub filename: String,
}
