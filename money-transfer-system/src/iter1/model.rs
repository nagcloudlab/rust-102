// models/mod.rs
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use validator_derive::Validate; // Import the derive macro

#[derive(Debug)]
pub struct Account {
    pub number: String,
    pub balance: f32,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct TransferRequest {
    pub from_account: String,
    pub to_account: String,
    #[validate(range(min = 0.01, message = "Amount must be greater than 0"))]
    pub amount: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransferResponse {
    pub success: bool,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionHistory {
    pub from_account: String, // Changed to String
    pub to_account: String,   // Changed to String
    pub amount: f32,
    pub timestamp: NaiveDateTime,
}
