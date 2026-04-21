use crate::channel::{CHANNEL, ProgressState, WSCommand};

pub fn validate_inputs(
    cmd: &WSCommand,
    bitcoin_current_wallet: &str,
) -> Result<(String, String, String), String> {
    let tx_type = cmd
        .tx_type
        .as_ref()
        .ok_or_else(|| {
            let error = "Missing tx_type".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: error.clone(),
            }));
            error
        })?
        .to_string();

    let wallet = cmd
        .wallet
        .as_ref()
        .ok_or_else(|| {
            let error = "Missing wallet".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: error.clone(),
            }));
            error
        })?
        .to_string();

    if cmd.passphrase.is_none() && cmd.seed.is_none() {
        let error = "Error: Must provide either passphrase or seed".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    // Fix: Allow empty current_wallet and trace mismatch if it exists.
    // This was likely killing the transaction because bitcoin_current_wallet starts empty.
    if !bitcoin_current_wallet.is_empty() && wallet != bitcoin_current_wallet {
        let error = format!("Wallet mismatch: {} != {}", wallet, bitcoin_current_wallet);
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    if tx_type != "BTC" {
        let error = "Invalid transaction type for Bitcoin".to_string();
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: error.clone(),
        }));
        return Err(error);
    }

    Ok((tx_type, wallet, String::new()))
}
