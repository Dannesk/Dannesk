use crate::channel::{ActiveView, Trade, XrpImport};
use crate::context::{EuroContext, RlusdContext, SgdContext, XrpContext};
use crate::utils::reserves::get_xrp_balance_info;
use crate::utils::styles::{nav_action};
// Import the newly created layout component
use crate::ui::components::xrp_manage_layout::XrpManageLayout;
use bip39::{Language, Mnemonic};
use dioxus_native::prelude::*;
use rand::{Rng, rng};
use zeroize::Zeroizing;

pub mod manageeuro;
pub mod managerlusd;
pub mod managesgd;
pub mod receive;
pub mod trade;
pub mod xrptransactions;
pub mod xrpbalance;
pub mod xrpcreate;
pub mod xrpimport;
pub mod xrpsend;

pub fn render_manage_xrp() -> Element {
    let xrp = use_context::<XrpContext>();
    let rlusd_ctx = use_context::<RlusdContext>();
    let euro_ctx = use_context::<EuroContext>();
    let sgd_ctx = use_context::<SgdContext>();

    let mut xrp_modal = xrp.xrp_modal;
    let mut wallet_process = xrp.wallet_process;
    let mut trade_tx = xrp.trade;

    let view_type = xrp_modal.read().view_type;
    let (_amount, address_opt, _) = xrp.wallet_balance.read().clone();
    let has_wallet = address_opt.is_some();

    let has_transactions = !xrp.transactions.read().transactions.is_empty();
    

    // === XRP RESERVE CALCULATION ===
    let xrp_reserve_info = use_memo(move || {
        let (xrp_amount, _, _) = xrp.wallet_balance.read().clone();
        let active_trustline_count = [
            rlusd_ctx.rlusd.read().1, 
            euro_ctx.euro.read().1, 
            sgd_ctx.sgd.read().1
        ].iter().filter(|&&active| active).count();

        get_xrp_balance_info(xrp_amount, active_trustline_count)
    });

    provide_context(xrp_reserve_info);

    // Early returns for sub-views
    match view_type {
        ActiveView::Import => return rsx! { xrpimport::view {} },
        ActiveView::Create => return rsx! { xrpcreate::view {} },
        ActiveView::Send => return rsx! { xrpsend::view {} },
        ActiveView::Trade => return rsx! { trade::view {} },
        ActiveView::Transactions => return rsx! { xrptransactions::view {} },
        ActiveView::Receive => return rsx! { receive::view {} },
        _ => {}
    }

    // --- BUTTON DEFINITIONS ---
    let nav_xrp = nav_action("XRP", matches!(view_type, ActiveView::Xrp), move |_| {
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Xrp)
    });
    let nav_usd = nav_action("USD", matches!(view_type, ActiveView::Rlusd), move |_| {
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Rlusd)
    });
    let nav_eur = nav_action("EUR", matches!(view_type, ActiveView::Euro), move |_| {
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Euro)
    });
    let nav_sgd = nav_action("SGD", matches!(view_type, ActiveView::Sgd), move |_| {
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Sgd)
    });

    let trade_btn = nav_action("DEX", matches!(view_type, ActiveView::Trade), move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(ActiveView::Xrp);
            state.view_type = ActiveView::Trade;
        });
        trade_tx.with_mut(|state| {
            state.send_trade = Some(Trade {
                step: 1, base_asset: None, quote_asset: None, amount: None,
                limit_price: None, fee_percentage: 0.0, flags: None, error: None,
                asset: "XRP".to_string(),
            });
        });
    });

  let history_btn = nav_action("TX_LOG", matches!(view_type, ActiveView::Transactions), move |_| {
    xrp_modal.with_mut(|state| { 
        state.last_view = Some(ActiveView::Xrp);
        state.view_type = ActiveView::Transactions;
    });
});

    // --- DETERMINE BALANCE VIEW ---
    let active_balance_view = match view_type {
        ActiveView::Rlusd => rsx! { managerlusd::render_rlusd_balance {} },
        ActiveView::Euro => rsx! { manageeuro::render_manage_euro {} },
        ActiveView::Sgd => rsx! { managesgd::render_sgd_balance {} },
        _ => rsx! { xrpbalance::render_xrp_balance {} },
    };

    // --- MAIN UI RENDER ---
    rsx! {
        XrpManageLayout {
            has_wallet: has_wallet,
            nav_xrp: nav_xrp,
            nav_usd: nav_usd,
            nav_eur: nav_eur,
            nav_sgd: nav_sgd,
            trade_btn: trade_btn,
            history_btn: if has_transactions { Some(history_btn) } else { None },
            on_create_click: move |_| {
                let mut entropy = [0u8; 32];
                rng().fill_bytes(&mut entropy);
                let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
                let seed = Zeroizing::new(mnemonic.to_string());
                wallet_process.with_mut(|state| {
                    state.create_wallet = Some(XrpImport { step: 1, seed: Some(seed), error: None })
                });
                xrp_modal.with_mut(|s| s.view_type = ActiveView::Create);
            },
            on_import_click: move |_| {
                wallet_process.with_mut(|state| {
                    state.import_wallet = Some(XrpImport { step: 1, seed: None, error: None })
                });
                xrp_modal.with_mut(|s| s.view_type = ActiveView::Import);
            },
            active_balance_view: active_balance_view,
        }
    }
}