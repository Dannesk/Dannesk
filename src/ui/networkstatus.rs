use crate::channel::SideBarView;
use crate::context::GlobalContext;
use crate::utils::styles::previous_icon_button;
use dioxus_native::prelude::*;

#[component]
pub fn view() -> Element {
    let mut global = use_context::<GlobalContext>();

    let crypto_connected = *global.crypto_ws_status.read();
    let exchange_connected = *global.exchange_ws_status.read();

    let crypto_text = if crypto_connected {
        "CONNECTED"
    } else {
        "DISCONNECTED"
    };
    let crypto_color = if crypto_connected {
        "var(--status-ok)"
    } else {
        "var(--status-warn)"
    };

    let exchange_text = if exchange_connected {
        "CONNECTED"
    } else {
        "DISCONNECTED"
    };
    let exchange_color = if exchange_connected {
        "var(--status-ok)"
    } else {
        "var(--status-warn)"
    };

    let on_back_click = move |_| {
        global.sidebar_view.with_mut(|v| *v = SideBarView::None);
    };

    rsx! {
        style { {r#"
            .network-outer-viewport {
                display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                align-items: center;
            }

            .back-button-container {
                position: absolute;
                top: 1.25rem;
                left: 1.25rem;
                cursor: pointer;
                z-index: 100;
            }

            .network-main-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 500px;
                margin: 0 auto;
                padding: 0 2rem;
                font-family: 'JetBrains Mono', monospace;
            }

            .network-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-end;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 2rem;
            }

            .network-label {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.25rem;
                white-space: nowrap;
            }

            .status-stack {
                display: flex;
                flex-direction: column;
                gap: 1.5rem;
                width: 100%;
            }

            .status-group {
                display: flex;
                flex-direction: column;
                gap: 0.5rem;
            }

            .group-label {
font-size: 0.65rem; color: var(--text); border-left: 2px solid var(--accent); padding-left: 8px; letter-spacing: 1px; margin-bottom: 0.75rem; white-space: nowrap;            }

            .status-row {
                background: var(--bg-secondary);
                border: 1px solid var(--border);
                border-radius: 4px;
                padding: 0.85rem 1rem;
                display: flex;
                justify-content: space-between;
                align-items: center;
                white-space: nowrap;
            }

            .status-value {
                font-size: 0.65rem;
                letter-spacing: 0.5px;
            }

            .status-subtext {
                font-size: 0.65rem;
                color: var(--text-secondary);
                opacity: 0.6;
            }
        "#} }

        div { class: "network-outer-viewport",
            div {
                class: "back-button-container",
                onclick: on_back_click,
                previous_icon_button { text_color: "var(--text)".to_string() }
            }

            div { class: "network-main-container",

                div { class: "network-header",
                    div { class: "network-label", "SERVER_HEALTH" }
                }

                div { class: "status-stack",

                    // CRYPTO SECTION
                    div { class: "status-group",
                        div { class: "group-label", "NODES" }
                        div { class: "status-row",
                            div {
                                class: "status-value",
                                style: "color: {crypto_color}",
                                "{crypto_text}"
                            }
                            div { class: "status-subtext", "XRPL / BTC / MAINNET" }
                        }
                    }

                    // EXCHANGE SECTION
                    div { class: "status-group",
                        div { class: "group-label", "EXCHANGE_RATES" }
                        div { class: "status-row",
                            div {
                                class: "status-value",
                                style: "color: {exchange_color}",
                                "{exchange_text}"
                            }
                            div { class: "status-subtext", "BINANCE / UPBIT" }
                        }
                    }
                }
            }
        }
    }
}
