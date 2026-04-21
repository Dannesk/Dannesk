// ws/commands/bitcoin_ledger.rs
use crate::channel::{CHANNEL, ProgressState};
use crate::ws::{LEDGER_REGISTRY_TX, CRYPTO_OUTGOING_TX};
use serde_json::{Value, json};
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite::Message;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Utxo {
    pub txid: String,
    pub vout: u32,
    pub amount: u64, // Amount in satoshis
}

pub async fn fetch_utxo_data(
    wallet_address: &str,
) -> Result<Vec<Utxo>, String> {
    static FAILED: &str = "Error: Unable to fetch UTXO";

    // Create a oneshot channel to receive the response
    let (tx, rx) = oneshot::channel();
    let (confirm_tx, confirm_rx) = oneshot::channel();

    // Register the sender with the main crypto loop and wait for confirmation
    if let Some(reg_tx) = LEDGER_REGISTRY_TX.get() {
        let _ = reg_tx.send((tx, confirm_tx)).await;
        let _ = confirm_rx.await;
    } else {
        let msg = "Internal Error: Registry not initialized".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: msg.clone(),
        }));
        return Err(msg);
    }

    let msg_json = json!({
        "command": "get_bitcoin_utxo_data",
        "address": wallet_address
    });

    if let Some(out_tx) = CRYPTO_OUTGOING_TX.get() {
        if out_tx.send(Message::text(msg_json.to_string())).await.is_err() {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: FAILED.to_string(),
            }));
            return Err(FAILED.to_string());
        }
    } else {
        let msg = "Internal Error: Outgoing channel not initialized".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: msg.clone(),
        }));
        return Err(msg);
    }

    // Wait for the response
    let timeout = tokio::time::sleep(Duration::from_secs(20));
    tokio::pin!(timeout);
    tokio::pin!(rx);

    tokio::select! {
        _ = &mut timeout => {
            let msg = "Timeout waiting for UTXO data".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: msg.clone(),
            }));
            Err(msg)
        }
        response = &mut rx => {
            match response {
                Ok(data) => parse_utxo_response(data).await,
                Err(_) => {
                    let msg = "Oneshot closed".to_string();
                    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                        progress: 1.0,
                        message: msg.clone(),
                    }));
                    Err(msg)
                }
            }
        }
    }
}

async fn parse_utxo_response(data: Value) -> Result<Vec<Utxo>, String> {
    static FAILED: &str = "Error: Failed to process UTXO data";

    if let Some(error) = data["utxo_info"]["error"].as_str() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.to_string(),
        }));
        return Err(format!("Server error: {}", error));
    }

    let utxos: Vec<Utxo> = data["utxo_info"]["utxos"]
        .as_array()
        .ok_or_else(|| {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: FAILED.to_string(),
            }));
            FAILED.to_string()
        })?
        .iter()
        .filter_map(|utxo| {
            let txid = match utxo["txid"].as_str() {
                Some(s) => s.to_string(),
                None => {
                    return None;
                }
            };
            let vout = match utxo["vout"].as_u64() {
                Some(n) => n as u32,
                None => {
                    return None;
                }
            };
            let amount = match utxo["value"].as_str().and_then(|s| s.parse::<u64>().ok()) {
                Some(n) => n,
                None => {
                    return None;
                }
            };
            Some(Utxo { txid, vout, amount })
        })
        .collect();

    if utxos.is_empty() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Error: No UTXOs found".to_string(),
        }));
        return Err("No UTXOs found".to_string());
    }

    for utxo in &utxos {
        if utxo.amount == 0 {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: UTXO with zero amount".to_string(),
            }));
            return Err("UTXO with zero amount".to_string());
        }
        if utxo.txid.len() != 64
            || !utxo.txid.chars().all(|c| c.is_ascii_hexdigit())
        {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Invalid UTXO txid".to_string(),
            }));
            return Err(format!("Invalid UTXO txid: {}", utxo.txid));
        }
    }

    Ok(utxos)
}

pub async fn process_response(
    message: Message,
    _wallet_address: &str,
) -> Result<Option<Vec<Utxo>>, String> {
    match message {
        Message::Text(text) => {
            if let Ok(data) = serde_json::from_str::<Value>(&text) {
                if data.get("command").and_then(|c| c.as_str()) == Some("get_bitcoin_utxo_data") {
                    return parse_utxo_response(data).await.map(Some);
                }
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}
