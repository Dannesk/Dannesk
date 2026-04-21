// src/ui/managexrp/transactions.rs
use crate::channel::{ActiveView, TransactionStatus};
use crate::context::XrpContext;
use crate::utils::styles;
use chrono::{DateTime, Utc};
use dioxus_native::prelude::*; // Or dioxus::prelude::* depending on your version

#[component]
pub fn view() -> Element {
    let xrp_ctx = use_context::<XrpContext>();
    let mut xrp_modal = xrp_ctx.xrp_modal;
    let tx_state = xrp_ctx.transactions.read();

    let mut sorted_txs: Vec<_> = tx_state.transactions.values().collect();
    sorted_txs.sort_by_key(|tx| {
        std::cmp::Reverse(
            tx.timestamp
                .parse::<DateTime<Utc>>()
                .unwrap_or(DateTime::<Utc>::MIN_UTC),
        )
    });

    let display_txs = sorted_txs.into_iter().take(100).collect::<Vec<_>>();

    let on_back_click = move |_: MouseEvent| {
        xrp_modal.with_mut(|m| {
            if let Some(previous) = m.last_view {
                m.view_type = previous;
            } else {
                m.view_type = ActiveView::Xrp;
            }
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
                max-width: 800px; /* narrowed from 1500px for card readability */
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
                gap: 8px; /* equivalent to arrangement.spacedBy(8.dp) */
            }
            /* Card Styling */
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
            
            /* Expanded Details Styling */
            .tx-details {
                display: flex;
                flex-direction: column;
                margin-top: 12px;
                padding-top: 12px;
                border-top: 1px solid rgba(255, 255, 255, 0.1); /* fallback for border opacity */
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

            div { class: "section-label", "NETWORK_LOG // XRPL_TRANSACTIONS" }

            div { class: "tx-list",
                for tx in display_txs.into_iter() {
                    TransactionCard {
                        key: "{tx.tx_id}",
                        tx_id: tx.tx_id.clone(),
                        order_type: tx.order_type.clone(),
                        status: tx.status.clone(),
                        execution_price: tx.execution_price.clone(),
                        amount: tx.amount.clone(),
                        currency: tx.currency.clone(),
                        fee: tx.fee.clone(),
                        flags: tx.flags.clone(),
                        receiver: tx.receiver.clone(),
                        sender: tx.sender.clone(),
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
    order_type: String,
    status: TransactionStatus,
    execution_price: String,
    amount: String,
    currency: String,
    fee: String,
    flags: Option<String>,
    receiver: String,
    sender: String,
    timestamp: String,
) -> Element {
    let mut expanded = use_signal(|| false);

    let display_flags = flags.as_deref().unwrap_or("---");

    let (status_text, status_color) = match status {
        TransactionStatus::Success => ("SUCCESS", "var(--status-ok)"),
        TransactionStatus::Failed => ("FAILED", "var(--status-warn)"),
        TransactionStatus::Pending => ("PENDING", "var(--accent)"),
        TransactionStatus::Cancelled => ("CANCELLED", "var(--text-secondary)"),
    };

    let currency_display = if !currency.is_empty() {
        format!("{} {}", amount, currency)
    } else {
        amount.clone()
    };

    // Normalize type for matching
    let type_normalized = order_type.to_uppercase();

    rsx! {
        div {
            class: "tx-card",
            onclick: move |_| expanded.toggle(),

            // Collapsed Top Section
            div { class: "tx-row",
                div { class: "tx-type", "{order_type}" }
                div { class: "tx-amount", style: "white-space: nowrap;", "{currency_display}" }
            }
            div { class: "tx-row tx-mt-1",
                div { class: "tx-status", style: "color: {status_color};", "{status_text}" }
                div { class: "tx-date", "{timestamp}" }
            }

            // Expanded Detail Section
            if expanded() {
                div { class: "tx-details",
                    DetailItem { label: "TX_ID".to_string(), value: tx_id.clone() }
                    
                    if !fee.is_empty() {
                        DetailItem { label: "FEE".to_string(), value: fee.clone() }
                    }
                    
                    // Conditionally render fields based on transaction type
                    match type_normalized.as_str() {
                        "OFFERCREATE" | "OFFER_CREATE" => rsx! {
                            if !execution_price.is_empty() {
                                DetailItem { label: "EXEC_PRICE".to_string(), value: execution_price.clone() }
                            }
                            if !sender.is_empty() {
                                DetailItem { label: "SENDER".to_string(), value: sender.clone() }
                            }
                            if !receiver.is_empty() {
                                DetailItem { label: "RECEIVER".to_string(), value: receiver.clone() }
                            }
                        },
                        "TRUSTSET" | "TRUST_SET" => rsx! {
                            // Currently identical to Payment, but isolated for future UI changes
                            if !sender.is_empty() {
                                DetailItem { label: "SENDER".to_string(), value: sender.clone() }
                            }
                            if !receiver.is_empty() {
                                DetailItem { label: "RECEIVER".to_string(), value: receiver.clone() }
                            }
                        },
                        // Default catch-all (Payment, etc.)
                        _ => rsx! {
                            if !sender.is_empty() {
                                DetailItem { label: "SENDER".to_string(), value: sender.clone() }
                            }
                            if !receiver.is_empty() {
                                DetailItem { label: "RECEIVER".to_string(), value: receiver.clone() }
                            }
                        }
                    }

                    if display_flags != "---" {
                        DetailItem { label: "FLAGS".to_string(), value: display_flags.to_string() }
                    }
                }
            }
        }
    }
}
// Reusable micro-component for the label/value pairings inside the expanded view
#[component]
fn DetailItem(label: String, value: String) -> Element {
    rsx! {
        div { class: "detail-row",
            div { class: "detail-label", "{label}" }
            div { class: "detail-value", title: "{value}", "{value}" } // Title adds tooltip hover for long hashes
        }
    }
}