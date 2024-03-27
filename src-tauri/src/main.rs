// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use block_padding::{Pkcs7, RawPadding};
use opencv::{core, highgui, imgproc, objdetect, prelude::*, types, videoio, Result};
use pem_rfc7468::LineEnding;
use std::fs::File;
use std::io::prelude::*;
use tauri::api::dialog::FileDialogBuilder;

#[macro_use]
mod building_shares;

#[tauri::command]
fn verify_mnemonic(mnemonic: &str) -> bool {
    Mnemonic::parse_normalized(mnemonic).is_ok()
}

fn pkcs7_pad_bip(msg: &[u8]) -> Vec<u8> {
    let len = msg.len();
    let block_len = (((len * 8) + 31) / 32 * 32) / 8;

    if block_len == len {
        return msg.to_vec();
    }

    let mut block = vec![0; block_len];
    block[..len].copy_from_slice(msg);
    Pkcs7::raw_pad(block.as_mut_slice(), len);
    block
}

fn pkcs7_unpad_bip(input: Vec<u8>) -> Vec<u8> {
    match Pkcs7::raw_unpad(input.as_slice()) {
        Ok(v) => v.to_vec(),
        Err(_) => input.to_vec(),
    }
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
fn exportaspem(base64: &str) {
    let share = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();
    
    FileDialogBuilder::new()
        .add_filter("PEM", &["pem"])
        .set_file_name("keyshard.pem")
        .save_file(move |file_path| {
            if let Some(path) = file_path {
                let mut file = match File::create(path) {
                    Ok(f) => f,
                    Err(_) => return,
                };

                let encoded_pem = match pem_rfc7468::encode_string(
                    "KEY SHARD",
                    LineEnding::default(),
                    share.as_slice(),
                ) {
                    Ok(pem) => pem,
                    Err(_) => return,
                };

                file.write_all(encoded_pem.as_bytes()).unwrap();
            }
        });
}

#[tauri::command]
fn exportasqr(base64: &str) {
    let share = base64.to_string();
    FileDialogBuilder::new()
        .add_filter("JPEG Image", &["jpg", "jpeg"])
        .set_file_name("keyshard.jpg")
        .save_file(move |file_path| {
            if let Some(path) = file_path {
                let mut qr_encoder = match objdetect::QRCodeEncoder::create_def() {
                    Ok(e) => e,
                    Err(_) => return,
                };

                let mut qr_code = Mat::default();
                match qr_encoder.encode(share.as_str(), &mut qr_code) {
                    Ok(_) => (),
                    Err(_) => return,
                };

                let mut resized_qr_code = Mat::default();
                match opencv::imgproc::resize(
                    &qr_code,  // Source image
                    &mut resized_qr_code,  // Destination image
                    opencv::core::Size { width: 800, height: 800 }, // New dimensions
                    0.0, // No scaling factor
                    0.0, // No scaling factor
                    opencv::imgproc::INTER_NEAREST, // Interpolation method
                ) {
                    Ok(_) => (),
                    Err(_) => return,
                };
                

                let truepath = match path.to_str() {
                    Some(p) => p,
                    None => return,
                };

                let _ = opencv::imgcodecs::imwrite(truepath, &resized_qr_code, &opencv::types::VectorOfi32::new());
            }
        });
}

#[tauri::command]
fn bytelength(base64: &str) -> usize {
    let bytes = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();
    return bytes.len();
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            verify_mnemonic,
            calculate_bip39,
            exportaspem,
            exportasqr,
            bytelength,
            building_shares::build_shares_base,
            building_shares::build_shares_bip,
            building_shares::build_shares_base_predefined,
            building_shares::build_shares_aead,
            building_shares::build_shares_aead_predefined,
            building_shares::build_shares_bip_aead,
            building_shares::build_shares_bip_predefined,
            building_shares::build_shares_bip_aead_predefined,
            building_shares::generate_predefined
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
