// ws/commands/bitcoin_import_wallet.rs

use crate::channel::{CHANNEL, ProgressState, WSCommand};
use crate::bridge::json_storage;
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

// Helper to remove files if communication fails
fn cleanup_failed_import() {
    let _ = json_storage::remove_json("btc.json");
    let _ = json_storage::remove_json("btc_encrypt.json");
}

pub async fn execute(
    _bitcoin_current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Bitcoin wallet import failed";
    if let Some(wallet) = cmd.wallet {
        let msg_json = json!({"command": "import_bitcoin_wallet", "wallet": wallet});

        if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
            if tx.send(Message::text(msg_json.to_string())).await.is_err() {
                cleanup_failed_import(); // Rollback local files on pipe error
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                return Err(FAILED.to_string());
            }
        }
        Ok(())
    } else {
        cleanup_failed_import();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: FAILED.to_string(),
        }));
        Err(FAILED.to_string())
    }
}

pub async fn process_response(
    message: Message,
    _bitcoin_current_wallet: &str,
) -> Result<(), String> {
    static FAILED: &str = "Error: Bitcoin wallet import failed";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|e| {
                cleanup_failed_import(); // Rollback local files on parse error
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("{}: Failed to parse JSON: {}", FAILED, e),
                }));
                format!("Failed to parse JSON: {}", e)
            })?;

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                // Files are already created by the logic layer, just update UI/Live state
                
                let balance_btc = data
                    .get("balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .map(|b| b / 100_000_000.0) // Convert satoshis to BTC
                    .unwrap_or(0.0);

                let _ = CHANNEL.bitcoin_wallet_tx.send((balance_btc, Some(wallet.to_string()), false));

                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Bitcoin wallet imported successfully".to_string(),
                }));
            } else {
                // If server returned error or invalid JSON structure
                cleanup_failed_import();
            }
        }
        _ => {
            cleanup_failed_import();
        }
    }
    Ok(())
}