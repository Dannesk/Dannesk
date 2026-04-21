use crate::context::{
    BalanceContext, BtcContext, EuroContext, GlobalContext, RlusdContext, SgdContext, XrpContext,
};
use crate::channel::{BalanceActiveView, CHANNEL};
use crate::utils::add_commas;
use crate::ui::{settings, change_pin}; 
use dioxus_native::prelude::*;

// --- TICKER CONFIGURATION ---
#[derive(Clone, Copy, PartialEq)]
pub struct AssetConfig {
    pub name: &'static str,
    pub symbol: &'static str,
    pub precision: usize,
}

pub struct AssetGroup {
    pub title: &'static str,
    pub assets: &'static [AssetConfig],
}

const MARKET_GROUPS: &[AssetGroup] = &[
    AssetGroup {
        title: "USD_MARKETS",
        assets: &[
            AssetConfig { name: "BTC/USD", symbol: "$", precision: 2 },
            AssetConfig { name: "XRP/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "EUR/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "SGD/USD", symbol: "$", precision: 4 },
        ],
    },
    AssetGroup {
        title: "EUR_MARKETS",
        assets: &[
            AssetConfig { name: "BTC/EUR", symbol: "€", precision: 2 },
            AssetConfig { name: "XRP/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "USD/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "SGD/EUR", symbol: "€", precision: 4 },
        ],
    },
    AssetGroup {
        title: "SGD_MARKETS",
        assets: &[
            AssetConfig { name: "BTC/SGD", symbol: "S$", precision: 2 },
            AssetConfig { name: "XRP/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "USD/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "EUR/SGD", symbol: "S$", precision: 4 },
        ],
    },
];

