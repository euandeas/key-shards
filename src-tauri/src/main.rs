// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use shami_rs::aead as saead;
use shami_rs::base as sbase;
use shami_rs::bip39 as sbip39;

#[tauri::command]
fn build_shares_base(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    sbase::build_shares(secret.as_bytes(), threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_bip(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    sbip39::build_shares(secret.as_bytes(), threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_base_predefined(secret: &str, pre_shares: Vec<String>, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    let pre_shares_bytes: Vec<Vec<u8>> = pre_shares
        .into_iter()
        .map(|share| URL_SAFE_NO_PAD.decode(share.as_bytes()).unwrap().to_vec())
        .collect();
    
    sbase::build_shares_predefined(secret.as_bytes(), pre_shares_bytes, threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_aead(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    saead::build_shares(secret.as_bytes(), threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_aead_predefined(secret: &str, pre_shares: Vec<String>, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    let pre_shares_bytes: Vec<Vec<u8>> = pre_shares
        .into_iter()
        .map(|share| URL_SAFE_NO_PAD.decode(share.as_bytes()).unwrap().to_vec())
        .collect();
    
    saead::build_shares_predefined(secret.as_bytes(), pre_shares_bytes, threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_bip_aead(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    sbip39::build_shares_aead(secret.as_bytes(), threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_bip_predefined(secret: &str, pre_shares: Vec<String>, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    let pre_shares_bytes: Vec<Vec<u8>> = pre_shares
        .into_iter()
        .map(|share| URL_SAFE_NO_PAD.decode(share.as_bytes()).unwrap().to_vec())
        .collect();
    
    sbip39::build_shares_predefined(secret.as_bytes(), pre_shares_bytes, threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_bip_aead_predefined(secret: &str, pre_shares: Vec<String>, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    let pre_shares_bytes: Vec<Vec<u8>> = pre_shares
        .into_iter()
        .map(|share| URL_SAFE_NO_PAD.decode(share.as_bytes()).unwrap().to_vec())
        .collect();
    
    sbip39::build_shares_aead_predefined(secret.as_bytes(), pre_shares_bytes, threshold as usize, totalshares as usize, pad)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn verify_mnemonic(mnemonic: &str) -> bool {
    Mnemonic::parse_normalized(mnemonic).is_ok()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            verify_mnemonic,
            build_shares_base,
            build_shares_bip,
            build_shares_base_predefined,
            build_shares_aead,
            build_shares_aead_predefined,
            build_shares_bip_aead,
            build_shares_bip_predefined,
            build_shares_bip_aead_predefined
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
