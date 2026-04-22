use dioxus_native::prelude::*;

#[component]
pub fn XrpManageLayout(
    has_wallet: bool,
    nav_xrp: Element,
    nav_usd: Element,
    nav_eur: Element,
    nav_sgd: Element,
    trade_btn: Element,
    history_btn: Option<Element>, 
    on_create_click: EventHandler<MouseEvent>,
    on_import_click: EventHandler<MouseEvent>,
    active_balance_view: Element,
) -> Element {
    rsx! {
        style { {r#"
            .terminal-viewport { 
                display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                justify-content: center; 
                padding: 0 2rem;
            }
            .term-sidebar { 
                display: flex; 
                flex-direction: column; 
                gap: 1rem; 
                justify-content: center; 
            }
            .term-main { 
                flex: 1; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center; 
            }
            .setup-container {
                width: 800px;
                display: flex;
                flex-direction: column;
                align-items: center;
                gap: 2rem;
            }
            .split-hero {
                display: flex;
                flex-direction: row;
                width: 100%;
                gap: 2rem;
            }

            /* NEW DIAGNOSTIC MODULE CARDS */
            .module-box {
                flex: 1;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 1.5rem;
                display: flex;
                flex-direction: column;
                gap: 1.25rem;
                cursor: pointer;
                transition: all 0.2s ease;
            }
            .module-box:hover {
                border-color: var(--accent);
            }
                  .section-label {
                font-family: 'JetBrains Mono', monospace;
                font-size: 0.65rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
                border-left: 2px solid var(--accent);
                padding-left: 8px;
            }
            .module-desc {
                font-family: 'JetBrains Mono', monospace;
                font-size: 0.8rem;
                color: var(--text-secondary);
                line-height: 1.5;
                flex: 1;
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
                font-size: 0.75rem;
                font-weight: bold;
                color: var(--text);
            }
            
        "#} }

        div { class: "terminal-viewport",
            div { class: "term-sidebar",
                if has_wallet {
                    {nav_xrp}
                    {nav_usd}
                    {nav_eur}
                    {nav_sgd}
                }
            }

            div { class: "term-main",
                if !has_wallet {
                    div { class: "setup-container",
                        div { 
                            style: "display: flex; flex-direction: column; align-items: center; font-family: 'JetBrains Mono', monospace; opacity: 0.6; white-space: nowrap;",
                             div { "> LOCAL.XRP.FILE: NOT_DETECTED" }
                             div { "> PLEASE_IMPORT_OR_CREATE_A_WALLET" }
                        }

                        div { class: "split-hero",
                            // PROTOCOL 01: CREATE
                            div { 
                                class: "module-box",
                                onclick: move |evt| on_create_click.call(evt),
                                div { class: "section-label", "CREATE_XRP_WALLET" }
                                div { class: "module-desc",
                                    "Generate a 24 word Mnemonic to create a new wallet. "
                                }
                                div { 
                                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; padding-top: 1rem; border-top: 1px solid var(--border);",
                                    div { class: "diag-row",
                                        div { class: "diag-label", "CURVE" },
                                        div { class: "diag-value", "ED25519" }
                                    }
                                     div { class: "diag-row",
                                        div { class: "diag-label", "DERIVATION_PATH" },
                                        div { class: "diag-value",  "m/44'/144'/0'/0/0" }
                                    }
                                    div { class: "diag-row",
                                        div { class: "diag-label", "NETWORK" },
                                        div { class: "diag-value",  "MAINNET/XRP" }
                                    }
                                }
                            }

                            // PROTOCOL 02: IMPORT
                            div { 
                                class: "module-box",
                                onclick: move |evt| on_import_click.call(evt),
                                div { class: "section-label", "IMPORT_XRP_WALLET" }
                                div { class: "module-desc",
                                    "Import an existing wallet using your 24-word mnemonic."
                                }
                                div { 
                                    style: "display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; padding-top: 1rem; border-top: 1px solid var(--border);",
                                    div { class: "diag-row",
                                        div { class: "diag-label", "REQUIREMENT" },
                                        div { class: "diag-value", "24_WORDS" }
                                    }
                                     div { class: "diag-row",
                                        div { class: "diag-label", "OPTIONAL" },
                                        div { class: "diag-value", "BIP39 (25th Word)" }
                                    }
                                    div { class: "diag-row",
                                        div { class: "diag-label", "NETWORK" },
                                        div { class: "diag-value", "MAINNET/XRP" }
                                    }
                                }
                            }
                        }
                    }
                } else {
                    {active_balance_view}
                }
            }

            div { class: "term-sidebar",
                style: "align-items: flex-end;",
                if has_wallet {
                    {trade_btn}
                    if let Some(btn) = history_btn {
                        {btn}
                    }
                }
            }
        }
    }
}