use dioxus_native::prelude::*;
use crate::context::GlobalContext;

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

pub const MARKET_GROUPS: &[AssetGroup] = &[
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

#[component]
pub fn TickerFeed() -> Element {
    let global = use_context::<GlobalContext>();
    let is_connected = *global.exchange_ws_status.read();
    
    let status_text = if is_connected { "ONLINE" } else { "OFFLINE" };
    let status_color = if is_connected { "var(--status-ok)" } else { "var(--status-warn)" };

    rsx! {
        div { class: "dashboard-panel",
            div { class: "panel-header",
                span { "REALTIME_FEED" }
                span { style: "color: {status_color};", "{status_text}" }
            }
            
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