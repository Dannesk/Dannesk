use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::Zeroizing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransactionStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TransactionState {
    pub transactions: HashMap<String, TransactionData>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct TransactionData {
    pub tx_id: String,
    pub status: TransactionStatus,
    pub execution_price: String,
    pub order_type: String,
    pub timestamp: String,
    pub amount: String,
    pub currency: String,
    pub fee: String,
    pub flags: Option<String>,
    pub receiver: String,
    pub sender: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SignTransaction {
    pub step: u8,
    pub error: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub asset: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignTransactionState {
    pub send_transaction: Option<SignTransaction>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum ActiveView {
    #[default]
    Xrp,
    Rlusd,
    Euro,
    Sgd,
    Receive,
    Transactions,
    Import,
    Create,
    Send,
    Trade,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct XrpModalState {
    pub view_type: ActiveView,
    pub last_view: Option<ActiveView>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct XrpWalletProcessState {
    pub import_wallet: Option<XrpImport>,
    pub create_wallet: Option<XrpImport>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct XrpImport {
    pub step: u8,
    pub seed: Option<Zeroizing<String>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Trade {
    pub step: u8,
    pub base_asset: Option<String>,
    pub quote_asset: Option<String>,
    pub amount: Option<String>,
    pub limit_price: Option<String>,
    pub fee_percentage: f64,
    pub flags: Option<Vec<String>>,
    pub error: Option<String>,
    pub asset: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct SignTradeState {
    pub send_trade: Option<Trade>,
}
