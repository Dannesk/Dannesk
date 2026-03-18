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
    rsx! {
        style { {r#"
            .manage-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
            }
            .balance-hero {
                padding: 2rem 0;
                border-bottom: 1px solid var(--border);
                margin-bottom: 2rem;
            }
            .action-grid {
                display: grid;
                grid-template-columns: 1fr 1fr;
                gap: 2rem;
                margin-bottom: 2rem;
            }
            .action-section {
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }
            .section-label {
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 0.5rem;
            }
            .info-box {
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 1rem;
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }
            .info-row {
                display: flex;
                justify-content: space-between;
                font-size: 0.75rem;
            }
            .button-group {
                display: flex;
                gap: 10px;
                flex-wrap: wrap;
            }
            .system-footer {
                margin-top: 2rem;
                padding: 1.5rem;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                display: flex;
                flex-direction: column;
                gap: 1rem;
            }
            .diag-row {
                display: flex;
                flex-direction: column;
                gap: 4px;
            }
            .diag-label {
                font-size: 0.6rem;
                color: var(--text-secondary);
                opacity: 0.7;
                letter-spacing: 1px;
            }
            .diag-value {
                font-size: 0.8rem;
                font-weight: bold;
                letter-spacing: 1px;
            }
        "#} }

        div { class: "manage-container",
            // 1. HERO BALANCE
            div { class: "balance-hero",
                div {
                    style: "font-size: 0.7rem; color: var(--text-secondary); opacity: 0.6; margin-bottom: 0.5rem;",
                    "ASSET_VALUATION // {asset_ticker}"
                }
                div {
                    style: "font-size: 3.5rem; font-weight: 800; display: flex; align-items: baseline;",
                    span { style: "font-size: 0.4em; color: var(--text-secondary); margin-right: 0.5rem;", "USD" }
                    span { "{int_part}" }
                    span { style: "font-size: 0.4em; color: var(--text-secondary);", "{frac_part}" }
                }
                div {
                    style: "color: var(--accent); font-size: 0.9rem; display: flex; align-items: center; gap: 8px;",
                    span { "{formatted_raw_amount} {asset_ticker}" }
                    div { {logo} }
                }
            }

            // 2. ACTION GRID (Operations Side-by-Side)
            div { class: "action-grid",
                div { class: "action-section",
                    div { class: "section-label", "FINANCIAL_OPERATIONS" }
                    div { class: "button-group", {send_btn}, {receive_btn} }
                }
                div { class: "action-section",
                    div { class: "section-label", "VAULT_MANAGEMENT" }
                    div { class: "button-group",
                        {purge_btn},
                        if let Some(btn) = delete_btn { {btn} }
                    }
                }
            }

            // 3. LEDGER CONSTRAINTS
            div { class: "action-section",
            if let LedgerInfo::Xrp(info) = ledger_info {
    div { class: "action-section",
        div { class: "section-label", "LEDGER_CONSTRAINTS" }
        div { class: "info-box",
            div { class: "info-row",
                span { style: "color: var(--text-secondary);", "AVAILABLE:" }
                span { style: "color: var(--accent); font-weight: bold;", "{info.available:.6} XRP" }
            }
            div { class: "info-row",
                span { style: "color: var(--text-secondary);", "RESERVE:" }
                span { style: "color: var(--text); font-weight: bold;", "{info.total_reserve:.2} XRP" }
            }
            div { class: "info-row",
                span { style: "color: var(--text-secondary);", "STATUS:" }
                if info.is_active {
                    span { style: "color: var(--status-ok); font-weight: bold;", "ACTIVE" }
                } else {
                    span { style: "color: var(--status-warn); font-weight: bold;", "INACTIVE" }
                }
            }
        }
    }
}
            }

            // 4. SYSTEM STATUS FOOTER
            div { class: "system-footer",
                div { class: "diag-row",
                    div { class: "diag-label", "SYSTEM_ENCRYPTION_STATUS" }
                    div {
                        class: "diag-value",
                        style: "color: {status_color}",
                        "{status_text}"
                    }
                }
                div {
                    style: "border-top: 1px solid var(--border); padding-top: 10px;",
                    class: "diag-row",
                    div { class: "diag-label", "NETWORK_PROTOCOL" }
                    div { class: "diag-value", style: "color: var(--accent)", "{network_protocol}" }
                }
            }
        }
    }
}
