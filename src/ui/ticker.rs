use crate::context::GlobalContext;
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
        title: "USD MARKETS",
        assets: &[
            AssetConfig { name: "BTC/USD", symbol: "$", precision: 2 },
            AssetConfig { name: "XRP/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "EUR/USD", symbol: "$", precision: 4 },
            AssetConfig { name: "SGD/USD", symbol: "$", precision: 4 },
        ],
    },
    AssetGroup {
        title: "EUR MARKETS",
        assets: &[
            AssetConfig { name: "BTC/EUR", symbol: "€", precision: 2 },
            AssetConfig { name: "XRP/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "USD/EUR", symbol: "€", precision: 4 },
            AssetConfig { name: "SGD/EUR", symbol: "€", precision: 4 },
        ],
    },
    AssetGroup {
        title: "SGD MARKETS",
        assets: &[
            AssetConfig { name: "BTC/SGD", symbol: "S$", precision: 2 },
            AssetConfig { name: "XRP/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "USD/SGD", symbol: "S$", precision: 4 },
            AssetConfig { name: "EUR/SGD", symbol: "S$", precision: 4 },
        ],
    },
];

#[component]
pub fn render_ticker() -> Element {
    let global = use_context::<GlobalContext>();
    let is_connected = *global.exchange_ws_status.read();
    
    let status_text = if is_connected { "ONLINE" } else { "OFFLINE" };
    let status_bg = if is_connected { "var(--brand-blue)" } else { "var(--status-warn)" };

    rsx! {
        style { {r#"
            .ticker-outer-viewport {
                display: flex;
                flex-direction: row;
                width: 100%;
                flex: 1;
                position: relative;
                align-items: center; 
            }
            .modern-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 900px;
                margin: 0 auto;
                font-family: 'Inter', sans-serif;
                gap: 1.5rem;
                justify-content: center;
            }
            .header-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 0 0.5rem;
            }
            .feed-label {
                font-size: 0.75rem;
                color: var(--text-secondary);
                font-weight: 700;
                text-transform: uppercase;
                letter-spacing: 0.5px;
                white-space: nowrap;
            }
            .three-column-grid {
                display: grid;
                grid-template-columns: 1fr 1fr 1fr;
                gap: 1rem;
                width: 100%;
            }
            .market-category {
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
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
                margin-bottom: 0.5rem;
                white-space: nowrap;
            }
            .info-row {
                display: flex;
                justify-content: space-between;
                align-items: center;
                font-size: 0.85rem;
                white-space: nowrap;
            }
            .item-name { 
                font-size: 0.85rem; 
                color: var(--text-secondary); 
                white-space: nowrap; 
            }
            .item-value { 
                font-size: 0.9rem; 
                color: var(--accent); 
                font-family: 'JetBrains Mono', monospace;
                white-space: nowrap; 
            }
            .status-indicator {
                background-color: var(--bg-faint);
                border: 1px solid var(--border);
                padding: 4px 10px;
                border-radius: 9999px;
                display: flex;
                align-items: center;
                gap: 6px;
            }
            .dot {
                width: 6px;
                height: 6px;
                border-radius: 50%;
            }
        "#} }

        div { class: "ticker-outer-viewport",
            div { class: "modern-container",
                
                // HEADER
                div { class: "header-row",
                    span { class: "feed-label", "LIVE FEED" }
                    div { class: "status-indicator",
                        div { class: "dot", style: "background: {status_bg};" }
                        span { 
                            style: "font-size: 0.65rem; font-weight: 700; color: var(--text); white-space: nowrap;", 
                            "{status_text}" 
                        }
                    }
                }

                // GRID
                div { class: "three-column-grid",
                    for group in MARKET_GROUPS {
                        div { class: "market-category",
                            div { class: "section-label", "{group.title}" }
                            for asset in group.assets {
                                MarketRow { asset: *asset }
                            }
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
        div { class: "info-row",
            span { class: "item-name", "{asset.name}" }
            span { class: "item-value", "{asset.symbol}{formatted_price}" }
        }
    }
}