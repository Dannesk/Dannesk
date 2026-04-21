// src/ui/managebtc/btccreate/btccreatelogic.rs

use bip39::{Language, Mnemonic};
use bitcoin::address::Address;
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use bitcoin::{CompressedPublicKey, Network};
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use serde::Serialize;
use serde_json::json; // <--- Added
use zeroize::{Zeroize, Zeroizing};

use crate::channel::{
    BtcActiveView, BtcModalState, BtcWalletProcessState, CHANNEL, ProgressState, WSCommand,
};
use crate::encrypt::encrypt_data;
use crate::bridge::json_storage::{write_json, remove_json}; // <--- Added remove_json

#[derive(Serialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub struct BTCCreateLogic;

impl BTCCreateLogic {
    pub async fn process(
        mnemonic_phrase: Zeroizing<String>,
        bip39_pass: Zeroizing<String>,
        encryption_pass: Zeroizing<String>,
        ws_tx: Sender<WSCommand>,
    ) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Finalizing wallet creation...".to_string(),
        }));

        let m_thread = mnemonic_phrase.clone();
        let b_thread = bip39_pass.clone();
        let e_thread = encryption_pass.clone();

        let crypto_result = tokio::task::spawn_blocking(
            move || -> Result<(String, String, String, String), String> {
                let mnemonic = Mnemonic::parse_in(Language::English, m_thread.as_str())
                    .map_err(|e| format!("Invalid generated mnemonic: {}", e))?;

                let (enc, salt, iv) = encrypt_data(e_thread, m_thread)
                    .map_err(|e| format!("Encryption failed: {}", e))?;

                let mut seed = mnemonic.to_seed(b_thread.as_str());
                let network = Network::Bitcoin;
                let secp = Secp256k1::new();

                let xpriv = Xpriv::new_master(network, &seed).map_err(|e| {
                    seed.zeroize();
                    format!("Failed to create master key: {}", e)
                })?;

                seed.zeroize();

                let derivation_path = DerivationPath::from_str("m/84'/0'/0'/0/0")
                    .map_err(|_| "Invalid derivation path".to_string())?;

                let child_xpriv = xpriv
                    .derive_priv(&secp, &derivation_path)
                    .map_err(|e| format!("Derivation failed: {}", e))?;

                let public_key = child_xpriv.to_priv().public_key(&secp);
                let compressed_pubkey = CompressedPublicKey(public_key.inner);
                let address = Address::p2wpkh(&compressed_pubkey, network);

                Ok((address.to_string(), enc, salt, iv))
            },
        )
        .await;

        match crypto_result {
            Ok(Ok((address, encrypted, salt, iv))) => {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 0.5,
                    message: "Saving encrypted credentials...".to_string(),
                }));

                // 1. Save Encryption Data
                let wallet_data = EncryptedWalletData {
                    address: address.clone(),
                    encrypted_phrase: encrypted,
                    salt,
                    iv,
                };

                if let Err(e) = write_json("btc_encrypt.json", &wallet_data) {
                    Self::handle_error(format!("FS Error (Encrypt): {}", e));
                    return;
                }

                // 2. Save Basic Wallet Metadata
                let wallet_metadata = json!({
                    "address": address.clone(),
                    "private_key_deleted": false
                });

                if let Err(e) = write_json("btc.json", &wallet_metadata) {
                    let _ = remove_json("btc_encrypt.json"); // Rollback
                    Self::handle_error(format!("FS Error (Metadata): {}", e));
                    return;
                }

                let _ = ws_tx.try_send(WSCommand {
                    command: "import_bitcoin_wallet".to_string(),
                    wallet: Some(address),
                    ..Default::default()
                });

                let _ = CHANNEL.btc_wallet_process_tx.send(BtcWalletProcessState {
                    import_wallet: None,
                    create_wallet: None,
                });

                let _ = CHANNEL.btc_modal_tx.send(BtcModalState {
                    view_type: BtcActiveView::Btc,
                    last_view: None,
                });
            }
            Ok(Err(e)) => Self::handle_error(e),
            Err(e) => Self::handle_error(format!("Internal Thread Error: {}", e)),
        }
    }

    fn handle_error(msg: String) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: msg,
        }));
    }
}