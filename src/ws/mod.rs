// src/ws/mod.rs
pub mod commands;
pub mod config;
pub mod connection;
pub mod crypto;
pub mod rates;

pub use crypto::run_crypto_websocket;
pub use rates::run_exchange_websocket;

use tokio::sync::{mpsc, oneshot};
use crate::channel::WSCommand;
use std::sync::OnceLock;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

// Channels for communicating with the main crypto task
pub static CRYPTO_COMMANDS_TX: OnceLock<mpsc::Sender<WSCommand>> = OnceLock::new();

// Registry for pending ledger responses.
// Tuple: (Response Sender, Registration Confirmation Sender)
pub static LEDGER_REGISTRY_TX: OnceLock<mpsc::Sender<(oneshot::Sender<Value>, oneshot::Sender<()>)>> = OnceLock::new();

// Channel for outgoing messages
pub static CRYPTO_OUTGOING_TX: OnceLock<mpsc::Sender<Message>> = OnceLock::new();

// Shutdown channels
pub static EXCHANGE_SHUTDOWN_TX: OnceLock<mpsc::Sender<()>> = OnceLock::new();
pub static CRYPTO_SHUTDOWN_TX: OnceLock<mpsc::Sender<()>> = OnceLock::new();
