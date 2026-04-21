// src/ui/managexrp/xrpcreate/xrpcreatelogic.rs

use crate::channel::{
    ActiveView, CHANNEL, ProgressState, WSCommand, XrpModalState, XrpWalletProcessState,
};
use crate::encrypt::encrypt_data;
use crate::bridge::json_storage::{write_json, remove_json}; // <--- Added remove_json
use bip39::{Language, Mnemonic};
use ripple_address_codec::{Ed25519, encode_seed};
use serde::Serialize;
use serde_json::json; // <--- Added
use tokio::sync::mpsc::Sender;
use xrpl::wallet::Wallet;
use zeroize::{Zeroize, Zeroizing};

#[derive(Serialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub struct XRPCreateLogic;

impl XRPCreateLogic {
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
                    .map_err(|e| format!("Invalid mnemonic: {}", e))?;

                let seed_bytes = mnemonic.to_seed(b_thread.as_str());

                let mut entropy: [u8; 16] =
                    seed_bytes[0..16].try_into().expect("BIP39 Invalid Seed");
                let mut base58_seed = encode_seed(&entropy, &Ed25519);

                entropy.zeroize();

                let wallet = Wallet::new(&base58_seed, 0).map_err(|e| {
                    base58_seed.zeroize();
                    format!("Wallet creation failed: {}", e)
                })?;

                let address = wallet.classic_address.clone();
                base58_seed.zeroize();

                let (enc, salt, iv) = encrypt_data(e_thread, m_thread)
                    .map_err(|e| format!("Encryption failed: {}", e))?;

                Ok((address, enc, salt, iv))
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

                if let Err(e) = write_json("xrp_encrypt.json", &wallet_data) {
                    Self::handle_error(format!("FS Error (Encrypt): {}", e));
                    return;
                }

                // 2. Save Basic Wallet Metadata
                let wallet_metadata = json!({
                    "address": address.clone(),
                    "private_key_deleted": false
                });

                if let Err(e) = write_json("xrp.json", &wallet_metadata) {
                    let _ = remove_json("xrp_encrypt.json"); // Rollback
                    Self::handle_error(format!("FS Error (Metadata): {}", e));
                    return;
                }

                let _ = ws_tx.try_send(WSCommand {
                    command: "import_wallet".to_string(),
                    wallet: Some(address),
                    ..Default::default()
                });

                let _ = CHANNEL.xrp_wallet_process_tx.send(XrpWalletProcessState {
                    import_wallet: None,
                    create_wallet: None,
                });

                let _ = CHANNEL.xrp_modal_tx.send(XrpModalState {
                    view_type: ActiveView::Xrp,
                    last_view: None,
                });
            }
            Ok(Err(e)) => Self::handle_error(e),
            _ => Self::handle_error("Internal thread error".to_string()),
        }
    }

    fn handle_error(msg: String) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 1.0,
            message: msg,
        }));
    }
}