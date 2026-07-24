use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use utoipa::ToSchema;

pub const SCRIPT_COMPUTE_PROTOCOL_VERSION: &str = "compute-v1";
pub const SCRIPT_ARTIFACT_KIND_JSON: &str = "json";
pub const SCRIPT_ARTIFACT_KIND_CSV: &str = "csv";
pub const SCRIPT_ARTIFACT_KIND_MARKDOWN: &str = "markdown";
pub const SCRIPT_ARTIFACT_KIND_CHART: &str = "chart";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ScriptDataKind {
    QuoteSnapshot,
    PriceSeries,
    FactorSnapshot,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScriptDataRequest {
    pub kind: ScriptDataKind,
    pub ts_codes: Vec<String>,
    #[serde(default)]
    pub from: Option<NaiveDate>,
    #[serde(default)]
    pub to: Option<NaiveDate>,
    #[serde(default)]
    pub period: Option<String>,
    #[serde(default)]
    pub adjustment: Option<String>,
    #[serde(default)]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScriptQuoteSnapshot {
    pub ts_code: String,
    pub name: Option<String>,
    pub latest_price: Option<f64>,
    pub change_amount: Option<f64>,
    pub pct_change: Option<f64>,
    pub open: Option<f64>,
    pub high: Option<f64>,
    pub low: Option<f64>,
    pub pre_close: Option<f64>,
    pub volume: Option<f64>,
    pub amount: Option<f64>,
    pub turnover_rate: Option<f64>,
    pub volume_ratio: Option<f64>,
    pub average_price: Option<f64>,
    pub total_market_value: Option<f64>,
    pub circulating_market_value: Option<f64>,
    pub fetched_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScriptPricePoint {
    pub trade_date: NaiveDate,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub pre_close: Option<f64>,
    pub volume: f64,
    pub amount: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScriptFactorSnapshot {
    pub ts_code: String,
    pub trade_date: NaiveDate,
    pub industry: Option<String>,
    pub adj_close: Option<f64>,
    pub close_raw: Option<f64>,
    pub ret_1d: Option<f64>,
    pub ret_5d: Option<f64>,
    pub ret_20d: Option<f64>,
    pub volatility_20d: Option<f64>,
    pub rsi14: Option<f64>,
    pub ma20: Option<f64>,
    pub pe_ttm: Option<f64>,
    pub pb: Option<f64>,
    pub roe: Option<f64>,
    pub risk_warning: bool,
    pub data_gaps_detected: bool,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ScriptDataSet {
    QuoteSnapshots {
        fetched_at: DateTime<Utc>,
        items: Vec<ScriptQuoteSnapshot>,
    },
    PriceSeries {
        period: String,
        adjustment: String,
        fetched_at: DateTime<Utc>,
        series: Vec<ScriptPriceSeries>,
    },
    FactorSnapshots {
        fetched_at: DateTime<Utc>,
        items: Vec<ScriptFactorSnapshot>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema)]
pub struct ScriptPriceSeries {
    pub ts_code: String,
    pub items: Vec<ScriptPricePoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct ScriptInputFile {
    pub path: String,
    pub format: String,
    pub content: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct ScriptComputeInput {
    pub protocol_version: String,
    #[serde(default)]
    pub files: Vec<ScriptInputFile>,
    #[serde(default)]
    pub parameters: Value,
    #[serde(default)]
    pub data_requests: Vec<ScriptDataRequest>,
    #[serde(default)]
    pub datasets: Vec<ScriptDataSet>,
}

impl ScriptComputeInput {
    pub fn new(files: Vec<ScriptInputFile>) -> Self {
        Self {
            protocol_version: SCRIPT_COMPUTE_PROTOCOL_VERSION.to_string(),
            files,
            parameters: Value::Object(Default::default()),
            data_requests: Vec::new(),
            datasets: Vec::new(),
        }
    }

    pub fn with_parameters(mut self, parameters: Value) -> Self {
        self.parameters = parameters;
        self
    }

    pub fn with_data_requests(mut self, data_requests: Vec<ScriptDataRequest>) -> Self {
        self.data_requests = data_requests;
        self
    }

    pub fn with_datasets(mut self, datasets: Vec<ScriptDataSet>) -> Self {
        self.datasets = datasets;
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct ScriptArtifact {
    pub path: String,
    pub kind: String,
    pub format: String,
    pub content: String,
    pub size: u64,
    pub sha256: String,
}

impl ScriptArtifact {
    pub fn new(
        path: impl Into<String>,
        kind: impl Into<String>,
        format: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            path: path.into(),
            kind: kind.into(),
            format: format.into(),
            content: content.into(),
            size: 0,
            sha256: String::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema, ToSchema, Default)]
pub struct ScriptComputeResult {
    pub summary: Option<String>,
    #[serde(default)]
    pub result: Value,
    #[serde(default)]
    pub metrics: Value,
    #[serde(default)]
    pub artifacts: Vec<ScriptArtifact>,
    #[serde(default)]
    pub charts: Vec<Value>,
}

impl ScriptComputeResult {
    pub fn new(result: Value) -> Self {
        Self {
            summary: None,
            result,
            metrics: Value::Object(Default::default()),
            artifacts: Vec::new(),
            charts: Vec::new(),
        }
    }

    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    pub fn with_metrics(mut self, metrics: Value) -> Self {
        self.metrics = metrics;
        self
    }

    pub fn with_artifact(mut self, artifact: ScriptArtifact) -> Self {
        self.artifacts.push(artifact);
        self
    }

    pub fn with_chart(mut self, chart: Value) -> Self {
        self.charts.push(chart);
        self
    }
}
