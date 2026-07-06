use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub const STOCK_SCRIPT_FILE_KIND_FILE: &str = "file";
pub const STOCK_SCRIPT_FILE_KIND_DIRECTORY: &str = "directory";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptProjectFileNode {
    pub path: String,
    pub name: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptProjectFileContent {
    pub path: String,
    pub name: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct CreateStockScriptProjectFileRequest {
    pub path: String,
    pub kind: String,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct UpdateStockScriptProjectFileRequest {
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct RenameStockScriptProjectPathRequest {
    pub path: String,
    pub new_path: String,
}
