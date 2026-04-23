// ws/commands/trustset.rs
use crate::channel::WSCommand;
use crate::ws::commands::wallet_auth::Bip44Wallet; // Import our custom wallet
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;
use xrpl::models::transactions::trust_set::{TrustSet, TrustSetFlag};
use xrpl::models::{IssuedCurrencyAmount, XRPAmount};

// Manual signing imports
use bitcoin::secp256k1::{Message, Secp256k1};
use sha2::{Digest, Sha512};

// Define known issuers
const RLUSD_ISSUER: &str = "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De";
const EUROP_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
const XSGD_ISSUER: &str = "rK67JczCpaYXVtfw3qJVmqwpSfa1bYTptw";

pub async fn construct_blob(
    wallet_obj: &Bip44Wallet, // Updated type
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    // Default to a high limit if not provided
    let trustline_limit_value = cmd
        .trustline_limit
        .clone()
        .unwrap_or_else(|| "1000000".to_string());

    let asset_type = cmd
        .wallet_type
        .as_deref()
        .ok_or("Missing asset type for trustset")?;

    let (currency_hex, issuer_address) = match asset_type {
        "RLUSD" => ("524C555344000000000000000000000000000000", RLUSD_ISSUER),
        "EUROP" => ("4555524F50000000000000000000000000000000", EUROP_ISSUER),
        "XSGD" => ("5853474400000000000000000000000000000000", XSGD_ISSUER),
        _ => {
            return Err(format!(
                "Unsupported asset type for trustset: {}",
                asset_type
            ));
        }
    };

    let trustline_limit = IssuedCurrencyAmount {
        currency: Cow::Owned(currency_hex.to_string()),
        issuer: Cow::Owned(issuer_address.to_string()),
        value: Cow::Owned(trustline_limit_value),
    };

    // Public key for SigningPubKey field
    let pub_key_hex = hex::encode(wallet_obj.public_key.serialize()).to_uppercase();

    // Create the TrustSet transaction model
    let trust_set = TrustSet::new(
        Cow::Owned(wallet_obj.address.clone()),
        None,
        Some(XRPAmount(Cow::Owned(fee))),
        Some(vec![TrustSetFlag::TfSetNoRipple].into()),
        None,
        None,
        Some(sequence),
        None,
        None,
        None,
        trustline_limit,
        None,
        None,
    );

    // Convert to JSON Value to inject signing fields
    let mut tx_json_val = serde_json::to_value(&trust_set)
        .map_err(|e| format!("Failed to serialize trustline to JSON: {}", e))?;

    // 1. Inject SigningPubKey
    if let Some(obj) = tx_json_val.as_object_mut() {
        obj.insert("SigningPubKey".to_string(), serde_json::Value::String(pub_key_hex));
    }

    // 2. Serialize to hex (unsigned)
    let unsigned_tx_json = serde_json::to_string(&tx_json_val)
        .map_err(|e| format!("Failed to reserialize JSON: {}", e))?;
    
    let unsigned_hex = serialize_tx(unsigned_tx_json, false)
        .ok_or_else(|| "Failed to encode unsigned trustline to hex".to_string())?;

    let unsigned_bytes = hex::decode(&unsigned_hex)
        .map_err(|_| "Failed to decode unsigned hex".to_string())?;

    // 3. Prefix + Hash (0x53544E00 = STN\0)
    let mut payload = Vec::new();
    payload.extend_from_slice(&[0x53, 0x54, 0x58, 0x00]);
    payload.extend_from_slice(&unsigned_bytes);

    let mut hasher = Sha512::new();
    hasher.update(&payload);
    let hash = hasher.finalize();

    let mut hash_32 = [0u8; 32];
    hash_32.copy_from_slice(&hash[0..32]);

    // 4. Sign
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(&hash_32)
        .map_err(|_| "Invalid message hash".to_string())?;
    
    let sig = secp.sign_ecdsa(&message, &wallet_obj.secret_key);
    let der_sig = sig.serialize_der();
    let sig_hex = hex::encode(der_sig.as_ref()).to_uppercase();

    // 5. Inject TxnSignature
    if let Some(obj) = tx_json_val.as_object_mut() {
        obj.insert("TxnSignature".to_string(), serde_json::Value::String(sig_hex));
    }

    // 6. Final serialization
    let final_tx_json = serde_json::to_string(&tx_json_val).unwrap();
    let tx_blob = serialize_tx(final_tx_json, false)
        .ok_or_else(|| "Failed to encode final trustset to hex".to_string())?;

    Ok(tx_blob)
}