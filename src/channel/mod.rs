pub mod btc;
pub mod global;
pub mod xrp;

pub use btc::*;
pub use global::*;
pub use xrp::*;

use std::collections::HashMap;
use std::sync::LazyLock;
use tokio::sync::watch;

pub static CHANNEL: LazyLock<Channel> = LazyLock::new(Channel::new);

pub struct Channel {
    //global related channels
    pub rates_tx: watch::Sender<HashMap<String, f32>>,
    pub rates_rx: watch::Receiver<HashMap<String, f32>>,
    pub selected_tab_tx: watch::Sender<Tab>,
    pub selected_tab_rx: watch::Receiver<Tab>,
    pub theme_user_tx: watch::Sender<(bool, bool)>,
    pub theme_user_rx: watch::Receiver<(bool, bool)>,
    pub progress_tx: watch::Sender<Option<ProgressState>>,
    pub progress_rx: watch::Receiver<Option<ProgressState>>,
    pub version_tx: watch::Sender<Option<String>>,
    pub version_rx: watch::Receiver<Option<String>>,
    pub exchange_ws_status_tx: watch::Sender<bool>,
    pub exchange_ws_status_rx: watch::Receiver<bool>,
    pub crypto_ws_status_tx: watch::Sender<bool>,
    pub crypto_ws_status_rx: watch::Receiver<bool>,
    pub sidebar_view_tx: watch::Sender<SideBarView>,
    pub sidebar_view_rx: watch::Receiver<SideBarView>,

    //rlusd / sgd / euro channels
    pub rlusd_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub rlusd_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    pub sgd_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub sgd_rx: watch::Receiver<(f64, bool, Option<f64>)>,
    pub euro_tx: watch::Sender<(f64, bool, Option<f64>)>,
    pub euro_rx: watch::Receiver<(f64, bool, Option<f64>)>,

    //xrp channels
    pub wallet_balance_tx: watch::Sender<(f64, Option<String>, bool)>,
    pub wallet_balance_rx: watch::Receiver<(f64, Option<String>, bool)>,
    pub xrp_modal_tx: watch::Sender<XrpModalState>,
    pub xrp_modal_rx: watch::Receiver<XrpModalState>,
    pub sign_transaction_tx: watch::Sender<SignTransactionState>,
    pub sign_transaction_rx: watch::Receiver<SignTransactionState>,
    pub transactions_tx: watch::Sender<TransactionState>,
    pub transactions_rx: watch::Receiver<TransactionState>,
    pub xrp_wallet_process_tx: watch::Sender<XrpWalletProcessState>,
    pub xrp_wallet_process_rx: watch::Receiver<XrpWalletProcessState>,
    pub trade_tx: watch::Sender<SignTradeState>,
    pub trade_rx: watch::Receiver<SignTradeState>,

    //bitcoin related channels
    pub btc_modal_tx: watch::Sender<BtcModalState>,
    pub btc_modal_rx: watch::Receiver<BtcModalState>,
    pub bitcoin_wallet_tx: watch::Sender<(f64, Option<String>, bool)>,
    pub bitcoin_wallet_rx: watch::Receiver<(f64, Option<String>, bool)>,
    pub btc_transactions_tx: watch::Sender<BtcTransactionState>,
    pub btc_transactions_rx: watch::Receiver<BtcTransactionState>,
    pub btc_wallet_process_tx: watch::Sender<BtcWalletProcessState>,
    pub btc_wallet_process_rx: watch::Receiver<BtcWalletProcessState>,
    pub btc_sign_transaction_tx: watch::Sender<BtcSignTransactionState>,
    pub btc_sign_transaction_rx: watch::Receiver<BtcSignTransactionState>,
}

impl Channel {
    pub fn new() -> Self {
        //global related
        let (theme_user_tx, theme_user_rx) = watch::channel((false, false));
        let (rates_tx, rates_rx) = watch::channel(HashMap::new());
        let (selected_tab_tx, selected_tab_rx) = watch::channel(Tab::Balance);
        let (progress_tx, progress_rx) = watch::channel(None);
        let (version_tx, version_rx) = watch::channel(None);
        let (exchange_ws_status_tx, exchange_ws_status_rx) = watch::channel(false);
        let (crypto_ws_status_tx, crypto_ws_status_rx) = watch::channel(false);
        let (sidebar_view_tx, sidebar_view_rx) = watch::channel(SideBarView::None);

        //token balances
        let (rlusd_tx, rlusd_rx) = watch::channel((0.0, false, None));
        let (sgd_tx, sgd_rx) = watch::channel((0.0, false, None));
        let (euro_tx, euro_rx) = watch::channel((0.0, false, None));

        //xrp related
        let (wallet_balance_tx, wallet_balance_rx) = watch::channel((0.0, None, false));
        let (xrp_modal_tx, xrp_modal_rx) = watch::channel(XrpModalState::default());
        let (sign_transaction_tx, sign_transaction_rx) =
            watch::channel(SignTransactionState::default());
        let (xrp_wallet_process_tx, xrp_wallet_process_rx) =
            watch::channel(XrpWalletProcessState::default());
        let (transactions_tx, transactions_rx) = watch::channel(TransactionState::default());
        let (trade_tx, trade_rx) = watch::channel(SignTradeState::default());

        //btc related
        let (bitcoin_wallet_tx, bitcoin_wallet_rx) = watch::channel((0.0, None, false));
        let (btc_modal_tx, btc_modal_rx) = watch::channel(BtcModalState::default());
        let (btc_transactions_tx, btc_transactions_rx) =
            watch::channel(BtcTransactionState::default());
        let (btc_sign_transaction_tx, btc_sign_transaction_rx) =
            watch::channel(BtcSignTransactionState::default());
        let (btc_wallet_process_tx, btc_wallet_process_rx) =
            watch::channel(BtcWalletProcessState::default());

        Channel {
            theme_user_tx,
            theme_user_rx,
            rates_tx,
            rates_rx,
            selected_tab_tx,
            selected_tab_rx,
            progress_tx,
            progress_rx,
            version_tx,
            version_rx,
            exchange_ws_status_tx,
            exchange_ws_status_rx,
            crypto_ws_status_tx,
            crypto_ws_status_rx,
            sidebar_view_tx,
            sidebar_view_rx,

            rlusd_tx,
            rlusd_rx,
            sgd_tx,
            sgd_rx,
            euro_tx,
            euro_rx,

            wallet_balance_tx,
            wallet_balance_rx,
            xrp_modal_tx,
            xrp_modal_rx,
            sign_transaction_tx,
            sign_transaction_rx,
            transactions_tx,
            transactions_rx,
            xrp_wallet_process_tx,
            xrp_wallet_process_rx,
            trade_tx,
            trade_rx,

            bitcoin_wallet_tx,
            bitcoin_wallet_rx,
            btc_modal_tx,
            btc_modal_rx,
            btc_transactions_tx,
            btc_transactions_rx,
            btc_wallet_process_tx,
            btc_wallet_process_rx,
            btc_sign_transaction_tx,
            btc_sign_transaction_rx,
        }
    }
}
