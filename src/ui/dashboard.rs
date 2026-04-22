// src/ui/dashboard.rs
use crate::channel::{Tab, Theme};
use crate::context::GlobalContext;
use crate::ui::{
    balance, managebtc, managexrp, progressbar::ProgressBar,
};
// Removed Sidebar imports entirely
use dioxus_native::prelude::*;

pub fn render_dashboard() -> Element {
    rsx! {
        div {
            class: "theme-root",
            style: "display: flex; flex-direction: column; height: 100%; width: 100%; overflow: hidden; position: relative;",
            MainViewSlot {}
            BottomDock {}
        }
    }
}

#[component]
fn MainViewSlot() -> Element {
    let global = use_context::<GlobalContext>();
    let progress = global.progress.read();

    match &*progress {
        Some(_) => rsx! { ProgressBar { operation_name: "Processing...".to_string() } },
        // Directly render TabContentSlot. Sub-view routing (Settings/Pin) is now handled inside balance::render_balance
        None => rsx! { TabContentSlot {} },
    }
}

#[component]
fn TabContentSlot() -> Element {
    let global = use_context::<GlobalContext>();
    let current_tab = *global.selected_tab.read();

    rsx! {
        div {
            class: "theme-bg-primary",
            style: "flex: 1; width: 100%; display: flex; overflow-y: auto;",
            match current_tab {
                Tab::Balance => rsx! { balance::render_balance {} },
                Tab::Xrp => rsx! { managexrp::render_manage_xrp {} },
                Tab::Btc => rsx! { managebtc::render_manage_btc {} },
            }
        }
    }
}

#[component]
fn BottomDock() -> Element {
    let global = use_context::<GlobalContext>();
    let current_tab = *global.selected_tab.read();
    
    // NEW: Also check if progress is happening
    let progress = global.progress.read();

    let (theme, _) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    let dock_bg = if is_dark { "#transparent" } else { "#f8fafc" };

    // HIDE if a sidebar is open OR if progress is active
    if progress.is_some() {
        return rsx! {};
    }

    rsx! {
        div {
            // Added z-index: 1 to ensure it stays below fixed overlays if they overlap
            style: "display: flex; width: 100%; height: 60px; background-color: {dock_bg}; z-index: 1;",
            DockButton {
                label: "BALANCE".to_string(),
                is_active: current_tab == Tab::Balance,
                is_dark,
                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::Balance); }
            }
            DockButton {
                label: "XRP".to_string(),
                is_active: current_tab == Tab::Xrp,
                is_dark,
                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::Xrp); }
            }
            DockButton {
                label: "BTC".to_string(),
                is_active: current_tab == Tab::Btc,
                is_dark,
                onclick: move |_| { let _ = crate::channel::CHANNEL.selected_tab_tx.send(Tab::Btc); }
            }
        }
    }
}
#[component]
fn DockButton(
    label: String,
    is_active: bool,
    is_dark: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let (text_color, bg_color) = if is_dark {
        if is_active {
            ("#ffffff", "#141414")
        } else {
            ("#737373", "transparent")
        }
    } else if is_active {
        ("#0f172a", "#e2e8f0")
    } else {
        ("#64748b", "transparent")
    };

    rsx! {
        button {
            style: "
                flex: 1;
                display: flex;
                align-items: center;
                justify-content: center;
                background-color: {bg_color};
                color: {text_color}; 
                font-family: monospace;
                font-size: 14px; 
                letter-spacing: 0.1em;
                cursor: pointer; 
                border: none;
                outline: none;
                margin: 0;
                padding: 0;
            ",
            onclick: onclick,
            "{label}"
        }
    }
}