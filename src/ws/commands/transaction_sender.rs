// ws/commands/transaction_sender.rs
//This module handles sending the transaction via the WebSocket connection.

use crate::channel::{CHANNEL, ProgressState};
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::json;
use tokio_tungstenite::tungstenite::Message;
use uuid::Uuid;

pub async fn send_transaction(
    wallet: &str,
    tx_type: &str,
    tx_blob: String,
) -> Result<(), String> {
    let tx_id = Uuid::new_v4().to_string();
    let msg_json = json!({
        "command": "submit_transaction",
        "wallet": wallet,
        "tx_type": tx_type,
        "tx_id": tx_id,
        "signed_blob": json!({ "tx_blob": tx_blob })
    });

    if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
        if tx.send(Message::text(msg_json.to_string())).await.is_err() {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Failed to Send Transaction".to_string(),
            }));
            return Err("Error: Failed to Send Transaction".to_string());
        }
    } else {
        return Err("Internal Error: Outgoing channel not initialized".to_string());
    }

    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.7,
        message: "Sending Transaction".to_string(),
    }));

    Ok(())
}
