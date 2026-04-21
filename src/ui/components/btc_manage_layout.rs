use dioxus_native::prelude::*;

#[component]
pub fn BtcManageLayout(
    has_wallet: bool,
    history_btn: Element,
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
                box-sizing: border-box;
            }
            .term-main { 
                flex: 1; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center; 
            }
            .term-sidebar-right { 
                position: absolute;
                right: 2rem;
                display: flex; 
                flex-direction: column; 
                gap: 1rem; 
                justify-content: center; 
                height: 100%;
                align-items: flex-end;
            }
            /* NEW CARD STYLES */
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
            .term-card {
                flex: 1;
                border: 1px solid var(--border); 
                padding: 1.5rem;
                display: flex;
                flex-direction: column;
                gap: 1.5rem;
                cursor: pointer;
                transition: all 0.2s ease;
            }
            .term-card:hover {
                border-color: var(--accent);
                background: var(--bg-faint);
            }
            .term-card-header {
                font-family: monospace;
                font-weight: bold;
                border-bottom: 1px dashed var(--border);
                padding-bottom: 0.5rem;
                white-space: nowrap;
            }
            .term-card-text {
                font-family: monospace;
                font-size: 0.9rem;
                line-height: 1.4;
                opacity: 0.7;
                width: 100%;
                flex: 1;
            }
            .term-card-footer {
                font-family: monospace;
                font-size: 0.8rem;
                color: var(--accent);
                opacity: 0.8;
            }
        "#} }

        div { class: "terminal-viewport",
            div { class: "term-main",
                if !has_wallet {
                    div { class: "setup-container",
                        div { 
                            style: "display: flex; flex-direction: column; align-items: center; font-family: monospace; opacity: 0.6; white-space: nowrap;",
                             div { "> CHECKING.BTC.FILE: NO_LOCAL_KEYPAIR_DETECTED" }
                             div { "> PLEASE_CREATE_OR_IMPORT_A_WALLET" }
                        }

                        div { class: "split-hero",
                            div { 
                                class: "term-card",
                                onclick: move |evt| on_create_click.call(evt),
                                div { class: "term-card-header", "> CREATE_BTC_WALLET" }
                                div { class: "term-card-text",
                                    "Generate a Bitcoin wallet. This creates a high-entropy 24-word mnemonic seed. The private keys are derived locally and never leave this machine's encrypted storage."
                                }
                                div { class: "term-card-footer", "[ START_CREATE_FLOW ]" }
                            }

                            div { 
                                class: "term-card",
                                onclick: move |evt| on_import_click.call(evt),
                                div { class: "term-card-header", "> IMPORT_BTC_WALLET" }
                                div { class: "term-card-text",
                                    "Recover an existing wallet using your 24-word recovery phrase. Note: For security and derivation path standard compliance, we strictly require 24 words for Bitcoin imports."
                                }
                                div { class: "term-card-footer", "[ START_IMPORT_FLOW ]" }
                            }
                        }
                    }
                } else {
                    {active_balance_view}
                }
            }

            if has_wallet {
                div { class: "term-sidebar-right",
                    {history_btn}
                }
            }
        }
    }
}