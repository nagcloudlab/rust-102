use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Account {
    number: String,
    balance: f32,
}

#[wasm_bindgen]
impl Account {
    #[wasm_bindgen(constructor)]
    pub fn new(number: &str, balance: f32) -> Account {
        Account {
            number: number.to_string(),
            balance,
        }
    }

    pub fn validate_transfer(&self, amount: f32) -> Result<(), String> {
        if amount <= 0.0 {
            Err("Transfer amount must be positive.".to_string())
        } else if self.balance < amount {
            Err("Insufficient balance.".to_string())
        } else {
            Ok(())
        }
    }
}

//validation

// calc
