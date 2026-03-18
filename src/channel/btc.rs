use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use zeroize::Zeroizing;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct BtcImport {
    pub step: u8,
    pub seed: Option<Zeroizing<String>>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtcModalState {
    pub view_type: BtcActiveView,
    pub last_view: Option<BtcActiveView>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BtcSignTransaction {
    pub step: u8,
    pub error: Option<String>,
    pub recipient: Option<String>,
    pub amount: Option<String>,
    pub asset: String,
    pub fee: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtcSignTransactionState {
    pub send_transaction: Option<BtcSignTransaction>,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtcWalletProcessState {
    pub import_wallet: Option<BtcImport>,
    pub create_wallet: Option<BtcImport>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum BtcActiveView {
    #[default]
    Btc,
    Receive,
    Transactions,
    Import,
    Create,
    Send,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BitcoinTransactionStatus {
    Success,
    Failed,
    Pending,
    Cancelled,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct BtcTransactionState {
    pub transactions: HashMap<String, BtcTransactionData>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Deserialize)]
pub struct BtcTransactionData {
    pub txid: String,                     // Transaction ID (txid)
    pub status: BitcoinTransactionStatus, // Pending, Success, Failed, or Cancelled
    pub amount: String, // Amount transferred to non-wallet addresses (in satoshis)
    pub fees: String,   // Fee in satoshis
    pub receiver_addresses: Vec<String>, // List of recipient addresses
    pub sender_addresses: Vec<String>, // List of sender addresses
    pub timestamp: String, // ISO 8601 timestamp
}
