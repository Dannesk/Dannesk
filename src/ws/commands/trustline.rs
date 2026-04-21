use crate::channel::CHANNEL;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

#[derive(Clone, Copy)]
enum Asset {
    Rlusd,
    Euro,
    Xsgd,
}

impl Asset {
    fn from_command(command: &str) -> Option<Self> {
        match command {
            "get_trustline_limit" => Some(Asset::Rlusd),
            "get_trustline_euro_limit" => Some(Asset::Euro),
            "get_trustline_sgd_limit" => Some(Asset::Xsgd),
            _ => None,
        }
    }

    fn get_state(&self) -> (f64, bool, Option<f64>) {
        match self {
            Asset::Rlusd => *CHANNEL.rlusd_rx.borrow(),
            Asset::Euro => *CHANNEL.euro_rx.borrow(),
            Asset::Xsgd => *CHANNEL.sgd_rx.borrow(),
        }
    }

    fn send(&self, balance: f64, has: bool, limit: Option<f64>) -> Result<(), String> {
        match self {
            Asset::Rlusd => CHANNEL.rlusd_tx.send((balance, has, limit)),
            Asset::Euro => CHANNEL.euro_tx.send((balance, has, limit)),
            Asset::Xsgd => CHANNEL.sgd_tx.send((balance, has, limit)),
        }
        .map_err(|e| format!("Failed to send trustline update: {}", e))
    }
}

pub async fn execute(
    _current_wallet: String,
    _cmd: crate::channel::WSCommand,
) -> Result<(), String> {
    Ok(())
}

pub async fn process_response(message: Message, _current_wallet: &str) -> Result<(), String> {
    let Message::Text(text) = message else {
        return Err("Non-text message received".to_string());
    };

    let data: Value =
        serde_json::from_str(&text).map_err(|e| format!("Failed to parse JSON: {}", e))?;

    let command = data
        .get("command")
        .and_then(|c| c.as_str())
        .ok_or_else(|| "Missing command field".to_string())?;

    let Some(asset) = Asset::from_command(command) else {
        return Ok(());
    };

    let (current_balance, _current_has, current_limit) = asset.get_state();

    let trustline_limit = data
        .get("trustline_limit")
        .and_then(|l| l.as_str())
        .and_then(|l| l.parse::<f64>().ok())
        .or(current_limit);

    let has_asset = true;

    asset.send(current_balance, has_asset, trustline_limit)?;

    Ok(())
}
