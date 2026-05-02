use crate::bridge::json_storage;
use sha2::{Sha256, Digest};
use base64::Engine;
use base64::engine::general_purpose::STANDARD as BASE64;
use rand::RngExt;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum PinError {
    InvalidPin,
    IoError(String),
    PinNotSet,
    IncorrectPin,
}

impl fmt::Display for PinError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PinError::InvalidPin => write!(f, "Invalid PIN: must be a six-digit number"),
            PinError::IoError(e) => write!(f, "IO error: {}", e),
            PinError::PinNotSet => write!(f, "PIN not set"),
            PinError::IncorrectPin => write!(f, "Incorrect PIN"),
        }
    }
}

impl Error for PinError {}

#[derive(Serialize, Deserialize, Debug)]
pub struct PinData {
    pub pin_hash: String, // Base64-encoded SHA-256 hash
    pub pin_salt: String, // Base64-encoded salt
}

pub fn load_pin_data() -> Result<PinData, PinError> {
    json_storage::read_json("pin.json").map_err(|e| PinError::IoError(e.to_string()))
}

pub fn save_pin_data(pin_data: &PinData) -> Result<(), PinError> {
    json_storage::write_json("pin.json", pin_data).map_err(|e| PinError::IoError(e.to_string()))?;
    Ok(())
}

pub fn set_pin(pin: &str) -> Result<(), PinError> {
    if !pin.chars().all(|c| c.is_ascii_digit()) || pin.len() != 6 {
        return Err(PinError::InvalidPin);
    }

    let salt: [u8; 16] = rand::rng().random();

    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(pin.as_bytes());
    let hash = hasher.finalize();

    let pin_data = PinData {
        pin_hash: BASE64.encode(hash),
        pin_salt: BASE64.encode(salt),
    };

    save_pin_data(&pin_data)?;
    Ok(())
}

pub fn verify_pin(pin: &str) -> Result<(), PinError> {
    let pin_data = load_pin_data().map_err(|_| PinError::PinNotSet)?;

    let stored_hash = BASE64
        .decode(&pin_data.pin_hash)
        .map_err(|_| PinError::IncorrectPin)?;
    let salt = BASE64
        .decode(&pin_data.pin_salt)
        .map_err(|_| PinError::IncorrectPin)?;

    let mut hasher = Sha256::new();
    hasher.update(&salt);
    hasher.update(pin.as_bytes());
    let computed_hash = hasher.finalize();

    // Constant-time comparison is preferred but for a 6-digit PIN on a personal device,
    // standard equality is usually acceptable. However, we have the hashes here:
    if computed_hash.as_slice() == stored_hash.as_slice() {
        Ok(())
    } else {
        Err(PinError::IncorrectPin)
    }
}

pub fn change_pin(old_pin: &str, new_pin: &str) -> Result<(), PinError> {
    verify_pin(old_pin)?;
    set_pin(new_pin)?;
    Ok(())
}
