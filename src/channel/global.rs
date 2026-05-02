use zeroize::Zeroizing;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Tab {
    Balance,
    Xrp,
    Btc,
    Rates,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ProgressState {
    pub progress: f32,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BalanceActiveView {
    Main,
    Settings,
    ChangePin,
}


#[derive(Debug, Default, Clone)]
pub struct WSCommand {
    pub command: String,
    pub wallet: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub passphrase: Option<Zeroizing<String>>,
    pub trustline_limit: Option<String>,
    pub fee: Option<String>,
    pub tx_type: Option<String>,
    pub taker_pays: Option<(String, String)>,
    pub taker_gets: Option<(String, String)>,
    pub seed: Option<Zeroizing<String>>,
    pub flags: Option<Vec<String>>,
    pub wallet_type: Option<String>,
    pub bip39: Option<Zeroizing<String>>,
}
