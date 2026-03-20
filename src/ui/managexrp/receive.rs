// src/ui/managexrp/receive.rs
use crate::context::{GlobalContext, XrpContext};
use crate::utils::receive_layout::ReceiveAddressLayout;
use dioxus_native::prelude::*;
use crate::channel::Theme;

#[component]
pub fn view() -> Element {
    let mut xrp_ctx = use_context::<XrpContext>();
    let global = use_context::<GlobalContext>();

    let (_, address_opt, _) = xrp_ctx.wallet_balance.read().clone();
    let address = address_opt.unwrap_or_else(|| "No Address".to_string());
    let (theme, _) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    rsx! {
        ReceiveAddressLayout {
            network_name: "XRP_LEDGER".to_string(),
            protocol_label: "XRPL".to_string(),
            address: address,
            is_dark: is_dark,
            on_back: move |_| {
                xrp_ctx.xrp_modal.with_mut(|state| {
                    if let Some(prev) = state.last_view {
                        state.view_type = prev;
                    }
                });
            }
        }
    }
}
