use crate::context::{
    BalanceContext, BtcContext, EuroContext, GlobalContext, RlusdContext, SgdContext, XrpContext,
};
use crate::channel::{BalanceActiveView, CHANNEL, Theme};
use crate::utils::add_commas;
use crate::bridge::json_storage;
use crate::utils::styles::terminal_action;
use dioxus_native::prelude::*;
use serde_json::Value;

pub mod change_pin;

#[component]
pub fn render_balance() -> Element {
    let bal_ctx = use_context::<BalanceContext>();
    let current_view = *bal_ctx.balance_view.read();

    // Now render_balance just handles the routing.
    // Each sub-view is responsible for its own full-width outer viewport.
    rsx! {
        match current_view {
            BalanceActiveView::ChangePin => rsx! { change_pin::view {} },
            _ => rsx! { BalanceDisplay {} }
        }
    }
}

#[component]
fn BalanceDisplay() -> Element {
    let global = use_context::<GlobalContext>();
    let xrp_ctx = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let btc_ctx = use_context::<BtcContext>();
    let sgd_ctx = use_context::<SgdContext>();

    let (theme, hide_balance) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);
    
    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = *rlusd_ctx.rlusd.read();
    let (euro_amount, _, _) = *euro_ctx.euro.read();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();
    let (sgd_amount, _, _) = *sgd_ctx.sgd.read();

    let rates = global.rates.read();
    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0).into();
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0).into();
    let eur_usd_rate: f64 = rates.get("EUR/USD").copied().unwrap_or(0.0).into();
    let sgd_usd_rate: f64 = rates.get("SGD/USD").copied().unwrap_or(0.0).into();

    let total_usd: f64 = (xrp_amount * xrp_usd_rate)
            + rlusd_amount
            + (euro_amount * eur_usd_rate)
            + (btc_amount * btc_usd_rate)
            + (sgd_amount * sgd_usd_rate);

    let (int_part, frac_part) = if hide_balance {
        ("••••".to_string(), "••".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64),
            format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64),
        )
    };

    let fmt_amt = |amt: f64, prec: usize| -> String {
        if hide_balance { "••••".to_string() } else { format!("{:.*}", prec, amt) }
    };

    rsx! {
        style { {r#"
            /* THE FIX: The outer viewport taking up full space and setting relative positioning */
            .balance-outer-viewport {
                display: flex;
                flex-direction: row;
                width: 100%;
                flex: 1;
                position: relative;
                align-items: center; 
            }
            /* The inner box handling the 900px constraint */
            .modern-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 900px;
                margin: 0 auto;
                font-family: 'Inter', sans-serif;
                gap: 1rem;
                justify-content: center;
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
            }
            .column {
                display: flex;
                flex-direction: column;
                gap: 1rem;
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
            .leader { 
                flex: 1; 
                border-bottom: 2px dotted var(--border); 
                margin: 0 0.75rem; 
                opacity: 0.3; 
                transform: translateY(-4px); 
            }
            .monospace-data { 
                font-family: 'JetBrains Mono', monospace; 
                white-space: nowrap; 
            }
            .item-name { font-size: 0.85rem; color: var(--text-secondary); white-space: nowrap; }
            .item-value { font-size: 0.9rem; color: var(--accent); white-space: nowrap; }
        "#} }

        // BOX WITHIN A BOX
        div { class: "balance-outer-viewport",
            div { class: "modern-container",
                
                // HERO CARD
                div { class: "balance-card",
                    div { style: "font-size: 0.8rem; color: var(--text-secondary); margin-bottom: 0.25rem; white-space: nowrap;",
                        "Portfolio Value"
                    }
                    div {
                        style: "font-size: 3rem; font-weight: 700; display: flex; align-items: baseline; white-space: nowrap;",
                        if !hide_balance { span { style: "font-size: 0.35em; color: var(--text-secondary); margin-right: 0.5rem;", "USD" } }
                        span { class: "monospace-data", "{int_part}" }
                        span { class: "monospace-data", style: "font-size: 0.35em; color: var(--text-secondary);", "{frac_part}" }
                    }
                }

                // GRID
                div { class: "dashboard-grid",
                    // LEFT COLUMN: ASSETS
                    div { class: "column",
                        div { class: "content-box",
                            div { class: "section-label", "Asset Balances" }
                            AssetRow { label: "XRP", value: fmt_amt(xrp_amount, 2) }
                            AssetRow { label: "BTC", value: fmt_amt(btc_amount, 5) }
                            AssetRow { label: "RLUSD", value: fmt_amt(rlusd_amount, 2) }
                            AssetRow { label: "EUR", value: fmt_amt(euro_amount, 2) }
                            AssetRow { label: "SGD", value: fmt_amt(sgd_amount, 2) }
                        }
                    }

                    // RIGHT COLUMN: SETTINGS
                    div { class: "column",
                        div { class: "content-box",
                            div { class: "section-label", "System Preferences" }
                            
                            div { class: "info-row",
                                span { class: "item-name", "Interface Theme" }
                                div { class: "leader" }
                                {terminal_action(if is_dark { "LIGHT" } else { "DARK" }, is_dark, move |_| {
                                    let new_theme = if is_dark { Theme::Light } else { Theme::Dark };
                                    let _ = json_storage::update_json::<Value>("settings.json", |json| {
                                        if let Some(obj) = json.as_object_mut() {
                                            obj.insert("theme".to_string(), serde_json::json!(new_theme));
                                        }
                                    });
                                    let _ = CHANNEL.theme_user_tx.send((new_theme, hide_balance));
                                })}
                            }

                            div { class: "info-row",
                                span { class: "item-name", "Balance Visibility" }
                                div { class: "leader" }
                                {terminal_action(if hide_balance { "REVEAL" } else { "HIDE" }, !hide_balance, move |_| {
                                    let new_hide = !hide_balance;
                                    let _ = json_storage::update_json::<Value>("settings.json", |json| {
                                        if let Some(obj) = json.as_object_mut() {
                                            obj.insert("is_hidden".to_string(), serde_json::json!(new_hide));
                                        }
                                    });
                                    let _ = CHANNEL.theme_user_tx.send((theme, new_hide));
                                })}
                            }

                            div { style: "border-top: 1px solid var(--border); margin-top: 8px; padding-top: 12px;",
                                div { class: "info-row",
                                    span { class: "item-name", "Security Access" }
                                    div { class: "leader" }
                                    {terminal_action("CHANGE PIN", true, move |_| {
                                        CHANNEL.balance_view_tx.send(BalanceActiveView::ChangePin).ok();
                                    })}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn AssetRow(label: String, value: String) -> Element {
    rsx! {
        div { class: "info-row",
            span { class: "item-name", "{label}" }
            div { class: "leader" }
            span { class: "item-value monospace-data", "{value}" }
        }
    }
}