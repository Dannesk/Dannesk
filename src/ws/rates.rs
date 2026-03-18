use crate::channel::CHANNEL;
use crate::ws::config::{EXCHANGE_WS_URL, MAX_RECONNECT_ATTEMPTS, RECONNECT_BACKOFF_SECONDS};
use futures_util::{SinkExt, StreamExt};
use serde_json::Value;
use tokio::sync::mpsc::Receiver;
use tokio_tungstenite::{connect_async, tungstenite::Message};

pub async fn run_exchange_websocket(mut shutdown_rx: Receiver<()>) -> Result<(), String> {
    let rates_tx = CHANNEL.rates_tx.clone();
    let ws_status_tx = CHANNEL.exchange_ws_status_tx.clone();

    let mut attempts = 0;

    loop {
        if attempts >= MAX_RECONNECT_ATTEMPTS {
            let _ = ws_status_tx.send(false);
            tokio::time::sleep(tokio::time::Duration::from_secs(RECONNECT_BACKOFF_SECONDS)).await;
            return Err("Failed to connect after max attempts".to_string());
        }

        let _ = ws_status_tx.send(false);

        let ws_stream = match connect_async(EXCHANGE_WS_URL).await {
            Ok((stream, _)) => {
                attempts = 0;
                let _ = ws_status_tx.send(true);
                stream
            }
            Err(_) => {
                attempts += 1;
                let delay_secs = attempts as u64;
                let _ = ws_status_tx.send(false);
                tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
                continue;
            }
        };

        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    let _ = ws_sink.close().await;
                    let _ = ws_status_tx.send(false);
                    return Ok(());
                }
                result = ws_stream.next() => {
                    match result {
                        Some(Ok(Message::Text(text))) => {
                            if let Ok(data) = serde_json::from_str::<Value>(&text)
                                && let (Some(symbol), Some(price)) = (
                                    data.get("symbol").and_then(|v| v.as_str()),
                                    data.get("price").and_then(|v| v.as_str()),
                                )
                                    && let Some(pair) = binance_stream_to_pair(symbol)
                                        && let Ok(rate) = price.parse::<f32>() {
                                            let mut new_rates = rates_tx.borrow().clone();
                                            new_rates.insert(pair, rate);
                                            let _ = rates_tx.send(new_rates);
                                        }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = ws_sink.send(Message::Pong(data)).await;
                        }
                        Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                            let _ = ws_status_tx.send(false);
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        attempts += 1;
        let delay_secs = attempts as u64;
        tokio::time::sleep(tokio::time::Duration::from_secs(delay_secs)).await;
    }
}

fn binance_stream_to_pair(stream: &str) -> Option<String> {
    match stream {
        "xrpusdt@ticker" => return Some("XRP/USD".to_string()),
        "btcusdt@ticker" => return Some("BTC/USD".to_string()),
        "eurusdt@ticker" => return Some("EUR/USD".to_string()),
        _ => {}
    }

    // Accept all clean pairs from backend (both directions)
    let parts: Vec<&str> = stream.split('/').collect();
    if parts.len() == 2 {
        let base = parts[0];
        let quote = parts[1];
        const VALID: [&str; 5] = ["XRP", "BTC", "USD", "EUR", "SGD"];
        if VALID.contains(&base) && VALID.contains(&quote) {
            return Some(stream.to_string());
        }
    }
    None
}
