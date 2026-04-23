// src/ui/managexrp/xrpimport/xrpimportlogic.rs

use crate::channel::{
    ActiveView, CHANNEL, ProgressState, WSCommand, XrpModalState, XrpWalletProcessState,
};
use crate::encrypt::encrypt_data;
use crate::bridge::json_storage::{write_json, remove_json}; 
use bip39::{Language, Mnemonic};
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::Secp256k1;
use serde::Serialize;
use serde_json::json; 
use std::str::FromStr;
use tokio::sync::mpsc::Sender;
use zeroize::{Zeroize, Zeroizing};

// Hashing for address generation
use sha2::{Digest, Sha256};
use ripemd::Ripemd160;

#[derive(Serialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

pub struct XRPImportLogic;

impl XRPImportLogic {
    pub async fn process(
        mnemonic_phrase: Zeroizing<String>,
        bip39_pass: Zeroizing<String>,
        encryption_pass: Zeroizing<String>,
        ws_tx: Sender<WSCommand>,
    ) {
        let _ = CHANNEL.progress_tx.send(Some(ProgressState {
            progress: 0.0,
            message: "Importing XRP Wallet...".to_string(),
        }));

        let m_thread = mnemonic_phrase.clone();
        let b_thread = bip39_pass.clone();
        let e_thread = encryption_pass.clone();

        let crypto_result = tokio::task::spawn_blocking(
            move || -> Result<(String, String, String, String), String> {
                // 1. Parse Mnemonic
                let mnemonic = Mnemonic::parse_in(Language::English, m_thread.as_str())
                    .map_err(|e| format!("Invalid mnemonic: {}", e))?;

                let mut seed = mnemonic.to_seed(b_thread.as_str());
                let secp = Secp256k1::new();

                // 2. Derive Master Key
                let xpriv = Xpriv::new_master(bitcoin::Network::Bitcoin, &seed)
                    .map_err(|e| {
                        seed.zeroize();
                        format!("Failed to create master key: {}", e)
                    })?;
                seed.zeroize();

                // 3. Derive BIP44 Path: m/44'/144'/0'/0/0
                let path = DerivationPath::from_str("m/44'/144'/0'/0/0")
                    .map_err(|_| "Invalid derivation path".to_string())?;

                let child_xpriv = xpriv
                    .derive_priv(&secp, &path)
                    .map_err(|e| format!("Derivation failed: {}", e))?;

                // 4. Generate Public Key (Compressed 33 bytes)
                let public_key = child_xpriv.to_priv().public_key(&secp);
                let pk_bytes = public_key.inner.serialize(); 

                // 5. Hash to Account ID (SHA256 then RIPEMD160)
                let sha_hash = Sha256::digest(&pk_bytes);
                let rip_hash = Ripemd160::digest(&sha_hash);

                // 6. Encode to XRP Classic Address (Base58Check with XRP Alphabet)
                let mut account_id = [0u8; 21];
                account_id[0] = 0x00; // Type 0 = Secp256k1
                account_id[1..].copy_from_slice(&rip_hash);

                let alphabet = bs58::Alphabet::new(b"rpshnaf39wBUDNEGHJKLM4PQRST7VWXYZ2bcdeCg65jkm8oFqi1tuvAxyz").unwrap();
                let address = bs58::encode(&account_id)
                    .with_alphabet(&alphabet)
                    .with_check()
                    .into_string();

                // 7. Encrypt for storage
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

                let wallet_data = EncryptedWalletData {
                    address: address.clone(),
                    encrypted_phrase: encrypted,
                    salt,
                    iv,
                };

                if let Err(e) = write_json("xrp_encrypt.json", &wallet_data) {
                    Self::handle_error(format!("FS Error: {}", e));
                    return;
                }

                let wallet_metadata = json!({
                    "address": address.clone(),
                    "private_key_deleted": false
                });

                if let Err(e) = write_json("xrp.json", &wallet_metadata) {
                    let _ = remove_json("xrp_encrypt.json");
                    Self::handle_error(format!("Metadata Error: {}", e));
                    return;
                }

                let _ = ws_tx.try_send(WSCommand {
                    command: "import_wallet".to_string(), 
                    wallet: Some(address),
                    recipient: None,
                    amount: None,
                    passphrase: None,
                    trustline_limit: None,
                    fee: None,
                    tx_type: None,
                    taker_pays: None,
                    taker_gets: None,
                    seed: None,
                    flags: None,
                    wallet_type: None,
                    bip39: None,
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