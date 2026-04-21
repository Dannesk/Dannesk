// src/ui/enterpin.rs
use crate::bridge::json_storage;
use dioxus_native::prelude::*;

#[derive(Clone, Copy, PartialEq)]
enum PinState {
    Enter,
    Set,
    Confirm,
}

#[component]
pub fn PinScreen(on_unlock: EventHandler<()>) -> Element {
    let mut input = use_signal(String::new);
    let mut error_msg = use_signal(|| None::<String>);
    let mut stored_pin_for_confirmation = use_signal(String::new);
    let mut attempts_left = use_signal(|| 5);
    let mut is_processing = use_signal(|| false);

    let pin_exists = use_memo(|| json_storage::read_json::<crate::pin::PinData>("pin.json").is_ok());

    let state = use_memo(move || {
        if !pin_exists() {
            if stored_pin_for_confirmation().is_empty() { PinState::Set } else { PinState::Confirm }
        } else {
            PinState::Enter
        }
    });

    let prompt = match state() {
        PinState::Set => "CREATE PIN:",
        PinState::Confirm => "CONFIRM PIN:",
        PinState::Enter => "ENTER PIN:",
    };

    let mut run_submit = move || {
        if *is_processing.peek() || input.peek().is_empty() { return; }
        is_processing.set(true);

        let pin = input.read().clone();
        let current_state = state();
        let stored_pin = stored_pin_for_confirmation.read().clone();

        spawn(async move {
            match current_state {
                PinState::Set => {
                    stored_pin_for_confirmation.set(pin);
                    input.set(String::new());
                    error_msg.set(None);
                }
                PinState::Confirm => {
                    if pin == stored_pin {
                        let _ = crate::pin::set_pin(&pin);
                        on_unlock.call(());
                    } else {
                        error_msg.set(Some("MISMATCH".to_string()));
                        stored_pin_for_confirmation.set(String::new());
                        input.set(String::new());
                    }
                }
                PinState::Enter => {
                    if crate::pin::verify_pin(&pin).is_ok() {
                        on_unlock.call(());
                    } else {
                        let left = *attempts_left.peek() - 1;
                        attempts_left.set(left);
                        input.set(String::new());
                        error_msg.set(Some(format!("DENIED: {} LEFT", left)));
                    }
                }
            }
            is_processing.set(false);
        });
    };

    rsx! {
        style { {r#"
            .pin-container {
                height: 100vh;
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                background: var(--bg-primary);
                font-family: 'JetBrains Mono', monospace;
                color: var(--text);
            }

            .input-row {
                display: flex;
                flex-direction: row;
                align-items: center;
                /* Increased gap to stop the visual bleed */
                gap: 2rem; 
            }

            .prompt {
                font-size: 0.9rem;
                color: var(--text-secondary);
                /* Fixed width ensures the input doesn't jump when text changes */
                width: 120px;
                text-align: right;
            }

            .pin-input {
                background: transparent;
                /* Removed border from the row, moved it here */
                border: none;
                border-bottom: 1px solid var(--border);
                outline: none;
                color: var(--text-accent);
                font-family: inherit;
                font-size: 1.2rem;
                letter-spacing: 0.5rem;
                width: 140px;
                padding-bottom: 4px;
            }

            .error {
                margin-top: 20px;
                font-size: 0.7rem;
                color: var(--status-warn);
                height: 1rem;
            }
        "#} }

        div { class: "pin-container",
            div { class: "input-row",
                span { class: "prompt", "{prompt}" }
                if *is_processing.read() {
                    span { class: "pin-input", "..." }
                } else {
                    input {
                        class: "pin-input",
                        r#type: "password",
                        autofocus: true,
                        value: "{input}",
                        oninput: move |evt| {
                            let val = evt.value();
                            if val.len() <= 6 && val.chars().all(|c| c.is_numeric()) {
                                input.set(val.clone());
                                if val.len() == 6 { run_submit(); }
                            }
                        }
                    }
                }
            }
            div { class: "error", "{error_msg.read().clone().unwrap_or_default()}" }
        }
    }
}