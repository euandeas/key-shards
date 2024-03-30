use std::str::from_utf8;

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};
use shami_rs::aead as saead;
use shami_rs::base as sbase;

use crate::padding::pkcs7_unpad;

#[tauri::command]
pub fn build_secret(list: Vec<String>) -> Result<String, String> {
    let mut shares_bytes = vec![vec![]; list.len()];
    for (i, share) in list.iter().enumerate() {
        match Mnemonic::parse_normalized(share.as_str()) {
            Ok(m) => {
                shares_bytes[i] = pkcs7_unpad(m.to_entropy().to_vec());
            }
            Err(_) => match URL_SAFE_NO_PAD.decode(share) {
                Ok(b) => {
                    shares_bytes[i] = b;
                }
                Err(_) => shares_bytes[i] = share.as_bytes().to_vec(),
            },
        }
    }

    let longest = shares_bytes
        .iter()
        .map(|element| element.len())
        .max()
        .unwrap_or(0);

    let mut short_share_bytes: Vec<Vec<u8>> = shares_bytes
        .iter()
        .filter(|share| share.len() < longest)
        .cloned()
        .collect();

    shares_bytes.retain(|share| share.len() == longest);

    if short_share_bytes.len() > 2 {
        return Err("Invalid Shares".to_string());
    }

    let aead = check_suffixes_same(&shares_bytes);

    if !short_share_bytes.is_empty() {
        if aead {
            let suffix_to_append = &shares_bytes[0][34..];
            short_share_bytes = hash_short_shares(&short_share_bytes, longest, true);
            for short_share in &mut short_share_bytes {
                short_share.extend_from_slice(suffix_to_append);
            }
            let mut all_shares: Vec<Vec<u8>> = vec![];
            all_shares.extend(short_share_bytes.iter().cloned());
            all_shares.extend(shares_bytes);
            handle_secret_result(saead::rebuild_secret(all_shares))
        } else {
            short_share_bytes = hash_short_shares(&short_share_bytes, longest, false);
            let mut all_shares: Vec<Vec<u8>> = vec![];
            all_shares.extend(short_share_bytes.iter().cloned());
            all_shares.extend(shares_bytes);
            handle_secret_result(sbase::rebuild_secret(all_shares))
        }
    } else if aead {
        handle_secret_result(saead::rebuild_secret(shares_bytes))
    } else {
        handle_secret_result(sbase::rebuild_secret(shares_bytes))
    }
}

fn handle_secret_result<E: std::fmt::Display>(
    result: Result<Vec<u8>, E>,
) -> Result<String, String> {
    match result {
        Ok(data) => match from_utf8(&data) {
            Ok(s) => Ok(s.to_string()),
            Err(_) => match Mnemonic::from_entropy(&data) {
                Ok(m) => Ok(m.to_string()),
                Err(_) => Err("Invalid Secret".to_string()),
            },
        },
        Err(e) => Err(e.to_string()),
    }
}

fn check_suffixes_same(shares_bytes: &[Vec<u8>]) -> bool {
    let mut reference_suffix: &[u8] = &[];
    for share in shares_bytes {
        if share.len() >= 34 {
            if reference_suffix.is_empty() {
                reference_suffix = &share[34..];
            } else if &share[34..] != reference_suffix {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}

fn hash_short_shares(short_shares: &Vec<Vec<u8>>, longlen: usize, aead: bool) -> Vec<Vec<u8>> {
    let sharelen: usize = if aead { 32 + 1 } else { longlen };

    let mut short_shares_hashed = Vec::new();
    for share in short_shares {
        let mut hasher = Shake256::default();
        hasher.update(share);
        let mut reader = hasher.finalize_xof();
        let mut share_hashed = vec![0u8; sharelen];
        reader.read(&mut share_hashed);
        short_shares_hashed.push(share_hashed);
    }

    short_shares_hashed
}
