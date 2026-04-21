// src/ui/managebtc/mod.rs
use crate::channel::{BtcActiveView, BtcImport};
use crate::context::BtcContext;
use crate::utils::styles::terminal_action;
// Import the newly created layout component
use crate::ui::components::btc_manage_layout::BtcManageLayout;
use bip39::{Language, Mnemonic};
use dioxus_native::prelude::*;
use rand::{Rng, rng};
use zeroize::Zeroizing;

pub mod btcbalance;
pub mod btccreate;
pub mod btcimport;
pub mod btcsend;
pub mod btctransactions;
pub mod receive;

#[component]
pub fn render_manage_btc() -> Element {
    let btc_ctx = use_context::<BtcContext>();

    let mut btc_modal = btc_ctx.btc_modal;
    let mut btc_wallet_process = btc_ctx.btc_wallet_process;

    let view_type = btc_modal.read().view_type;
    let (_balance, address_opt, _) = btc_ctx.bitcoin_wallet.read().clone();
    let has_wallet = address_opt.is_some();

    // --- THE GATE ---
    match view_type {
        BtcActiveView::Import => return rsx! { btcimport::view {} },
        BtcActiveView::Create => return rsx! { btccreate::view {} },
        BtcActiveView::Send => return rsx! { btcsend::view {} },
        BtcActiveView::Transactions => return rsx! { btctransactions::view {} },
        BtcActiveView::Receive => return rsx! { receive::view {} },
        BtcActiveView::Btc => {}
    }

    // --- TERMINAL ACTIONS ---
    let history_btn = terminal_action(
        "HISTORY",
        matches!(view_type, BtcActiveView::Transactions),
        move |_| {
            btc_modal.with_mut(|s| s.view_type = BtcActiveView::Transactions);
        },
    );

    // --- RENDER ---
    rsx! {
        BtcManageLayout {
            has_wallet: has_wallet,
            history_btn: history_btn,
            on_create_click: move |_| {
                let mut entropy = [0u8; 32];
                rng().fill_bytes(&mut entropy);
                let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
                let seed = Zeroizing::new(mnemonic.to_string());
                btc_wallet_process.with_mut(|state| {
                    state.create_wallet = Some(BtcImport { step: 1, seed: Some(seed), error: None })
                });
                btc_modal.with_mut(|s| s.view_type = BtcActiveView::Create);
            },
            on_import_click: move |_| {
                btc_wallet_process.with_mut(|state| {
                    state.import_wallet = Some(BtcImport { step: 1, seed: None, error: None })
                });
                btc_modal.with_mut(|s| s.view_type = BtcActiveView::Import);
            },
            active_balance_view: rsx! { btcbalance::view {} },
        }
    }
}