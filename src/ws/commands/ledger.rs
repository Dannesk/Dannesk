// ws/commands/ledger.rs
//this module fetches fee from xrp ledger (mobile version)
use crate::channel::{CHANNEL, ProgressState};
use crate::ws::{LEDGER_REGISTRY_TX, CRYPTO_OUTGOING_TX};
use serde_json::{Value, json};
use tokio::sync::oneshot;
use tokio_tungstenite::tungstenite::Message;
use std::time::Duration;

pub async fn fetch_ledger_data(
    wallet_address: &str,
) -> Result<(u32, String), String> {
    static FAILED: &str = "Error: Unable to fetch fee or sequence";

    // Create oneshot for response
    let (tx, rx) = oneshot::channel();
    let (confirm_tx, confirm_rx) = oneshot::channel();

    // Register with main crypto loop
    if let Some(reg_tx) = LEDGER_REGISTRY_TX.get() {
        let _ = reg_tx.send((tx, confirm_tx)).await;
        let _ = confirm_rx.await;
    } else {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Internal Error: Registry not initialized".to_string(),
        }));
        return Err("Internal Error: Registry not initialized".to_string());
    }

    let msg_json = json!({
        "command": "get_ledger_data",
        "account": wallet_address
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
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Internal Error: Outgoing channel not initialized".to_string(),
        }));
        return Err("Internal Error: Outgoing channel not initialized".to_string());
    }

    // Wait for response (clean oneshot, no spam)
    let response = tokio::select! {
        _ = tokio::time::sleep(Duration::from_secs(20)) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Timeout waiting for ledger response".to_string(),
            }));
            return Err("Timeout".to_string());
        }
        response = rx => response,
    };

    match response {
        Ok(data) => parse_ledger_response(data, wallet_address).await,
        Err(_) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Oneshot closed - no response received".to_string(),
            }));
            Err("Oneshot closed".to_string())
        }
    }
}

async fn parse_ledger_response(
    data: Value,
    _current_wallet: &str,
) -> Result<(u32, String), String> {
    static FAILED: &str = "Error: Failed to process ledger data";


    if data.get("error").and_then(|e| e.as_str()).is_some() {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: "Ledger returned error from server".to_string(),
        }));
        return Err(FAILED.to_string());
    }

    let fee = match data["fee"].as_str() {
        Some(f) => f.to_string(),
        None => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Ledger response missing 'fee' field".to_string(),
            }));
            return Err(FAILED.to_string());
        }
    };

    let sequence_str = match data["sequence"].as_str() {
        Some(s) => s,
        None => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Ledger response missing 'sequence' field".to_string(),
            }));
            return Err(FAILED.to_string());
        }
    };

    let sequence = match sequence_str.parse::<u64>() {
        Ok(n) => n,
        Err(_) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Failed to parse sequence number".to_string(),
            }));
            return Err(FAILED.to_string());
        }
    };
    let sequence: u32 = match sequence.try_into() {
        Ok(n) => n,
        Err(_) => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Sequence number too large for u32".to_string(),
            }));
            return Err(FAILED.to_string());
        }
    };

    Ok((sequence, fee))
}

pub async fn process_response(
    message: Message,
    current_wallet: &str,
) -> Result<Option<(u32, String)>, String> {
    match message {
        Message::Text(text) => {
                    if let Ok(data) = serde_json::from_str::<Value>(&text)
           && data.get("command").and_then(|c| c.as_str()) == Some("get_ledger_data") {
           return parse_ledger_response(data, current_wallet).await.map(Some);

                
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}