use crate::channel::{
    BitcoinTransactionStatus, BtcTransactionData, BtcTransactionState, CHANNEL, WSCommand,
};
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use std::collections::HashMap;
use tokio_tungstenite::tungstenite::Message;

pub async fn execute(
    _bitcoin_current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    if let Some(wallet) = &cmd.wallet {
        let msg_json = json!({ "command": "get_bitcoin_cached_balance", "wallet": wallet });
        if let Some(tx) = CRYPTO_OUTGOING_TX.get() {
            let _ = tx.send(Message::text(msg_json.to_string())).await;
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
    match message {
        Message::Text(text) => {
            let data: Value =
                serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

            let command = data.get("command").and_then(|c| c.as_str());
            if command != Some("get_bitcoin_cached_balance") {
                return Ok(());
            }

            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
                let bitcoin_wallet_rx = CHANNEL.bitcoin_wallet_rx.clone();
                let (_current_balance, _wallet_opt, private_key_deleted) =
                    bitcoin_wallet_rx.borrow().clone();

                // Process balance
                let balance_btc =
                    if let Some(balance) = data.get("balance").and_then(|b| b.as_str()) {
                        if balance == "0" && data.get("balance").is_none() {
                            0.0
                        } else if let Ok(balance) = balance.parse::<f64>() {
                            balance / 100_000_000.0 // Convert satoshis to BTC
                        } else {
                            return Err(format!("Invalid balance format: {}", balance));
                        }
                    } else {
                        return Err("Missing balance field".to_string());
                    };

                // Process transactions
                let mut transactions_map = HashMap::new();
                if let Some(transactions) = data.get("transaction").and_then(|t| t.as_array()) {
                    for tx in transactions {
                        let txid = tx.get("txid").and_then(|h| h.as_str()).map(|s| s.to_string());
                        if txid.is_none() { continue; }
                        let txid = txid.unwrap();
                        let status = match tx.get("status").and_then(|s| s.as_str()) {
                            Some("pending") => BitcoinTransactionStatus::Pending,
                            Some("confirmed") | Some("success") => BitcoinTransactionStatus::Success,
                            Some("failed") => BitcoinTransactionStatus::Failed,
                            Some("cancelled") => BitcoinTransactionStatus::Cancelled,
                            _ => continue,
                        };
                        let tx_data = BtcTransactionData {
                            txid: txid.clone(),
                            status,
                            amount: tx.get("amount").and_then(|a| a.as_str()).unwrap_or("0").to_string(),
                            fees: tx.get("fees").and_then(|f| f.as_str()).unwrap_or("0").to_string(),
                            receiver_addresses: tx.get("receiver_addresses").and_then(|r| r.as_array())
                                .map(|arr| arr.iter().filter_map(|a| a.as_str().map(|s| s.to_string())).collect())
                                .unwrap_or_default(),
                            sender_addresses: tx.get("sender_addresses").and_then(|s| s.as_array())
                                .map(|arr| arr.iter().filter_map(|a| a.as_str().map(|s| s.to_string())).collect())
                                .unwrap_or_default(),
                            timestamp: tx.get("timestamp").and_then(|t| t.as_str()).unwrap_or_default().to_string(),
                        };
                        transactions_map.insert(txid, tx_data);
                    }
                }

                let _ = CHANNEL.bitcoin_wallet_tx.send((balance_btc, Some(wallet.to_string()), private_key_deleted));
                let _ = CHANNEL.btc_transactions_tx.send(BtcTransactionState { transactions: transactions_map });

                Ok(())
            } else {
                Err("Missing wallet field".to_string())
            }
        }
        _ => Err("Non-text message received".to_string()),
    }
}
