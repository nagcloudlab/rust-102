// models/mod.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferRequest {
    pub from_account: String,
    pub to_account: String,
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    pub success: bool,
    pub message: String,
}
