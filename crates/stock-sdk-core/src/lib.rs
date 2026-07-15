mod daily_factor;
mod project_files;
mod screening;

pub use daily_factor::DailyFactorSnapshot;
pub use project_files::{
    CreateStockScriptProjectFileRequest, RenameStockScriptProjectPathRequest,
    STOCK_SCRIPT_FILE_KIND_DIRECTORY, STOCK_SCRIPT_FILE_KIND_FILE, StockScriptProjectFileContent,
    StockScriptProjectFileNode, UpdateStockScriptProjectFileRequest,
};
pub use screening::{EffectivePriceSource, ScreeningContext, UniverseSnapshotRow};

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

pub const SDK_VERSION: &str = "v1";
pub const SDK_CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SCRIPT_KIND_SCREEN: &str = "screen";
pub const SCRIPT_KIND_NODE_PLUGIN: &str = "node_plugin";
pub const SOURCE_KIND_BLANK_LOCAL: &str = "blank_local";
pub const SOURCE_KIND_GIT_CLONE: &str = "git_clone";
pub const GIT_SYNC_STATUS_READY: &str = "ready";
pub const GIT_SYNC_STATUS_FAILED: &str = "sync_failed";
pub const VISIBILITY_KIND_PRIVATE: &str = "private";
pub const VISIBILITY_KIND_PUBLIC: &str = "public";
pub const VISIBILITY_KIND_SYSTEM: &str = "system";

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct CandidateRow {
    pub ts_code: String,
    pub name: Option<String>,
    pub score: Option<f64>,
    pub reason: Option<String>,
    #[serde(default)]
    pub fields: Value,
}

impl CandidateRow {
    pub fn new(ts_code: impl Into<String>) -> Self {
        Self {
            ts_code: ts_code.into(),
            name: None,
            score: None,
            reason: None,
            fields: Value::Object(Default::default()),
        }
    }

    pub fn with_name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn with_score(mut self, score: f64) -> Self {
        self.score = Some(score);
        self
    }

    pub fn with_reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub fn with_fields(mut self, fields: Value) -> Self {
        self.fields = fields;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct StageResult {
    pub label: String,
    pub input_count: usize,
    pub output_count: usize,
    #[serde(default)]
    pub sample_ts_codes: Vec<String>,
    pub elapsed_ms: u64,
}

impl StageResult {
    pub fn new(label: impl Into<String>, input_count: usize, output_count: usize) -> Self {
        Self {
            label: label.into(),
            input_count,
            output_count,
            sample_ts_codes: Vec::new(),
            elapsed_ms: 0,
        }
    }

    pub fn with_samples(mut self, sample_ts_codes: Vec<String>) -> Self {
        self.sample_ts_codes = sample_ts_codes;
        self
    }

    pub fn with_elapsed_ms(mut self, elapsed_ms: u64) -> Self {
        self.elapsed_ms = elapsed_ms;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct ScreeningReport {
    pub summary: String,
    #[serde(default)]
    pub metrics: Value,
    #[serde(default)]
    pub stages: Vec<StageResult>,
    #[serde(default)]
    pub candidates: Vec<CandidateRow>,
    #[serde(default)]
    pub notes: Vec<String>,
}

impl ScreeningReport {
    pub fn new(summary: impl Into<String>) -> Self {
        Self {
            summary: summary.into(),
            metrics: Value::Object(Default::default()),
            stages: Vec::new(),
            candidates: Vec::new(),
            notes: Vec::new(),
        }
    }

    pub fn with_metrics(mut self, metrics: Value) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn with_stage(mut self, stage: StageResult) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn with_candidates(mut self, candidates: Vec<CandidateRow>) -> Self {
        self.candidates = candidates;
        self
    }

    pub fn with_note(mut self, note: impl Into<String>) -> Self {
        self.notes.push(note.into());
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct CreateStockScriptProjectRequest {
    pub name: String,
    pub description: Option<String>,
    pub script_kind: Option<String>,
    /// Use `blank_local` for a generated scaffold, or `git_clone` for an HTTPS repository import.
    pub source_kind: String,
    pub git_remote_url: Option<String>,
    pub git_branch: Option<String>,
    pub git_token: Option<String>,
    pub visibility_kind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct UpdateStockScriptProjectRequest {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct SubmitStockScriptRunRequest {
    pub script_id: String,
    pub sdk_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptTaskAccepted {
    pub task_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptFuelUsage {
    pub fuel_limit: i64,
    pub fuel_used: i64,
    pub fuel_remaining: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptRunFailure {
    pub code: String,
    pub category: String,
    pub message: String,
    pub repairable: bool,
    pub retryable: bool,
    pub suggested_action: String,
    pub task_id: String,
    pub details: Option<Value>,
    pub trace_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptProjectSummary {
    pub script_id: String,
    pub name: String,
    pub description: String,
    pub script_kind: String,
    pub source_kind: String,
    pub visibility_kind: String,
    pub is_system: bool,
    pub can_edit: bool,
    pub can_execute: bool,
    pub git_remote_url: Option<String>,
    pub git_branch: Option<String>,
    pub git_head_commit: Option<String>,
    pub git_sync_status: String,
    pub git_sync_error: Option<String>,
    pub has_git_token: bool,
    pub workspace_available: bool,
    pub workspace_error: Option<String>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptProjectDetail {
    #[serde(flatten)]
    pub summary: StockScriptProjectSummary,
    pub git_dirty: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptWorkspaceEntry {
    pub path: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptWorkspaceFileResponse {
    pub path: String,
    pub content: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ApplyStockScriptWorkspacePatchRequest {
    pub patch: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ApplyStockScriptWorkspacePatchResponse {
    pub summary: String,
    #[schema(required)]
    pub diff: Option<String>,
    pub diff_truncated: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct WorkflowNodePort {
    pub key: String,
    pub label: String,
    pub value_kind: String,
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct WorkflowNodeField {
    pub key: String,
    pub label: String,
    pub field_kind: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub options: Vec<String>,
    pub default_value: Option<Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct WorkflowNodeManifest {
    pub node_type: String,
    pub display_name: String,
    pub description: String,
    pub category: String,
    pub script_id: Option<String>,
    pub script_kind: Option<String>,
    pub visibility_kind: String,
    pub is_system: bool,
    pub can_edit: bool,
    pub can_execute: bool,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub input_ports: Vec<WorkflowNodePort>,
    #[serde(default)]
    pub output_ports: Vec<WorkflowNodePort>,
    #[serde(default)]
    pub fields: Vec<WorkflowNodeField>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct StockScriptRunView {
    pub task_id: String,
    pub script_id: String,
    pub status: String,
    pub stderr: Option<String>,
    pub failure: Option<StockScriptRunFailure>,
    pub fuel: Option<StockScriptFuelUsage>,
    pub workspace_hash: String,
    pub git_head_commit: Option<String>,
    pub input: Value,
    pub report: Option<ScreeningReport>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub finished_at: Option<DateTime<Utc>>,
}
