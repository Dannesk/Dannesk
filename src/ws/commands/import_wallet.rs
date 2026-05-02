use crate::channel::{CHANNEL, ProgressState, WSCommand};
use crate::bridge::json_storage;
use crate::ws::CRYPTO_OUTGOING_TX;
use serde_json::{Value, json};
use tokio_tungstenite::tungstenite::Message;

fn cleanup_failed_import() {
    let _ = json_storage::remove_json("xrp.json");
    let _ = json_storage::remove_json("xrp_encrypt.json");
}

// ====================== HELPER (add new assets here only) ======================
fn parse_asset(
    data: &Value,
    has_key: &str,
    balance_key: &str,
    limit_key: &str,
) -> (f64, bool, Option<f64>) {
    let has = data.get(has_key).and_then(|h| h.as_bool()).unwrap_or(false);
    let balance = data
        .get(balance_key)
        .and_then(|b| b.as_str())
        .and_then(|b| b.parse::<f64>().ok())
        .unwrap_or(0.0);
    let limit = data
        .get(limit_key)
        .and_then(|l| l.as_str())
        .and_then(|l| l.parse::<f64>().ok());

    (balance, has, limit)
}

pub async fn execute(
    _current_wallet: String,
    cmd: WSCommand,
) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet import failed";
    if let Some(wallet) = cmd.wallet {
        let msg_json = json!({"command": "import_wallet", "wallet": wallet});

        if let Some(tx) = CRYPTO_OUTGOING_TX.get()
            && tx.send(Message::text(msg_json.to_string())).await.is_err() {
             cleanup_failed_import(); // <--- Cleanup on Send failure

                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: FAILED.to_string(),
                }));
                return Err(FAILED.to_string());
            
        }
        Ok(())
    } else {
        cleanup_failed_import();
        Err(FAILED.to_string())
    }
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    static FAILED: &str = "Error: Wallet import failed";
    match message {
        Message::Text(text) => {
            let data: Value = serde_json::from_str(&text).map_err(|e| {
                cleanup_failed_import(); 
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: format!("{}: Failed to parse JSON: {}", FAILED, e),
                }));
                format!("Failed to parse JSON: {}", e)
            })?;

           
            if let Some(wallet) = data.get("wallet").and_then(|w| w.as_str()) {
               

                let balance_xrp = data
                    .get("balance")
                    .and_then(|b| b.as_str())
                    .and_then(|b| b.parse::<f64>().ok())
                    .map(|b| b / 1_000_000.0)
                    .unwrap_or(0.0);

                let (rlusd_balance, has_rlusd, trustline_limit) =
                    parse_asset(&data, "has_rlusd", "rlusd_balance", "trustline_limit");

                let (euro_balance, has_euro, trustline_euro_limit) =
                    parse_asset(&data, "has_euro", "euro_balance", "trustline_euro_limit");

                let (xsgd_balance, has_xsgd, trustline_xsgd_limit) =
                    parse_asset(&data, "has_xsgd", "xsgd_balance", "trustline_xsgd_limit");

                let (_, _, private_key_deleted) = *CHANNEL.wallet_balance_rx.borrow();
                let _ = CHANNEL.wallet_balance_tx.send((balance_xrp, Some(wallet.to_string()), private_key_deleted));
                let _ = CHANNEL.rlusd_tx.send((rlusd_balance, has_rlusd, trustline_limit));
                let _ = CHANNEL.euro_tx.send((euro_balance, has_euro, trustline_euro_limit));
                let _ = CHANNEL.sgd_tx.send((xsgd_balance, has_xsgd, trustline_xsgd_limit));

                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Wallet imported successfully".to_string(),
                }));
            } else {
                // Server returned JSON
                cleanup_failed_import();
            }
        }
        _ => {
            cleanup_failed_import();
        }
    }
    Ok(())
}
