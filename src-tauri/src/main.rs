// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use padding::{pkcs7_pad_bip, pkcs7_unpad};
use std::collections::HashSet;
use std::str::from_utf8;

#[macro_use]
mod building_shares;
mod building_secret;
mod exporting_shares;
mod importing_shares;
mod password;
pub mod padding;

#[tauri::command]
fn verify_mnemonic(mnemonic: &str) -> bool {
    Mnemonic::parse_normalized(mnemonic).is_ok()
}

#[tauri::command]
fn calculate_bip39(base64: &str) -> String {
    let mut bytes = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();

    if (bytes.len() * 8) % 32 != 0 {
        bytes = pkcs7_pad_bip(bytes.as_slice());
    }

    Mnemonic::from_entropy(&bytes).unwrap().to_string()
}

#[tauri::command]
// bip, short, unable to decode, identical
fn check_shares(list: Vec<String>) -> Vec<(bool, bool, bool, bool)> {
    let mut shares_bytes = vec![vec![]; list.len()];
    let mut shares_props = vec![(false, false, false, false); list.len()];
    for (i, share) in list.iter().enumerate() {
        match Mnemonic::parse_normalized(share.as_str()) {
            Ok(m) => {
                shares_bytes[i] = pkcs7_unpad(m.to_entropy().to_vec());
                shares_props[i].0 = true;
            }
            Err(_) => match URL_SAFE_NO_PAD.decode(share) {
                Ok(b) => {
                    shares_bytes[i] = b;
                }
                Err(_) => shares_props[i].2 = true,
            },
        }
    }

    let longest = shares_bytes
        .iter()
        .map(|element| element.len())
        .max()
        .unwrap_or(0);

    let mut unique_shares: HashSet<Vec<u8>> = HashSet::new();

    for (i, share) in shares_bytes.iter_mut().enumerate() {
        if share.len() < longest {
            shares_props[i].1 = true;
        }

        if unique_shares.contains(share) {
            shares_props[i].3 = true;
        } else {
            unique_shares.insert(share.to_vec());
        }
    }

    shares_props
}

#[tauri::command]
fn tryutf8tomnemonic(utf: &str) -> String {
    match from_utf8(utf.as_bytes()) {
        Ok(s) => match Mnemonic::from_entropy(s.as_bytes()) {
            Ok(m) => m.to_string(),
            Err(_) => "".to_string(),
        },
        Err(_) => "".to_string(),
    }
}

#[tauri::command]
fn bytelength(base64: &str) -> usize {
    let bytes = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();
    bytes.len()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            verify_mnemonic,
            calculate_bip39,
            bytelength,
            check_shares,
            tryutf8tomnemonic,
            exporting_shares::exportaspem,
            exporting_shares::exportasqr,
            importing_shares::scanqr,
            importing_shares::uploadfile,
            building_shares::build_shares_base,
            building_shares::build_shares_bip,
            building_shares::build_shares_base_predefined,
            building_shares::build_shares_aead,
            building_shares::build_shares_aead_predefined,
            building_shares::build_shares_bip_aead,
            building_shares::build_shares_bip_predefined,
            building_shares::build_shares_bip_aead_predefined,
            building_shares::generate_predefined,
            building_secret::build_secret,
            password::generate_password,
            password::generate_passphrase,
            password::generate_bip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
