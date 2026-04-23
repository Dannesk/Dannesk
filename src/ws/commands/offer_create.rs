// ws/commands/offer_create.rs
use crate::channel::WSCommand;
use crate::ws::commands::wallet_auth::Bip44Wallet;
use rippled_binary_codec::serialize::serialize_tx;
use std::borrow::Cow;
use xrpl::models::transactions::offer_create::{OfferCreate, OfferCreateFlag};
use xrpl::models::transactions::{CommonFields, TransactionType};
use xrpl::models::{Amount, IssuedCurrencyAmount, XRPAmount};

// Manual signing and hashing
use bitcoin::secp256k1::{Message, Secp256k1};
use sha2::{Digest, Sha512};

const RLUSD_ISSUER: &str = "rMxCKbEDwqr76QuheSUMdEGf4B9xJ8m5De";
const EUROP_ISSUER: &str = "rMkEuRii9w9uBMQDnWV5AA43gvYZR9JxVK";
const XSGD_ISSUER: &str = "rK67JczCpaYXVtfw3qJVmqwpSfa1bYTptw";

// --- HIGH PRECISION HELPERS ---

fn xrp_str_to_drops(xrp_str: &str) -> Result<String, String> {
    if xrp_str.is_empty() || !xrp_str.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-') {
        return Err("Invalid XRP format".to_string());
    }
    let abs_str = xrp_str.trim_start_matches('-');
    let parts: Vec<&str> = abs_str.split('.').collect();
    
    let integer_part = parts[0];
    let mut fractional_part = if parts.len() == 2 { parts[1].to_string() } else { String::new() };

    while fractional_part.len() < 6 { fractional_part.push('0'); }
    if fractional_part.len() > 6 { fractional_part.truncate(6); }

    let integer_drops: u128 = integer_part.parse().unwrap_or(0);
    let fractional_drops: u128 = fractional_part.parse().unwrap_or(0);
    let total_drops = (integer_drops * 1_000_000) + fractional_drops;

    Ok(total_drops.to_string())
}

fn get_asset_config(symbol: &str) -> Option<(&'static str, &'static str)> {
    match symbol {
        "RLUSD" => Some(("524C555344000000000000000000000000000000", RLUSD_ISSUER)),
        "EUROP" => Some(("4555524F50000000000000000000000000000000", EUROP_ISSUER)),
        "XSGD" => Some(("5853474400000000000000000000000000000000", XSGD_ISSUER)),
        _ => None,
    }
}

fn to_xrpl_amount(amount_str: &str, currency: &str) -> Result<Amount<'static>, String> {
    if currency == "XRP" {
        let drops = xrp_str_to_drops(amount_str)?;
        Ok(Amount::XRPAmount(XRPAmount(Cow::Owned(drops))))
    } else {
        let (hex, issuer) = get_asset_config(currency)
            .ok_or_else(|| format!("Unsupported currency: {}", currency))?;
        
        // Use string formatting to avoid f64 precision issues where possible
        // We parse to f64 only to validate it's a number, then use the string directly
        let _ = amount_str.parse::<f64>().map_err(|_| "Invalid amount".to_string())?;

        Ok(Amount::IssuedCurrencyAmount(IssuedCurrencyAmount {
            currency: Cow::Owned(hex.to_string()),
            issuer: Cow::Owned(issuer.to_string()),
            value: Cow::Owned(amount_str.to_string()),
        }))
    }
}

// --- CORE LOGIC ---

pub async fn construct_blob(
    wallet_obj: &Bip44Wallet,
    cmd: &WSCommand,
    sequence: u32,
    fee: String,
) -> Result<String, String> {
    let taker_pays_raw = cmd.taker_pays.as_ref().ok_or("Missing taker_pays")?;
    let taker_gets_raw = cmd.taker_gets.as_ref().ok_or("Missing taker_gets")?;

    let taker_pays_amount = to_xrpl_amount(&taker_pays_raw.0, &taker_pays_raw.1)?;
    let taker_gets_amount = to_xrpl_amount(&taker_gets_raw.0, &taker_gets_raw.1)?;

    let mut flags: Vec<OfferCreateFlag> = vec![];
    if let Some(cmd_flags) = &cmd.flags {
        for flag in cmd_flags {
            match flag.as_str() {
                "tfFillOrKill" => flags.push(OfferCreateFlag::TfFillOrKill),
                "tfImmediateOrCancel" => flags.push(OfferCreateFlag::TfImmediateOrCancel),
                _ => (),
            }
        }
    }

    let pub_key_hex = hex::encode(wallet_obj.public_key.serialize()).to_uppercase();

    let common_fields = CommonFields {
        transaction_type: TransactionType::OfferCreate,
        account: Cow::Owned(wallet_obj.address.clone()),
        fee: Some(XRPAmount(Cow::Owned(fee))),
        sequence: Some(sequence),
        flags: flags.into(),
        ..Default::default() // Use default for unused fields
    };

    let offer_create = OfferCreate {
        common_fields,
        taker_gets: taker_gets_amount,
        taker_pays: taker_pays_amount,
        expiration: None,
        offer_sequence: None,
    };

    // --- MANUAL SIGNING FLOW ---

    let mut tx_json_val = serde_json::to_value(&offer_create)
        .map_err(|e| format!("JSON Serialization Error: {}", e))?;

    // 1. Inject SigningPubKey
    if let Some(obj) = tx_json_val.as_object_mut() {
        obj.insert("SigningPubKey".to_string(), serde_json::Value::String(pub_key_hex));
    }

    // 2. Hash the Binary (Unsigned)
    let unsigned_tx_json = serde_json::to_string(&tx_json_val).unwrap();
    let unsigned_hex = serialize_tx(unsigned_tx_json, false)
        .ok_or_else(|| "Binary encoding failed".to_string())?;
    let unsigned_bytes = hex::decode(&unsigned_hex).unwrap();

    // 3. Prefix + SHA-512/256 (STN\0)
    let mut payload = Vec::new();
    payload.extend_from_slice(&[0x53, 0x54, 0x58, 0x00]);
    payload.extend_from_slice(&unsigned_bytes);

    let mut hasher = Sha512::new();
    hasher.update(&payload);
    let hash = hasher.finalize();

    // 4. ECDSA Sign
    let secp = Secp256k1::new();
    let message = Message::from_digest_slice(&hash[0..32]).unwrap();
    let sig = secp.sign_ecdsa(&message, &wallet_obj.secret_key);
    let sig_hex = hex::encode(sig.serialize_der().as_ref()).to_uppercase();

    // 5. Inject TxnSignature
    if let Some(obj) = tx_json_val.as_object_mut() {
        obj.insert("TxnSignature".to_string(), serde_json::Value::String(sig_hex));
    }

    // 6. Final Blob
    let final_tx_json = serde_json::to_string(&tx_json_val).unwrap();
    let tx_blob = serialize_tx(final_tx_json, false)
        .ok_or_else(|| "Final encoding failed".to_string())?;

    Ok(tx_blob)
}