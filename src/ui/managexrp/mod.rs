use crate::channel::{ActiveView, Trade, XrpImport};
use crate::context::{EuroContext, RlusdContext, SgdContext, XrpContext};
use crate::utils::reserves::get_xrp_balance_info;
use crate::utils::styles::{nav_action, terminal_action};
use bip39::{Language, Mnemonic};
use dioxus_native::prelude::*;
use rand::{Rng, rng};
use zeroize::Zeroizing;

pub mod manageeuro;
pub mod managerlusd;
pub mod managesgd;
pub mod receive;
pub mod trade;
pub mod transactions;
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

    // === XRP RESERVE CALCULATION ===
    let xrp_reserve_info = use_memo(move || {
        let (xrp_amount, _, _) = xrp.wallet_balance.read().clone();
        let has_rlusd = rlusd_ctx.rlusd.read().1;
        let has_euro = euro_ctx.euro.read().1;
        let has_sgd = sgd_ctx.sgd.read().1;

        let active_trustline_count = [has_rlusd, has_euro, has_sgd]
            .iter()
            .filter(|&&active| active)
            .count();

        get_xrp_balance_info(xrp_amount, active_trustline_count)
    });

    // We MUST provide the context so children can hook into it
    provide_context(xrp_reserve_info);

    match view_type {
        ActiveView::Import => return rsx! { xrpimport::view {} },
        ActiveView::Create => return rsx! { xrpcreate::view {} },
        ActiveView::Send => return rsx! { xrpsend::view {} },
        ActiveView::Trade => return rsx! { trade::view {} },
        ActiveView::Transactions => return rsx! { transactions::view {} },
        ActiveView::Receive => return rsx! { receive::view {} },
        _ => {}
    }

    // Asset Navigation
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

    let create_btn = terminal_action("CREATE_XRP_WALLET", true, move |_| {
        let mut entropy = [0u8; 32];
        rng().fill_bytes(&mut entropy);
        let mnemonic = Mnemonic::from_entropy_in(Language::English, &entropy).unwrap();
        let seed = Zeroizing::new(mnemonic.to_string());
        wallet_process.with_mut(|state| {
            state.create_wallet = Some(XrpImport {
                step: 1,
                seed: Some(seed),
                error: None,
            })
        });
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Create);
    });

    let import_btn = terminal_action("IMPORT_XRP_WALLET", true, move |_| {
        wallet_process.with_mut(|state| {
            state.import_wallet = Some(XrpImport {
                step: 1,
                seed: None,
                error: None,
            })
        });
        xrp_modal.with_mut(|s| s.view_type = ActiveView::Import);
    });

    let trade_btn = terminal_action("TRADE", matches!(view_type, ActiveView::Trade), move |_| {
        xrp_modal.with_mut(|state| {
            state.last_view = Some(ActiveView::Xrp);
            state.view_type = ActiveView::Trade;
        });
        trade_tx.with_mut(|state| {
            state.send_trade = Some(Trade {
                step: 1,
                base_asset: None,
                quote_asset: None,
                amount: None,
                limit_price: None,
                fee_percentage: 0.0,
                flags: None,
                error: None,
                asset: "XRP".to_string(),
            });
        });
    });

    let history_btn = terminal_action(
        "HISTORY",
        matches!(view_type, ActiveView::Transactions),
        move |_| {
            xrp_modal.with_mut(|state| {
                state.last_view = Some(ActiveView::Xrp);
                state.view_type = ActiveView::Transactions;
            });
        },
    );

    rsx! {
        style { {r#"
            .terminal-viewport { 
                display: flex; 
                flex-direction: row; 
                width: 100%; 
                flex: 1;
                justify-content: center; 
                padding: 0 2rem; 
                box-sizing: border-box; 
            }
            .setup-container {
                width: 100%;
                max-width: 600px; 
                display: flex;
                flex-direction: column;
                align-items: center;
            }
            .term-sidebar { 
                display: flex; 
                flex-direction: column; 
                gap: 1rem; 
                justify-content: center; 
                
            }
            .term-main { 
                flex: 1; 
                display: flex; 
                flex-direction: column; 
                align-items: center; 
                justify-content: center; 
            }
        "#} }

        div { class: "terminal-viewport",
            div { class: "term-sidebar",
                if has_wallet {
                    {nav_xrp}
                    {nav_usd}
                    {nav_eur}
                    {nav_sgd}
                }
            }

            div { class: "term-main",
                if !has_wallet {
                    div { class: "setup-container",
                       
                        div {
                        style: "display: flex; flex-direction: column; gap: 1rem; width: 100%; align-items: center;",
                            {create_btn}
                            {import_btn}
                        }
                    }
                } else {
                    match view_type {
                        ActiveView::Rlusd => rsx! { managerlusd::render_rlusd_balance {} },
                        ActiveView::Euro => rsx! { manageeuro::render_manage_euro {} },
                        ActiveView::Sgd => rsx! { managesgd::render_sgd_balance {} },


                        _ => rsx! { xrpbalance::render_xrp_balance {} },
                       }
                }
            }

            div { class: "term-sidebar",
                style: "align-items: flex-end;",
                if has_wallet {
                    {trade_btn}
                    {history_btn}
                }
            }
        }
    }
}
