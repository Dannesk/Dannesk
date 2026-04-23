use crate::channel::{CHANNEL, ProgressState};
use crate::decrypt::decrypt_data;
use crate::bridge::json_storage::read_json;
use bip39::{Language, Mnemonic};
use bitcoin::bip32::{DerivationPath, Xpriv};
use bitcoin::secp256k1::{PublicKey, Secp256k1, SecretKey};
use serde::Deserialize;
use std::str::FromStr;
use zeroize::{Zeroize, Zeroizing};

// Matches the structure used in xrpimportlogic
#[derive(Deserialize)]
struct EncryptedWalletData {
    address: String,
    encrypted_phrase: String,
    salt: String,
    iv: String,
}

/// A custom struct to hold our BIP44 derived keys since we can no longer 
/// rely on xrpl::wallet::Wallet for m/44'/144'/0'/0/0 derivation.
#[derive(Clone)]
pub struct Bip44Wallet {
    pub address: String,
    pub secret_key: SecretKey,
    pub public_key: PublicKey,
    
}

pub fn authenticate_wallet(
    passphrase: Option<Zeroizing<String>>,
    seed: Option<Zeroizing<String>>,
    bip39: Option<Zeroizing<String>>,
    wallet_address: &str,
) -> Result<Bip44Wallet, String> {
    let _ = CHANNEL.progress_tx.send(Some(ProgressState {
        progress: 0.4,
        message: "Authenticating wallet".to_string(),
    }));

    let mnemonic_phrase: Zeroizing<String> = match (passphrase, seed) {
        (None, Some(s)) => s,
        (Some(p), None) => {
            // --- FILE-BASED AUTHENTICATION ---
            let stored_data: EncryptedWalletData = read_json("xrp_encrypt.json").map_err(|e| {
                let err_msg = format!("Error: XRP credentials not found: {}", e);
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Could not find encrypted credentials.".to_string(),
                }));
                err_msg
            })?;

            // Security check: ensure the file matches the requested wallet
            if stored_data.address != wallet_address {
                let err_msg = "Error: Stored data address mismatch".to_string();
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: err_msg.clone(),
                }));
                return Err(err_msg);
            }

            decrypt_data(
                p.clone(),
                &stored_data.encrypted_phrase,
                &stored_data.salt,
                &stored_data.iv,
            )
            .map_err(|_| {
                let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                    progress: 1.0,
                    message: "Error: Decryption failed (Incorrect passphrase)".to_string(),
                }));
                "Error: Decryption failed".to_string()
            })?
        }
        _ => {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Must provide either passphrase or seed".to_string(),
            }));
            return Err("Error: Must provide either passphrase or seed".to_string());
        }
    };

    // Use .as_str() to access the protected memory
    // Note: Mnemonic::parse_in rather than parse_in_normalized to match your create logic
    let mnemonic = Mnemonic::parse_in(Language::English, mnemonic_phrase.as_str())
        .map_err(|_| {
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: "Error: Invalid mnemonic".to_string(),
            }));
            "Error: Invalid mnemonic".to_string()
        })?;

    let seed_passphrase = bip39.as_deref().map(|s| s.as_str()).unwrap_or("");
    let mut bip39_seed = mnemonic.to_seed(seed_passphrase);
    
    let secp = Secp256k1::new();

    // Derive master key
    let xpriv = Xpriv::new_master(bitcoin::Network::Bitcoin, &bip39_seed)
        .map_err(|e| {
            bip39_seed.zeroize();
            let err_msg = format!("Error: Failed to create master key: {}", e);
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: err_msg.clone(),
            }));
            err_msg
        })?;
        
    bip39_seed.zeroize();

    // Standard BIP44 XRP Path
    let path = DerivationPath::from_str("m/44'/144'/0'/0/0")
        .map_err(|_| {
            let err_msg = "Error: Invalid derivation path".to_string();
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: err_msg.clone(),
            }));
            err_msg
        })?;

    let child_xpriv = xpriv
        .derive_priv(&secp, &path)
        .map_err(|e| {
            let err_msg = format!("Error: Derivation failed: {}", e);
            let _ = CHANNEL.progress_tx.send(Some(ProgressState {
                progress: 1.0,
                message: err_msg.clone(),
            }));
            err_msg
        })?;

    // Extract raw secp256k1 secret and public keys
    let secret_key = child_xpriv.private_key;
    let public_key = child_xpriv.to_priv().public_key(&secp).inner; 

    Ok(Bip44Wallet {
        address: wallet_address.to_string(),
        secret_key,
        public_key,
    })
}