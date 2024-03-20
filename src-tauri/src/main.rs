// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use shami_rs::base as sbase;
use shami_rs::bip39 as sbip39;

#[tauri::command]
fn verify_mnemonic(mnemonic: &str) -> bool {
    Mnemonic::parse_normalized(mnemonic).is_ok()
}

#[tauri::command]
fn build_shares_base(secret: &str, threshold: u64, totalshares: u64) -> Vec<String> {
    sbase::build_shares(secret.as_bytes(), threshold as usize, totalshares as usize)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

#[tauri::command]
fn build_shares_bip(secret: &str, threshold: u64, totalshares: u64) -> Vec<String> {
    sbip39::build_shares(secret.as_bytes(), threshold as usize, totalshares as usize)
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            verify_mnemonic,
            build_shares_base,
            build_shares_bip
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
