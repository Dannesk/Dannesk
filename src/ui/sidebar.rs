use crate::channel::{CHANNEL, SideBarView, Theme};
use crate::context::GlobalContext;
use crate::utils::styles::sidebar_action;
use dioxus_native::prelude::*;

pub fn render_theme_toggle() -> Element {
    let global = use_context::<GlobalContext>();
    let (theme, hide_balance) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);
    let label = if is_dark { "LIGHT" } else { "DARK" };

    sidebar_action(label, is_dark, move |_| {
        let new_theme = if is_dark { Theme::Light } else { Theme::Dark };
        let _ = CHANNEL.theme_user_tx.send((new_theme, hide_balance));
    })
}

pub fn render_balance_toggle() -> Element {
    let global = use_context::<GlobalContext>();
    let (theme, hide_balance) = *global.theme_user.read();
    let is_visible = !hide_balance;
    let label = if is_visible { "HIDE" } else { "REVEAL" };

    sidebar_action(label, is_visible, move |_| {
        let _ = CHANNEL.theme_user_tx.send((theme, !hide_balance));
    })
}

pub fn render_pin_button() -> Element {
    let global = use_context::<GlobalContext>();
    let sidebar_view = *global.sidebar_view.read();
    let is_active = sidebar_view == SideBarView::ChangePin;

    sidebar_action("PIN", is_active, move |_| {
        let _ = CHANNEL.sidebar_view_tx.send(SideBarView::ChangePin);
    })
}

pub fn render_rates_button() -> Element {
    let global = use_context::<GlobalContext>();
    let sidebar_view = *global.sidebar_view.read();
    let is_active = sidebar_view == SideBarView::ExchangeRates;

    sidebar_action("RATES", is_active, move |_| {
        let _ = CHANNEL.sidebar_view_tx.send(SideBarView::ExchangeRates);
    })
}

pub fn render_network_button() -> Element {
    let global = use_context::<GlobalContext>();
    let sidebar_view = *global.sidebar_view.read();
    let is_active = sidebar_view == SideBarView::NetworkStatus;

    sidebar_action("NETWORK", is_active, move |_| {
        let _ = CHANNEL.sidebar_view_tx.send(SideBarView::NetworkStatus);
    })
}