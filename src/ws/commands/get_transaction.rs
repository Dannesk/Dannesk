use crate::channel::{CHANNEL, TransactionData, TransactionState, TransactionStatus, WSCommand};
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

pub async fn execute(
    _current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    if let Some(wallet) = &cmd.wallet {
        let msg_json = json!({ "command": "get_transaction", "wallet": wallet });
        if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
            let _ = tx.send(Message::text(msg_json.to_string())).await;
        }
        Ok(())
    } else {
        Err("Missing wallet parameter".to_string())
    }
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

            let command = data.get("command").and_then(|c| c.as_str());
            if command != Some("get_transaction") {
                return Ok(());
            }

            let transactions_data = if let Some(tx) = data.get("transaction") {
                if tx.is_null() {
                    Vec::new()
                } else {
                    if let Some(tx_data) = parse_tx(tx) {
                        vec![tx_data]
                    } else {
                        Vec::new()
                    }
                }
            } else {
                return Err("Missing transaction field".to_string());
            };

            if !transactions_data.is_empty() {
                let mut current_transactions = CHANNEL.transactions_rx.borrow().transactions.clone();
                for tx_data in transactions_data {
                    current_transactions.insert(tx_data.tx_id.clone(), tx_data);
                }
                let _ = CHANNEL.transactions_tx.send(TransactionState {
                    transactions: current_transactions,
                });
            }
            Ok(())
        }
        _ => Err("Non-text message received".to_string()),
    }
}

fn parse_tx(tx: &Value) -> Option<TransactionData> {
    let tx_id = tx.get("hash").and_then(|h| h.as_str())?.to_string();
    let status = match tx.get("status").and_then(|s| s.as_str()) {
        Some("success") => TransactionStatus::Success,
        Some("failed") => TransactionStatus::Failed,
        Some("pending") => TransactionStatus::Pending,
        Some("cancelled") => TransactionStatus::Cancelled,
        _ => return None,
    };
    Some(TransactionData {
        tx_id,
        status,
        execution_price: tx.get("price").and_then(|p| p.as_str()).unwrap_or("0").to_string(),
        order_type: tx.get("tx_type").and_then(|t| t.as_str()).unwrap_or_default().to_string().to_lowercase(),
        timestamp: tx.get("timestamp").and_then(|t| t.as_str()).unwrap_or_default().to_string(),
        amount: tx.get("amount").and_then(|a| a.as_str()).unwrap_or("0").to_string(),
        currency: tx.get("currency").and_then(|c| c.as_str()).unwrap_or_default().to_string(),
        fee: tx.get("fee").and_then(|f| f.as_str()).unwrap_or_default().to_string(),
        flags: tx.get("flags").and_then(|f| f.as_str()).map(|s| s.to_string()),
        receiver: tx.get("receiver").and_then(|r| r.as_str()).unwrap_or_default().to_string(),
        sender: tx.get("sender").and_then(|s| s.as_str()).unwrap_or_default().to_string(),
    })
}
