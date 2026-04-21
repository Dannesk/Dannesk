use dioxus_native::prelude::*;
use crate::context::{GlobalContext, XrpContext};
use crate::ui::managexrp::xrpbalance::wallet_operations::WalletOperations;
use crate::utils::add_commas;
use crate::utils::balance_layout::{BalanceLayout, LedgerInfo};
use crate::utils::reserves::XrpBalanceInfo;
use crate::utils::styles::terminal_action;
use crate::utils::xrp::{XrpLogo, XrpLogoWhite};
use crate::channel::Theme;


pub mod wallet_operations;

#[component]
pub fn render_xrp_balance() -> Element {
    let global = use_context::<GlobalContext>(); 
    let xrp_ctx = use_context::<XrpContext>();

    let mut sign_tx = xrp_ctx.sign_transaction;
    let mut xrp_modal = xrp_ctx.xrp_modal;

    let (xrp_amount, address, key_is_deleted) = xrp_ctx.wallet_balance.read().clone();

    // --- THEME CHECK ---
   let (theme, hide_balance) = *global.theme_user.read();
    let is_dark = matches!(theme, Theme::Dark);

    let xrp_reserve_info = use_context::<Memo<XrpBalanceInfo>>();

    let xrp_logo = if is_dark {
        rsx! { XrpLogoWhite { size: "14".to_string() } }
    } else {
        rsx! { XrpLogo { size: "14".to_string() } }
    };

    // --- STATUS ---
    let status_color = if key_is_deleted {
        "var(--text-accent)"
    } else {
        "var(--text-secondary)"
    };
    let status_text = if key_is_deleted {
        "PURGED // KEY_OFF_DEVICE"
    } else {
        "ACTIVE // KEY_ON_DEVICE"
    };

    let rates = global.rates.read();
    let xrp_usd_rate = rates.get("XRP/USD").copied().unwrap_or(0.0) as f64;

    let total_usd = xrp_amount * xrp_usd_rate;
    let (int_part, frac_part) = if hide_balance {
        ("****".to_string(), "".to_string())
    } else {
        (
            add_commas(total_usd.floor() as i64),
            format!(".{:02}", (total_usd.fract() * 100.0).round() as i64),
        )
    };

    let formatted_raw_xrp = if hide_balance {
        "****".to_string()
    } else {
        format!("{:.6}", xrp_amount)
    };

    // ACTIONS
    let send_btn = terminal_action("SEND", true, move |_| {
        xrp_modal.with_mut(|s| {
            s.last_view = Some(crate::channel::ActiveView::Xrp);
            s.view_type = crate::channel::ActiveView::Send;
        });
        sign_tx.with_mut(|s| {
            s.send_transaction = Some(crate::channel::SignTransaction {
                step: 1,
                error: None,
                recipient: None,
                amount: None,
                asset: "XRP".to_string(),
            });
        });
    });

    let receive_btn = terminal_action("RECEIVE", true, move |_| {
        xrp_modal.with_mut(|s| {
            s.last_view = Some(crate::channel::ActiveView::Xrp);
            s.view_type = crate::channel::ActiveView::Receive;
        });
    });

    let purge_btn = terminal_action("PURGE", true, {
        let ws_tx = global.ws_tx.clone();
        let addr = address.clone();
        move |_| {
            if let Some(a) = addr.clone() {
                tokio::spawn(WalletOperations::remove_wallet(a, ws_tx.clone()));
            }
        }
    });

    let delete_btn = terminal_action("DELETE_KEY", true, {
        let addr = address.clone();
        move |_| {
            if let Some(a) = addr.clone() {
                tokio::spawn(WalletOperations::delete_key(a));
            }
        }
    });

    let optional_delete_btn = if !key_is_deleted {
        Some(delete_btn)
    } else {
        None
    };

    rsx! {
        BalanceLayout {
            asset_ticker: "XRP".to_string(),
            int_part: int_part,
            frac_part: frac_part,
            formatted_raw_amount: formatted_raw_xrp,
            status_color: status_color.to_string(),
            status_text: status_text.to_string(),
            network_protocol: "XRP_LEDGER // MAINNET".to_string(),
            send_btn: send_btn,
            receive_btn: receive_btn,
            purge_btn: purge_btn,
            delete_btn: optional_delete_btn,
            ledger_info: LedgerInfo::Xrp(xrp_reserve_info.read().clone()),
            logo: xrp_logo,
        }
    }
}
