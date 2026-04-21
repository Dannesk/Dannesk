// src/ui/managebtc/transactions.rs
use crate::channel::{BitcoinTransactionStatus, BtcActiveView};
use crate::context::BtcContext;
use crate::utils::styles;
use chrono::{DateTime, TimeZone, Utc};
use dioxus_native::prelude::*;

fn parse_timestamp(ts: &str) -> DateTime<Utc> {
    ts.parse::<i64>()
        .ok()
        .and_then(|s| Utc.timestamp_opt(s, 0).single())
        .unwrap_or(DateTime::<Utc>::MIN_UTC)
}

fn format_timestamp(ts: &str) -> String {
    if ts.is_empty() || ts == "0" {
        "—".to_string()
    } else {
        parse_timestamp(ts).format("%Y-%m-%d %H:%M").to_string()
    }
}

#[component]
pub fn view() -> Element {
    let mut btc_ctx = use_context::<BtcContext>();
    let tx_state = btc_ctx.btc_transactions.read();

    let mut sorted_txs: Vec<_> = tx_state.transactions.values().collect();
    sorted_txs.sort_by_key(|tx| std::cmp::Reverse(parse_timestamp(&tx.timestamp)));

    let display_txs = sorted_txs.into_iter().take(100).collect::<Vec<_>>();

    let on_back_click = move |_: MouseEvent| {
        btc_ctx.btc_modal.with_mut(|state| {
            state.view_type = BtcActiveView::Btc;
        });
    };

    rsx! {
        style { {r#"
            .tx-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                align-items: center; 
                padding-top: 4rem;
                padding-bottom: 2rem;
                color: var(--text);
                font-family: 'JetBrains Mono', monospace;
                box-sizing: border-box;
            }
            .back-button-container {
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10;
            }
            .section-label {
                width: 100%;
                max-width: 800px;
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 1rem;
            }
            .tx-list {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                gap: 8px;
            }
            .tx-card {
                display: flex;
                flex-direction: column;
                width: 100%;
                background-color: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 12px;
                cursor: pointer;
                box-sizing: border-box;
            }
            .tx-card:hover {
                border-color: var(--accent);
            }
            .tx-row {
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                align-items: center;
                width: 100%;
            }
            .tx-mt-1 { margin-top: 4px; }
            .tx-type { font-weight: 700; font-size: 0.8rem; text-transform: uppercase; color: var(--text); }
            .tx-amount { font-weight: 700; white-space: nowrap;  font-size: 0.8rem; color: var(--accent); }
            .tx-status { font-size: 0.65rem; font-weight: 700; }
            .tx-date { font-size: 0.65rem; color: var(--text-secondary); }
            
            .tx-details {
                display: flex;
                flex-direction: column;
                margin-top: 12px;
                padding-top: 12px;
                border-top: 1px solid rgba(255, 255, 255, 0.1);
                gap: 8px;
            }
            .detail-row {
                display: flex;
                flex-direction: column;
                gap: 2px;
            }
            .detail-label {
                font-size: 0.55rem;
                color: var(--text-secondary);
                opacity: 0.7;
            }
            .detail-value {
                font-size: 0.7rem;
                color: var(--text-secondary);
                overflow: hidden;
                white-space: nowrap;
                text-overflow: ellipsis;
            }
        "#} }

        div { class: "tx-container",
            div {
                class: "back-button-container",
                onclick: on_back_click,
                styles::previous_icon_button { text_color: "var(--text)".to_string() }
            }

            div { class: "section-label", "NETWORK_LOG // BITCOIN_TRANSACTIONS" }

            div { class: "tx-list",
                for tx in display_txs.into_iter() {
                    TransactionCard {
                        key: "{tx.txid}",
                        tx_id: tx.txid.clone(),
                        status: tx.status.clone(),
                        amount: tx.amount.clone(),
                        fee: tx.fees.clone(),
                        receivers: tx.receiver_addresses.clone(),
                        senders: tx.sender_addresses.clone(),
                        timestamp: tx.timestamp.clone(),
                    }
                }
            }
        }
    }
}

#[component]
fn TransactionCard(
    tx_id: String,
    status: BitcoinTransactionStatus,
    amount: String,
    fee: String,
    receivers: Vec<String>,
    senders: Vec<String>,
    timestamp: String,
) -> Element {
    let mut expanded = use_signal(|| false);

    let (status_text, status_color) = match status {
        BitcoinTransactionStatus::Success => ("OK", "var(--status-ok)"),
        BitcoinTransactionStatus::Failed => ("FAIL", "var(--status-warn)"),
        BitcoinTransactionStatus::Pending => ("WAIT", "var(--accent)"),
        BitcoinTransactionStatus::Cancelled => ("VOID", "var(--text-secondary)"),
    };

    let full_senders = senders.join(", ");
    let full_receivers = receivers.join(", ");

    rsx! {
        div {
            class: "tx-card",
            onclick: move |_| expanded.toggle(),

            div { class: "tx-row",
                div { class: "tx-type", "BITCOIN" }
                div { class: "tx-amount", "{amount} BTC" }
            }
            div { class: "tx-row tx-mt-1",
                div { class: "tx-status", style: "color: {status_color};", "{status_text}" }
                div { class: "tx-date", "{format_timestamp(&timestamp)}" }
            }

            if expanded() {
                div { class: "tx-details",
                    DetailItem { label: "TX_ID".to_string(), value: tx_id.clone() }
                    DetailItem { label: "FEE".to_string(), value: fee.clone() }
                    DetailItem { label: "SENDER".to_string(), value: full_senders }
                    DetailItem { label: "RECEIVER".to_string(), value: full_receivers }
                }
            }
        }
    }
}

#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "detail-row",
            div { class: "detail-label", "{label}" }
            div { class: "detail-value", title: "{value}", "{value}" }
        }
    }
}