// --- MAIN BALANCE COMPONENT ---
#[component]
pub fn render_balance() -> Element {
    let bal_ctx = use_context::<BalanceContext>();
    let current_view = *bal_ctx.balance_view.read();

    rsx! {
        style { {r#"
            .balance-container {
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                width: 100%;
                font-family: 'JetBrains Mono', monospace;
                position: relative;
            }
            .settings-trigger {
                position: absolute;
                top: 1.25rem;
                right: 1.25rem;
                cursor: pointer;
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.1rem;
                z-index: 10;
            }
            
            /* Main Header Styles - Adjusted for split column */
            .total-amount {
                margin: 0;
                font-size: clamp(2.5rem, 6vw, 4.5rem);
                font-weight: 800;
                display: flex;
                align-items: baseline;
                line-height: 1;
                margin-bottom: 2rem;
            }
            .currency-symbol { font-size: 0.36em; color: var(--text-secondary); margin-right: 0.65rem; }
            .int-part { color: var(--text); }
            .frac-part { font-size: 0.36em; color: var(--text-secondary); margin-left: 6px; }

            /* Dashboard Split Styles */
            .dashboard-split {
                display: flex;
                flex-direction: row;
                width: 100%;
                max-width: 1200px;
                gap: 7rem;
            }
            .dashboard-panel {
                flex: 1;
                display: flex;
                flex-direction: column;
                gap: 1.5rem;
            }
            .panel-header {
                font-size: 0.8rem;
                color: var(--accent);
                letter-spacing: 0.1rem;
                font-weight: bold;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.75rem;
                display: flex;
                justify-content: space-between;
            }
            .panel-section {
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }
            .section-title {
                font-size: 0.75rem;
                color: var(--text-secondary);
                opacity: 0.5;
                letter-spacing: 0.05rem;
                margin-bottom: 0.25rem;
                margin-top: 0.5rem;
            }
            .list-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                width: 100%;
                padding: 0.25rem 0;
            }
            .item-name { font-size: 0.85rem; color: var(--text-secondary); white-space: nowrap; opacity: 0.8; }
            .leader { flex: 1; border-bottom: 2px dotted var(--border); margin: 0 0.75rem; opacity: 0.3; transform: translateY(-4px); }
            .item-value { font-size: 0.9rem; color: var(--accent); white-space: nowrap; }
        "#} }

        div { class: "balance-container",
            match current_view {
                BalanceActiveView::Main => rsx! {
                    div { 
                        class: "settings-trigger",
                        onclick: move |_| { CHANNEL.balance_view_tx.send(BalanceActiveView::Settings).ok(); },
                        "[ SETTINGS ]"
                    }
                    BalanceDisplay {}
                },
                BalanceActiveView::Settings => rsx! {
                    settings::view {}
                },
                BalanceActiveView::ChangePin => rsx! {
                    change_pin::view {}
                }
            }
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

    // Subscribe to balance updates
    let (xrp_amount, _, _) = xrp_ctx.wallet_balance.read().clone();
    let (rlusd_amount, _, _) = *rlusd_ctx.rlusd.read();
    let (euro_amount, _, _) = *euro_ctx.euro.read();
    let (btc_amount, _, _) = btc_ctx.bitcoin_wallet.read().clone();
    let (sgd_amount, _, _) = *sgd_ctx.sgd.read();

    // Subscribe to rate & status updates
    let rates = global.rates.read();
    let is_connected = *global.exchange_ws_status.read();
    let (_, hide_balance) = *global.theme_user.read();

    let xrp_usd_rate: f64 = rates.get("XRP/USD").copied().unwrap_or(0.0).into();
    let btc_usd_rate: f64 = rates.get("BTC/USD").copied().unwrap_or(0.0).into();
    let eur_usd_rate: f64 = rates.get("EUR/USD").copied().unwrap_or(0.0).into();
    let sgd_usd_rate: f64 = rates.get("SGD/USD").copied().unwrap_or(0.0).into();

    let total_usd: f64 = if hide_balance {
        0.0
    } else {
        (xrp_amount * xrp_usd_rate)
            + rlusd_amount
            + (euro_amount * eur_usd_rate)
            + (btc_amount * btc_usd_rate)
            + (sgd_amount * sgd_usd_rate)
    };

    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64),
            format!(".{:02}", (total_usd.fract() * 100.0).floor() as i64),
        )
    };

    // Helper macro for hiding individual balances
    let format_balance = |amt: f64, precision: usize| -> String {
        if hide_balance {
            "****".to_string()
        } else {
            format!("{:.*}", precision, amt)
        }
    };

    let status_text = if is_connected { "ONLINE" } else { "OFFLINE" };
    let status_color = if is_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    rsx! {
        div { class: "dashboard-split",
            
            // LEFT PANEL: Total Balance + Asset Balances
            div { class: "dashboard-panel",
                
                // TOP OF LEFT COLUMN: Main USD Value
                h1 { class: "total-amount",
                    if !hide_balance {
                        span { class: "currency-symbol", "$" }
                    }
                    span { class: "int-part", "{int_part}" }
                    if !hide_balance {
                        span { class: "frac-part", "{frac_part}" }
                    }
                }

                div { class: "panel-header",
                    span { "LOCAL_ASSETS" }
                    span { style: "color: var(--text-secondary); opacity: 0.5;", "TOTAL" }
                }
                div { class: "panel-section",
                    div { class: "list-row",
                        span { class: "item-name", "XRP" }
                        div { class: "leader" }
                        span { class: "item-value", "{format_balance(xrp_amount, 2)}" }
                    }
                    div { class: "list-row",
                        span { class: "item-name", "BTC" }
                        div { class: "leader" }
                        span { class: "item-value", "{format_balance(btc_amount, 5)}" }
                    }
                    div { class: "list-row",
                        span { class: "item-name", "RLUSD" }
                        div { class: "leader" }
                        span { class: "item-value", "{format_balance(rlusd_amount, 2)}" }
                    }
                    div { class: "list-row",
                        span { class: "item-name", "EUR" }
                        div { class: "leader" }
                        span { class: "item-value", "{format_balance(euro_amount, 2)}" }
                    }
                    div { class: "list-row",
                        span { class: "item-name", "SGD" }
                        div { class: "leader" }
                        span { class: "item-value", "{format_balance(sgd_amount, 2)}" }
                    }
                }
            }

            // RIGHT PANEL: Ticker Feed
            div { class: "dashboard-panel",
                div { class: "panel-header",
                    span { "REALTIME_FEED" }
                    span { style: "color: {status_color};", "{status_text}" }
                }
                
                // Stack market groups vertically
                for group in MARKET_GROUPS {
                    div { class: "panel-section",
                        div { class: "section-title", ":: {group.title}" }
                        for asset in group.assets {
                            MarketRow { asset: *asset }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn MarketRow(asset: AssetConfig) -> Element {
    let global = use_context::<GlobalContext>();

    let formatted_price = use_memo(move || {
        let rates = global.rates.read();
        let rate = rates.get(asset.name).copied().unwrap_or(0.0);

        if rate > 0.0 {
            format!("{:.precision$}", rate, precision = asset.precision)
        } else {
            "0.0000".to_string()
        }
    });

    rsx! {
        div { class: "list-row",
            span { class: "item-name", "{asset.name}" }
            div { class: "leader" }
            span { class: "item-value", "[ {asset.symbol} {formatted_price} ]" }
        }
    }
}