// src/utils/enable_layout.rs
use crate::utils::styles::terminal_action;
use dioxus_native::prelude::*;

#[component]
pub fn render_token_enable(
    symbol: String,
    reserve_info: String,
    enable_btn_text: String,
    has_token: bool,
    input_mode: Signal<String>,
    passphrase_val: Signal<String>,
    bip39_val: Signal<String>,
    mut seed_words: Signal<Vec<String>>,
    error_msg: Signal<Option<String>>,
    on_enable: EventHandler<MouseEvent>,

    children: Element,
) -> Element {
    // --- LOGIC SECTION (Matching SendAuthForm) ---

    let mut display_text = use_signal(|| {
        let current = seed_words.read();
        let mut out = String::new();
        for (i, word) in current.iter().enumerate() {
            if word.is_empty() { continue; }
            out.push_str(word);
            // Add newline after 12th word, otherwise a space
            if (i + 1) == 12 { out.push('\n'); } else { out.push(' '); }
        }
        out.trim().to_string()
    });

    let word_count = use_memo(move || {
        seed_words.read().iter().filter(|w| !w.is_empty()).count()
    });

    let handle_seed_input = move |evt: FormEvent| {
        let raw = evt.value();
        error_msg.set(None); // Clear error on interaction

        let words: Vec<String> = raw
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        // Update the underlying signal vector
        let mut new_vec = vec![String::new(); 24];
        for (i, word) in words.iter().enumerate() {
            if i < 24 { new_vec[i] = word.clone(); }
        }
        seed_words.set(new_vec);

        let current_len = display_text.read().len();
        let new_len = raw.len();

        // If significantly more text is added (a paste), re-format with the 12-word wrap
        if new_len > current_len + 10 {
            let mut wrapped = String::new();
            for (i, word) in words.iter().enumerate() {
                wrapped.push_str(word);
                if (i + 1) == 12 { wrapped.push('\n'); } else { wrapped.push(' '); }
            }
            display_text.set(wrapped.trim_end().to_string());
        } else {
            // Keep raw input for standard typing to preserve cursor position
            display_text.set(raw);
        }
    };

    rsx! {
        style { {r#"
            .terminal-container { display: flex; flex-direction: column; width: 100%; max-width: 800px; margin: 0 auto; font-family: 'JetBrains Mono', monospace; padding: 2rem; }
            .step-header { border-bottom: 1px solid var(--border); padding-bottom: 1rem; margin-bottom: 2rem; }
            .step-title { font-size: 0.7rem; color: var(--text-secondary); letter-spacing: 2px; }
            
            .auth-tabs { display: flex; gap: 2rem; margin-bottom: 2rem; border-bottom: 1px solid var(--border); }
            .auth-tab { padding: 0.5rem 0; font-size: 0.7rem; background: transparent; border: none; cursor: pointer; color: var(--text-secondary); position: relative; }
            .auth-tab-active { color: var(--accent); border-bottom: 2px solid var(--accent); }

            .input-section { margin-bottom: 1.5rem; }
            .input-label-row { display: flex; align-items: baseline; margin-bottom: 0.75rem; gap: 1rem; }
            .input-label { font-size: 0.65rem; color: var(--accent); border-left: 2px solid var(--accent); padding-left: 8px; }
            .input-hint { font-size: 0.6rem; color: var(--text-secondary); opacity: 0.6; }
            
            .terminal-input-wrapper { display: flex; align-items: center; background: var(--bg-grid); border: 1px solid var(--border); padding: 0.8rem 1rem; box-sizing: border-box; }
            .bracket { color: var(--text-secondary); opacity: 0.4; font-weight: bold; }
            .inner-input { flex: 1; background: transparent; border: none; outline: none; color: var(--text); font-family: inherit; font-size: 1rem; padding: 0 1rem; }

            .native-textarea { 
                display: block;
                width: 100%; 
                min-height: 120px; 
                background: var(--bg-grid); 
                border: 1px solid var(--border); 
                outline: none; 
                color: var(--text); 
                font-family: 'JetBrains Mono', monospace; 
                font-size: 1rem; 
                line-height: 1.4; 
                padding: 1rem; 
                resize: none; 
                box-sizing: border-box; 
            }

            .counter-row { display: flex; justify-content: flex-end; margin-top: 0.5rem; }
            .word-count { font-size: 0.65rem; color: var(--text-secondary); }
            .complete { color: var(--accent); font-weight: bold; }

            .error-box { background: rgba(239, 68, 68, 0.1); border-left: 3px solid var(--status-warn); padding: 0.75rem 1rem; margin-top: 1rem; font-size: 0.75rem; color: var(--status-warn); }
            .footer-nav { margin-top: 2rem; display: flex; justify-content: space-between; align-items: center; }
        "#} }

        div { class: "terminal-container",
            if !has_token {
                div { class: "step-header",
                    div { class: "step-title", "TRUSTLINE_AUTH // XRP_MAINNET // {symbol} " }
                    div { style: "font-size: 0.6rem; color: #888; margin-top: 0.5rem;", "{reserve_info}" }
                }

                div { class: "auth-tabs",
                    button {
                        class: if input_mode() == "passphrase" { "auth-tab auth-tab-active" } else { "auth-tab" },
                        onclick: move |_| {
                            input_mode.set("passphrase".to_string());
                            error_msg.set(None);
                        },
                        "DECRYPTION_PASSPHRASE"
                    }
                    button {
                        class: if input_mode() == "seed" { "auth-tab auth-tab-active" } else { "auth-tab" },
                        onclick: move |_| {
                            input_mode.set("seed".to_string());
                            error_msg.set(None);
                        },
                        "MNEMONIC_SEED"
                    }
                }

                if input_mode() == "passphrase" {
                    div { class: "input-section",
                        div { class: "input-label-row", div { class: "input-label", "ENCRYPTION_KEY" } }
                        div { class: "terminal-input-wrapper",
                            span { class: "bracket", "[" }
                            input {
                                class: "inner-input",
                                r#type: "password",
                                value: "{passphrase_val()}",
                                oninput: move |e| passphrase_val.set(e.value())
                            }
                            span { class: "bracket", "]" }
                        }
                    }
                } else {
                    div { class: "input-section",
                        div { class: "input-label-row",
                            div { class: "input-label", "MNEMONIC_SEED" }
                            div { class: "input-hint", "[PASTE_ALLOWED]" }
                        }
                        textarea {
                            class: "native-textarea",
                            value: "{display_text}",
                            placeholder: "ENTER_24_WORDS...",
                            oninput: handle_seed_input
                        }
                        div { class: "counter-row",
                            span { 
                                class: if word_count() == 24 { "word-count complete" } else { "word-count" },
                                "{word_count()} / 24 WORDS"
                            }
                        }
                    }
                }

                div { class: "input-section",
                    div { class: "input-label-row",
                        div { class: "input-label", "BIP39_PASSPHRASE" }
                        div { class: "input-hint", "[OPTIONAL]" }
                    }
                    div { class: "terminal-input-wrapper",
                        span { class: "bracket", "[" }
                        input {
                            class: "inner-input",
                            r#type: "password",
                            value: "{bip39_val()}",
                            oninput: move |e| bip39_val.set(e.value())
                        }
                        span { class: "bracket", "]" }
                    }
                }

                if let Some(err) = error_msg() {
                    div { class: "error-box", "SIGNAL_INTERRUPT: {err}" }
                }

                div { class: "footer-nav",
                    {terminal_action(&enable_btn_text, true, move |ev| on_enable.call(ev))}
                }
            } else {
                {children}
            }
        }
    }
}