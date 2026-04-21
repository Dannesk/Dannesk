// src/ui/managebtc/receive.rs
use crate::context::{BtcContext, GlobalContext};
use crate::ui::components::receive_layout::ReceiveAddressLayout;
use dioxus_native::prelude::*;
use crate::channel::Theme;


#[component]
pub fn view() -> Element {
    let mut btc_ctx = use_context::<BtcContext>();
    let global = use_context::<GlobalContext>();

    let (_, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let address = address_opt.unwrap_or_else(|| "No Address".to_string());
      let (theme, _) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    rsx! {
        ReceiveAddressLayout {
            network_name: "BITCOIN_NETWORK".to_string(),
            protocol_label: "BITCOIN".to_string(),
            address: address,
            is_dark: is_dark,
            on_back: move |_| {
                btc_ctx.btc_modal.with_mut(|state| {
                    if let Some(prev) = state.last_view {
                        state.view_type = prev;
                    }
                });
            }
        }
    }
}
