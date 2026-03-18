use crate::channel::SideBarView;
use crate::context::GlobalContext;
use crate::ui::pinlogic::PinLogic;
use crate::utils::styles::{previous_icon_button, terminal_action};
use dioxus_native::prelude::*;

#[component]
pub fn view() -> Element {
    let mut global = use_context::<GlobalContext>();

    let mut old_pin = use_signal(String::new);
    let mut new_pin = use_signal(String::new);
    let mut confirm_pin = use_signal(String::new);

    let mut on_submit = move |_| {
        if new_pin().is_empty() || old_pin().is_empty() {
            return;
        }
        if new_pin() != confirm_pin() {
            return;
        }

        tokio::spawn(PinLogic::change_pin(old_pin(), new_pin()));

        old_pin.set(String::new());
        new_pin.set(String::new());
        confirm_pin.set(String::new());

        global.sidebar_view.with_mut(|v| *v = SideBarView::None);
    };

    rsx! {
        style { {r#"
            .pin-outer-viewport {
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

            .pin-main-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                padding-left: 2rem;
                padding-right: 2rem;
                font-family: 'JetBrains Mono', monospace;
            }

            .pin-header {
                display: flex;
                justify-content: space-between;
                align-items: flex-end;
                border-bottom: 1px solid var(--border);
                padding-bottom: 0.5rem;
                margin-bottom: 3rem;
            }

            .pin-label {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 0.25rem;
                font-weight: 600;
                text-transform: uppercase;
                white-space: nowrap;
            }

            /* NEW: Flex row for side-by-side inputs and centering */
            .input-row {
                display: flex;
                flex-direction: row;
                justify-content: space-between;
                gap: 2rem;
                width: 100%;
                margin-bottom: 2rem;
                white-space: nowrap;

            }

            /* UPDATED: Allows inputs to fill available flex space but cap out at 350px */
            .input-section { 
                flex: 1;
                max-width: 350px;
                display: flex;
                flex-direction: column;
            }

            .input-label {
                font-size: 0.65rem;
                color: var(--accent);
                border-left: 2px solid var(--accent);
                padding-left: 8px;
                margin-bottom: 0.75rem;
                letter-spacing: 1px;
            }

            /* UPDATED: Width handles full capacity of the parent .input-section max-width */
            .terminal-input-wrapper {
                display: flex;
                align-items: center;
                background: var(--bg-grid);
                border: 1px solid var(--border);
                padding: 0.8rem 1rem;
                width: 100%;
            }

            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }

            .inner-input {
                flex: 1; background: transparent; border: none; outline: none;
                color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem;
            }

            /* UPDATED: Centered the execute button */
            .footer-actions {
                margin-top: 1rem;
                display: flex;
            }
        "#} }

        div { class: "pin-outer-viewport",

            // 1. PINNED TOP-LEFT
            div {
                class: "back-button-container",
                onclick: move |_| {
                    global.sidebar_view.with_mut(|v| *v = SideBarView::None);
                },
                previous_icon_button { text_color: "var(--text)".to_string() }
            }

            // 2. CENTERED CONTENT
            div { class: "pin-main-container",

                div { class: "pin-header",
                    div { class: "pin-label", "SECURITY // MANAGE_PIN" }
                }

                // First Row: Old PIN & New PIN side-by-side
                div { class: "input-row",
                    div { class: "input-section",
                        div { class: "input-label", "CURRENT_PIN" }
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", "[" }
                            input {
                                class: "inner-input",
                                r#type: "password",
                                value: "{old_pin}",
                                oninput: move |e| old_pin.set(e.value()),
                            }
                            span { class: "bracket", "]" }
                        }
                    }

                    div { class: "input-section",
                        div { class: "input-label", "NEW_PIN" }
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", "[" }
                            input {
                                class: "inner-input",
                                r#type: "password",
                                value: "{new_pin}",
                                oninput: move |e| new_pin.set(e.value()),
                            }
                            span { class: "bracket", "]" }
                        }
                    }
                }

                // Second Row: Confirm PIN underneath
                div { class: "input-row",
                    div { class: "input-section",
                        div { class: "input-label", "CONFIRM_NEW_PIN" }
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", "[" }
                            input {
                                class: "inner-input",
                                r#type: "password",
                                value: "{confirm_pin}",
                                oninput: move |e| confirm_pin.set(e.value()),
                                onkeydown: move |e| if e.key() == Key::Enter { on_submit(()); }
                            }
                            span { class: "bracket", "]" }
                        }
                    }
                }

                // Footer centered
                div { class: "footer-actions",
                    {terminal_action("EXECUTE", true, move |_| on_submit(()))}
                }
            }
        }
    }
}
