use crate::channel::{CHANNEL, WSCommand, Theme};
use crate::bridge::json_storage;
use serde_json::{self, Value};
use tokio::sync::mpsc;

pub fn load_wallets(commands_tx: mpsc::Sender<WSCommand>) {
    let filename = "settings.json";

    if let Ok(json) = json_storage::read_json::<Value>(filename) {
        // 1. File exists: Read values and update the channel
        let theme = json.get("theme")
            .and_then(|v| serde_json::from_value::<Theme>(v.clone()).ok())
            .unwrap_or(Theme::Dark);
        let is_hidden = json.get("is_hidden")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let _ = CHANNEL.theme_user_tx.send((theme, is_hidden));
    } else {
        // 2. File doesn't exist: Create it with default values
        let default_settings = serde_json::json!({
            "theme": Theme::Dark,
            "is_hidden": false
        });
        let _ = json_storage::write_json(filename, &default_settings);
        // No need to send to channel here; it's already Dark/false by default
    }

    // Load XRP wallet from xrp.json
      if let Ok(path) = json_storage::get_config_path("xrp.json")
    && path.exists()
        && let Ok(json) = json_storage::read_json::<Value>("xrp.json") {


                let address = json
                    .get("address")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let private_key_deleted = json
                    .get("private_key_deleted")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // Update XRP wallet channel with initial data
                if !address.is_empty() {
                    let _ =
                        CHANNEL
                            .wallet_balance_tx
                            .send((0.0, Some(address.clone()), private_key_deleted));

                    // Send get_cached_balance command
                    let command = WSCommand {
                        command: "get_cached_balance".to_string(),
                        wallet: Some(address.clone()),
                        recipient: None,
                        amount: None,
                        passphrase: None,
                        trustline_limit: None,
                        fee: None,
                        tx_type: None,
                        taker_pays: None,
                        taker_gets: None,
                        seed: None,
                        flags: None,
                        wallet_type: None,
                        bip39: None,
                    };
                    let _ = commands_tx.try_send(command);
                }
    }

    // Load Bitcoin wallet from btc.json
    if let Ok(path) = json_storage::get_config_path("btc.json")
        && path.exists()
  && let Ok(json) = json_storage::read_json::<Value>("btc.json") {


                let address = json
                    .get("address")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let private_key_deleted = json
                    .get("private_key_deleted")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                // Update BTC wallet channel with initial data
                if !address.is_empty() {
                    let _ =
                        CHANNEL
                            .bitcoin_wallet_tx
                            .send((0.0, Some(address.clone()), private_key_deleted));

                    // Send get_bitcoin_cached_balance command
                    let command = WSCommand {
                        command: "get_bitcoin_cached_balance".to_string(),
                        wallet: Some(address.clone()),
                        recipient: None,
                        amount: None,
                        passphrase: None,
                        trustline_limit: None,
                        fee: None,
                        tx_type: None,
                        taker_pays: None,
                        taker_gets: None,
                        seed: None,
                        flags: None,
                        wallet_type: None,
                        bip39: None,
                    };
                    let _ = commands_tx.try_send(command);
                }
    }
}
