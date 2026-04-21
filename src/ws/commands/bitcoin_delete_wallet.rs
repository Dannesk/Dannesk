// ws/commands/bitcoin_delete_wallet.rs

use crate::channel::{CHANNEL, ProgressState, WSCommand};
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

pub async fn execute(
    _bitcoin_current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Bitcoin wallet deletion failed";
    if let Some(wallet) = cmd.wallet {
        let msg_json = json!({"command": "delete_bitcoin_wallet", "wallet": wallet});

        if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
            if tx.send(Message::text(msg_json.to_string())).await.is_err() {
                return Err(FAILED.to_string());
            }
        }
        Ok(())
    } else {
        Err("Missing wallet parameter".to_string())
    }
}

pub async fn process_response(
    message: Message,
    _bitcoin_current_wallet: &str,
) -> Result<(), String> {
    if let Message::Text(text) = message {
        let data: Value =
            serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

        if data.get("command").and_then(|c| c.as_str()) != Some("delete_bitcoin_wallet") {
            return Ok(());
        }

        if let Some(error) = data.get("error").and_then(|e| e.as_str()) {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: format!("Failed to delete wallet: {}", error),
            }));
        } else if data.get("status").and_then(|s| s.as_str()) == Some("deleted") {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Wallet deleted successfully".to_string(),
            }));
        }
    }
    Ok(())
}
