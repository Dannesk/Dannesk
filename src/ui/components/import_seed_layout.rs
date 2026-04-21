use crate::utils::styles::terminal_action;
use dioxus_native::prelude::*;

#[component]
pub fn ImportSeedForm(
    network_label: String,
    mut seed_words: Signal<Vec<String>>,
    mut error_msg: Signal<Option<String>>,
    on_continue: EventHandler<MouseEvent>,
) -> Element {
    // This local signal holds the actual string in the textarea (UI Truth)
    let mut display_text = use_signal(|| {
        let current = seed_words.read();
        let mut out = String::new();
        for (i, word) in current.iter().enumerate() {
            if word.is_empty() { continue; }
            out.push_str(word);
            if (i + 1) == 12 { out.push('\n'); } else { out.push(' '); }
        }
        out.trim().to_string()
    });

    let word_count = use_memo(move || {
        seed_words.read().iter().filter(|w| !w.is_empty()).count()
    });

    // logic outside rsx: handle input
    let handle_input = move |evt: FormEvent| {
        let raw = evt.value();
        error_msg.set(None);

        // 1. Sync the backend Vec<String> logic
        let words: Vec<String> = raw
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        let mut new_vec = vec![String::new(); 24];
        for (i, word) in words.iter().enumerate() {
            if i < 24 { new_vec[i] = word.clone(); }
        }
        seed_words.set(new_vec);

        // 2. PASTE VS TYPE LOGIC
        // If the user pastes a large block, we enforce the 12-word wrap
        // If they are just typing (incrementing by 1-2 chars), we preserve their exact input (spaces/returns)
        let current_len = display_text.read().len();
        let new_len = raw.len();

        if new_len > current_len + 10 {
            // Likely a PASTE: Reconstruct with hard wrap at 12
            let mut wrapped = String::new();
            for (i, word) in words.iter().enumerate() {
                wrapped.push_str(word);
                if (i + 1) == 12 {
                    wrapped.push('\n');
                } else {
                    wrapped.push(' ');
                }
            }
            display_text.set(wrapped.trim_end().to_string());
        } else {
            // Likely TYPING: Keep the string exactly as the user inputs it (respects Space and Return)
            display_text.set(raw);
        }
    };

    rsx! {
        style { {r#"
            .import-step-container {
                display: flex;
                flex-direction: column;
                width: 100%;
                max-width: 800px;
                margin: 0 auto;
                font-family: 'JetBrains Mono', monospace;
                padding: 2rem;
            }
            .step-header {
                border-bottom: 1px solid var(--border);
                padding-bottom: 1rem;
                margin-bottom: 1.5rem;
            }
            .step-title {
                font-size: 0.7rem;
                color: var(--text-secondary);
                letter-spacing: 2px;
            }
            .terminal-wrapper {
                display: grid;
                grid-template-columns: 1fr;
                background: var(--input-bg);
                border: 1px solid var(--border);
                padding: 1.5rem;
            }
            .native-textarea {
                width: 100%;
                min-height: 160px;
                background: transparent;
                border: none;
                outline: none;
                color: var(--text);
                font-family: 'JetBrains Mono', monospace;
                font-size: 1rem;
                line-height: 1.3; 
                padding: 0.5rem;
                resize: none;
            }
            .counter-row {
                display: flex;
                justify-content: flex-end;
                margin-top: 0.5rem;
            }
            .word-count {
                font-size: 0.65rem;
                color: var(--text-secondary);
            }
            .complete { color: var(--text-accent); font-weight: bold; }
            .error-banner {
                background: rgba(var(--status-warn-rgb), 0.1);
                color: var(--status-warn);
                border-left: 3px solid var(--status-warn);
                padding: 0.75rem 1rem;
                margin-top: 1.5rem;
                font-size: 0.7rem;
            }
            .footer-meta {
                margin-top: 2rem; 
                display: flex; 
                justify-content: flex-end; 
                align-items: center; 
                gap: 2rem; 
            }
        "#} }

        div { class: "import-step-container",
            div { class: "step-header",
                div { class: "step-title", "WALLET_IMPORT // MNEMONIC_ENTRY // {network_label}" }
            }

            div { class: "terminal-wrapper",
                textarea {
                    class: "native-textarea",
                    value: "{display_text}",
                    placeholder: "PASTE_OR_TYPE_24_WORDS...",
                    autofocus: true,
                    oninput: handle_input
                }
            }

            div { class: "counter-row",
                span { 
                    class: if word_count() == 24 { "word-count complete" } else { "word-count" },
                    "{word_count()} / 24 WORDS"
                }
            }

            if let Some(err) = error_msg() {
                div { class: "error-banner", ">> {err}" }
            }

            div { class: "footer-meta",
                {terminal_action("VERIFY_STRUCTURE", true, move |e| on_continue.call(e))}
            }
        }
    }
}