use serde::{Deserialize, Serialize};

/// Main report structure returned by API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Report {
    pub id: String,
    pub tx_hash: String,
    pub chain: String,
    pub status: TxStatus,
    pub root_cause: String,
    pub failing_frame: Option<FailingFrame>,
    pub backtrace: Vec<BacktraceItem>,
    pub call_tree: Vec<CallNode>,
    pub stylus_trace: Option<StylusTrace>,
    pub suggestions: Vec<Suggestion>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TxStatus {
    Success,
    Reverted,
    OutOfGas,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailingFrame {
    pub contract: String,
    pub function: String,
    pub error: String,
    pub source_line: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacktraceItem {
    pub from: String,
    pub to: String,
    pub function: String,
    pub depth: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub call_type: String, // CALL, DELEGATECALL, STATICCALL, CREATE
    pub from: String,
    pub to: String,
    pub value: String,
    pub gas_used: u64,
    pub input: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub function_name: Option<String>,
    pub decoded_input: Option<serde_json::Value>,
    pub calls: Vec<CallNode>, // Nested calls
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StylusTrace {
    pub ink_used: u64,
    pub function_calls: Vec<String>,
    pub replay_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Suggestion {
    pub issue: String,
    pub fix: String,
    pub priority: SuggestionPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SuggestionPriority {
    High,
    Medium,
    Low,
}

/// API request/response types
#[derive(Debug, Deserialize)]
pub struct AnalyzeRequest {
    pub name: String,
    pub chain: String,
    pub tx_hash: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyzeResponse {
    pub report: Report,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub details: Option<String>,
}