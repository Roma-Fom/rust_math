use wasm_bindgen::prelude::*;
use rust_decimal::Decimal;
use std::str::FromStr;

// Export functions to be accessible from JavaScript
#[wasm_bindgen]
pub fn add(a: &str, b: &str) -> String {
    match (Decimal::from_str(a), Decimal::from_str(b)) {
        (Ok(dec_a), Ok(dec_b)) => (dec_a + dec_b).to_string(),
        (Err(e), _) => format!("Error parsing a: {}", e),
        (_, Err(e)) => format!("Error parsing b: {}", e),
    }
}


#[wasm_bindgen]
pub fn subtract(a: &str, b: &str) -> String{
    match (Decimal::from_str(a), Decimal::from_str(b)) {
        (Ok(dec_a), Ok(dec_b)) => (dec_a - dec_b).to_string(),
        (Err(e), _) => format!("Error parsing a: {}", e),
        (_, Err(e)) => format!("Error parsing b: {}", e),
    }
}


#[wasm_bindgen]
pub fn multiply(a: &str, b: &str) -> String {
    match (Decimal::from_str(a), Decimal::from_str(b)) {
        (Ok(dec_a), Ok(dec_b)) => (dec_a * dec_b).to_string(),
        (Err(e), _) => format!("Error parsing a: {}", e),
        (_, Err(e)) => format!("Error parsing b: {}", e),
    }
}


#[wasm_bindgen]
pub fn divide(a: &str, b: &str) -> String {
    match (Decimal::from_str(a), Decimal::from_str(b)) {
        (Ok(dec_a), Ok(dec_b)) => {
            if dec_b.is_zero() {
                "Error: Division by zero is not allowed".to_string()
            } else {
                (dec_a / dec_b).to_string()
            }
        }
        (Err(e), _) => format!("Error parsing a: {}", e),
        (_, Err(e)) => format!("Error parsing b: {}", e),
    }
}