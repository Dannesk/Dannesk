use crate::bridge::json_storage;
use crate::channel::{BalanceActiveView, CHANNEL, Theme};
use crate::context::{GlobalContext};
use crate::utils::styles::{previous_icon_button, terminal_action};
use dioxus_native::prelude::*;
use serde_json::Value;

#[component]
pub fn view() -> Element {
    let global = use_context::<GlobalContext>();
    
    let (theme, hide_balance) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    rsx! {
        style { {r#"
            /* 1. Remove justify-content: center from here */
            .settings-viewport { 
                display: flex; 
                flex-direction: column; 
                width: 100%; 
                flex: 1; 
                position: relative; 
            }
            /* 2. New wrapper to handle centering the menu separately */
            .settings-content-wrapper {
                display: flex;
                flex-direction: column;
                justify-content: center;
                flex: 1;
                width: 100%;
            }
            .back-button-container {      
                position: absolute;
                top: 0.75rem;
                left: 0.75rem;
                cursor: pointer;
                z-index: 10; 
            }
            .settings-main { 
                display: flex; 
                flex-direction: column; 
                width: 100%; 
                max-width: 600px; 
                margin: 0 auto; 
                padding: 2rem; 
                font-family: 'JetBrains Mono', monospace; 
            }
            .settings-header { 
                font-size: 0.7rem; 
                color: var(--text_secondary); 
                letter-spacing: 0.25rem; 
                border-bottom: 1px solid var(--border); 
                padding-bottom: 0.5rem; 
                margin-bottom: 2.5rem; 
                text-transform: uppercase; 
            }
            .settings-row { 
                display: flex; 
                justify-content: space-between; 
                align-items: center; 
                padding: 1.25rem 0; 
                border-bottom: 1px solid rgba(255,255,255,0.05); 
            }
            .row-label { font-size: 0.8rem; color: var(--text); letter-spacing: 1px; }
        "#} }

        div { class: "settings-viewport",
            // Back button is now relative to the top of the viewport
            div {
                class: "back-button-container",
                onclick: move |_| {
                    CHANNEL.balance_view_tx.send(BalanceActiveView::Main).ok();
                },
                previous_icon_button { text_color: "var(--text-secondary)".to_string() }
            }

            // This wrapper pushes the content to the center without moving the back button
            div { class: "settings-content-wrapper",
                div { class: "settings-main",
                    div { class: "settings-header", "SYSTEM // PREFERENCES" }

                    // Theme Toggle
                    div { class: "settings-row",
                        span { class: "row-label", "INTERFACE_THEME" }
                        {terminal_action(
                            if is_dark { "LIGHT" } else { "DARK" }, 
                            is_dark, 
                            move |_| {
                                let new_theme = if is_dark { Theme::Light } else { Theme::Dark };
                                let _ = json_storage::update_json::<Value>("settings.json", |json| {
                                    if let Some(obj) = json.as_object_mut() {
                                        obj.insert("theme".to_string(), serde_json::json!(new_theme));
                                    }
                                });
                                let _ = CHANNEL.theme_user_tx.send((new_theme, hide_balance));
                            }
                        )}
                    }

                    // Privacy Toggle
                    div { class: "settings-row",
                        span { class: "row-label", "BALANCE_VISIBILITY" }
                        {terminal_action(
                            if hide_balance { "REVEAL" } else { "HIDE" }, 
                            !hide_balance, 
                            move |_| {
                                let new_hide = !hide_balance;
                                let _ = json_storage::update_json::<Value>("settings.json", |json| {
                                    if let Some(obj) = json.as_object_mut() {
                                        obj.insert("is_hidden".to_string(), serde_json::json!(new_hide));
                                    }
                                });
                                let _ = CHANNEL.theme_user_tx.send((theme, new_hide));
                            }
                        )}
                    }

                    div { class: "settings-row",
                        span { class: "row-label", "SECURITY_ACCESS" }
                        {terminal_action(
                            "CHANGE PIN", 
                            true, 
                            move |_| {
                                CHANNEL.balance_view_tx.send(BalanceActiveView::ChangePin).ok();
                            }
                        )}
                    }
                }
            }
        }
    }
}