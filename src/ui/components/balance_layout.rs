use crate::context::GlobalContext;
use crate::utils::reserves::XrpBalanceInfo;
use dioxus_native::prelude::*;

#[derive(Clone, PartialEq)]
pub enum LedgerInfo {
    None,
    Xrp(XrpBalanceInfo),
}

#[component]
pub fn BalanceLayout(
    asset_ticker: String,
    int_part: String,
    frac_part: String,
    formatted_raw_amount: String,
    status_color: String,
    status_text: String,
    network_protocol: String,
    send_btn: Element,
    receive_btn: Element,
    purge_btn: Element,
    delete_btn: Option<Element>,
    ledger_info: LedgerInfo,
    logo: Element,
) -> Element {
    let global = use_context::<GlobalContext>();

    let crypto_connected = *global.crypto_ws_status.read();
    let node_text = if crypto_connected { "Connected" } else { "Disconnected" };
    let node_bg = if crypto_connected { "var(--brand-blue)" } else { "var(--status-warn)" };

    let (xrp_status_text, xrp_status_bg) = if let LedgerInfo::Xrp(info) = &ledger_info {
        if info.is_active { ("Active", "var(--brand-blue)") } else { ("Inactive", "var(--status-warn)") }
    } else { ("", "") };

    // Strict nowrap pill for native rendering stability
    let pill_style = "padding: 4px 12px; border-radius: 9999px; font-size: 0.8em; font-weight: 600; color: var(--text); white-space: nowrap; display: inline-block; width: fit-content;";

    rsx! {
        style { {r#"
            /* THE FIX 1: The outer viewport shock absorber */
            .balance-outer-viewport {
                display: flex;
                flex-direction: row;
                width: 100%;
                flex: 1;
                align-items: center; 
                justify-content: center;
            }
            .modern-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 900px;
                font-family: 'Inter', sans-serif;
                gap: 1rem;
            }
            .balance-card {
                padding: 1.5rem;
                background: var(--bg-faint);
                border-radius: 12px;
                border: 1px solid var(--border);
            }
            .dashboard-grid {
                display: grid;
                grid-template-columns: 1.1fr 0.9fr;
                gap: 1rem;
                width: 100%; /* Force grid to respect container width */
            }
            .column {
                display: flex;
                flex-direction: column;
                gap: 1rem;
                min-width: 0; /* THE FIX 2: Prevents Taffy from freaking out during fr calculations */
            }
            .content-box {
                padding: 1.25rem;
                background: var(--bg-faint);
                border-radius: 12px;
                border: 1px solid var(--border);
            }
            .section-label {
                font-size: 0.75rem;
                color: var(--text-secondary);
                font-weight: 700;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                margin-bottom: 1rem;
                white-space: nowrap;
            }
            .info-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                font-size: 0.85rem;
                margin-bottom: 0.6rem;
                white-space: nowrap;
            }
            .button-group {
                display: flex;
                gap: 8px;
                flex-wrap: wrap;
            }
            .diag-label {
                font-size: 0.75rem;
                color: var(--text-secondary);
                margin-bottom: 2px;
                white-space: nowrap;
            }
            .diag-value {
                font-size: 0.9rem;
                font-weight: 600;
                white-space: nowrap;
            }
            .monospace-data { font-family: 'JetBrains Mono', monospace; white-space: nowrap; }
        "#} }

        // Box within a box wrapper
        div { class: "balance-outer-viewport",
            div { class: "modern-container",
                
                // HERO
                div { class: "balance-card",
                    div { style: "font-size: 0.8rem; color: var(--text-secondary); margin-bottom: 0.25rem; white-space: nowrap;",
                        "{asset_ticker} Portfolio"
                    }
                    div {
                        style: "font-size: 3rem; font-weight: 700; display: flex; align-items: baseline; white-space: nowrap;",
                        span { style: "font-size: 0.35em; color: var(--text-secondary); margin-right: 0.5rem;", "USD" }
                        span { class: "monospace-data", "{int_part}" }
                        span { class: "monospace-data", style: "font-size: 0.35em; color: var(--text-secondary);", "{frac_part}" }
                    }
                    div {
                        style: "color: var(--accent); font-size: 0.9rem; display: flex; align-items: center; gap: 8px; margin-top: 4px; white-space: nowrap;",
                        span { class: "monospace-data", "{formatted_raw_amount} {asset_ticker}" }
                        div { {logo} }
                    }
                }

                // GRID
                div { class: "dashboard-grid",
                    div { class: "column",
                        div { class: "content-box",
                            div { class: "section-label", "Financial Operations" }
                            div { class: "button-group", {send_btn}, {receive_btn} }
                        }
                        div { class: "content-box",
                            div { class: "section-label", "Vault Management" }
                            div { class: "button-group",
                                {purge_btn},
                                if let Some(btn) = delete_btn { {btn} }
                            }
                        }
                    }

                    div { class: "column",
                        if let LedgerInfo::Xrp(info) = ledger_info {
                            div { class: "content-box",
                                div { class: "section-label", "Ledger Data" }
                                div { class: "info-row",
                                    // Bulletproofing these spans with explicit nowrap
                                    span { style: "color: var(--text-secondary); white-space: nowrap;", "Available" }
                                    span { class: "monospace-data", "{info.available:.4} XRP" }
                                }
                                div { class: "info-row",
                                    span { style: "color: var(--text-secondary); white-space: nowrap;", "Reserve" }
                                    span { class: "monospace-data", "{info.total_reserve:.2} XRP" }
                                }
                                div { class: "info-row", style: "margin-top: 4px;",
                                    span { style: "color: var(--text-secondary); white-space: nowrap;", "Status" }
                                    span { style: "background-color: {xrp_status_bg}; {pill_style}", "{xrp_status_text}" }
                                }
                            }
                        }

                        div { class: "content-box",
                            div { class: "section-label", "System Diagnostics" }
                            div { style: "display: flex; flex-direction: column; gap: 12px;",
                                div {
                                    div { class: "diag-label", "Encryption" }
                                    div { class: "diag-value", style: "color: {status_color}", "{status_text}" }
                                }
                                div {
                                    div { class: "diag-label", "Node Connection" }
                                    span { style: "background-color: {node_bg}; {pill_style}", "{node_text}" }
                                }
                                div { style: "border-top: 1px solid var(--border); padding-top: 8px;",
                                    div { class: "diag-label", "Protocol" }
                                    div { class: "diag-value", style: "color: var(--accent)", "{network_protocol}" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}