use crate::channel::{CHANNEL, ProgressState, WSCommand};
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

pub async fn execute(
    _current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet deletion failed";
    if let Some(wallet) = cmd.wallet {
        let msg_json = json!({"command": "delete_wallet", "wallet": wallet});

        if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
            if tx.send(Message::text(msg_json.to_string())).await.is_err() {
                return Err(FAILED.to_string());
            }
        }
        Ok(())
    } else {
        Err(FAILED.to_string())
    }
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet deletion failed";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|_| FAILED.to_string())?;

            if data.get("status").and_then(|s| s.as_str()) == Some("deleted") {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Wallet deleted successfully".to_string(),
                }));
            }
        }
        _ => {}
    }
    Ok(())
}
