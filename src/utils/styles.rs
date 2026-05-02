use dioxus_native::prelude::*;

// 1. The exact helper from your sidebar
fn base_button_style() -> String {
    "background: transparent; border: none; cursor: pointer; padding: 6px; display: flex; align-items: center;".to_string()
}

// 2. The exact indicator from your sidebar
#[component]
fn CliIndicator(label: String, is_active: bool) -> Element {
    let bracket_color = "var(--text-secondary)";
    let symbol = if is_active { ":" } else { "." };
    let symbol_color = if is_active {
        "var(--accent)"
    } else {
        "var(--text-secondary)"
    };

    rsx! {
        span {
            style: "font-family: 'JetBrains Mono', monospace; font-size: 0.75rem; letter-spacing: 1px;",
            span { style: "color: {bracket_color}; opacity: 0.4;", "[" }
            span { style: "color: {symbol_color};", "{symbol}" }
            span { style: "color: var(--text);  padding: 0 4px;", "{label}" }
            span { style: "color: {symbol_color};", "{symbol}" }
            span { style: "color: {bracket_color}; opacity: 0.4;", "]" }
        }
    }
}

// 3. The merged component using the exact same style function
#[component]
pub fn previous_icon_button(text_color: String) -> Element {
    rsx! {
        button {
            // Uses the sidebar style string: no border, 6px padding
            style: base_button_style(),
            CliIndicator {
                label: "<< BACK".to_string(),
                is_active: true
            }
        }
    }
}

pub fn terminal_action(
    label: &str,
    active: bool,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> Element {
    let button_bg = if active {
        "var(--brand-blue)"
    } else {
        "var(--bg-grid)" 
    };
    let button_text = if active {
        "var(--text)"
    } else {
        "var(--text-secondary)"
    };
    // Calculate the dynamic style outside the rsx! macro
    let transform_style = if active {
        "transform: translateY(-1px);"
    } else {
        ""
    };

    rsx! {
        button {
            style: "background: {button_bg}; color: {button_text}; border: none; cursor: pointer; \
                    white-space: nowrap; padding: 10px 24px; display: flex; \
                    align-items: center; border-radius: 8px; font-size: 0.9rem; \
                    font-weight: 600; \
                    transition: background 0.15s, transform 0.1s; \
                    {transform_style}",
            onclick: on_click,

            "{label}"
        }
    }
}

pub fn nav_action(
    label: &str,
    active: bool,
    on_click: impl FnMut(MouseEvent) + 'static,
) -> Element {
    let label_color = if active {
        "var(--text)"
    } else {
        "var(--text-secondary)"
    };
    let label_weight = if active {
        "600"
    } else {
        "400"
    };

    rsx! {
        button {
            style: "background: transparent; border: none; cursor: pointer; \
                    padding: 8px 12px; display: flex; align-items: center; \
                    font-size: 0.9rem; \
                    font-weight: {label_weight}; color: {label_color}; \
                    transition: color 0.15s; margin-right: 12px;",
            onclick: on_click,

            "{label}"
        }
    }
}