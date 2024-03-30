// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use block_padding::{Pkcs7, RawPadding};
use opencv::{highgui, imgcodecs, objdetect, prelude::*, types, videoio, Result};
use pem_rfc7468::LineEnding;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tauri::api::dialog::{blocking, FileDialogBuilder};

#[macro_use]
mod building_shares;
mod building_secret;

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

#[tauri::command]
fn calculate_bip39(base64: &str) -> String {
    let mut bytes = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();

    if (bytes.len() * 8) % 32 != 0 {
        bytes = pkcs7_pad_bip(bytes.as_slice());
    }

    Mnemonic::from_entropy(&bytes).unwrap().to_string()
}

#[tauri::command]
// bip, short, unable to decode
fn check_shares(list: Vec<String>) -> Vec<(bool, bool, bool)> {
    let mut shares_bytes = vec![vec![]; list.len()];
    let mut shares_props = vec![(false, false, false); list.len()];
    for (i, share) in list.iter().enumerate() {
        match Mnemonic::parse_normalized(share.as_str()) {
            Ok(m) => {
                shares_bytes[i] = m.to_entropy().to_vec();
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

    for (i, share) in shares_bytes.iter_mut().enumerate() {
        if share.len() < longest {
            shares_props[i].1 = true;
        }
    }

    shares_props
}

#[tauri::command]
fn exportaspem(base64: &str) {
    let share = URL_SAFE_NO_PAD.decode(base64.as_bytes()).unwrap();

    FileDialogBuilder::new()
        .add_filter("PEM Files", &["pem", "key"])
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
                    &qr_code,             // Source image
                    &mut resized_qr_code, // Destination image
                    opencv::core::Size {
                        width: 800,
                        height: 800,
                    }, // New dimensions
                    0.0,                  // No scaling factor
                    0.0,                  // No scaling factor
                    opencv::imgproc::INTER_NEAREST, // Interpolation method
                ) {
                    Ok(_) => (),
                    Err(_) => return,
                };

                let truepath = match path.to_str() {
                    Some(p) => p,
                    None => return,
                };

                let _ = opencv::imgcodecs::imwrite(
                    truepath,
                    &resized_qr_code,
                    &opencv::types::VectorOfi32::new(),
                );
            }
        });
}

#[tauri::command]
fn scanqr() -> Result<String, String> {
    let qr_detector = match objdetect::QRCodeDetector::default() {
        Ok(d) => d,
        Err(_) => return Err("QR Detection Error".to_string()),
    };
    let mut res = types::VectorOfPoint::new();
    let mut camera = match videoio::VideoCapture::new(0, videoio::CAP_ANY) {
        Ok(d) => d,
        Err(_) => return Err("Camera Error".to_string()),
    };
    let mut img = Mat::default();
    let mut recqr = Mat::default();
    match highgui::named_window(
        "QR Capture",
        highgui::WINDOW_AUTOSIZE | highgui::WINDOW_GUI_NORMAL,
    ) {
        Ok(_) => (),
        Err(_) => return Err("GUI Error".to_string()),
    };
    loop {
        match camera.read(&mut img) {
            Ok(_) => (),
            Err(_) => return Err("Camera Error".to_string()),
        };
        let ret = match qr_detector.detect_and_decode(&img, &mut res, &mut recqr) {
            Ok(r) => r,
            Err(_) => return Err("QR Detection Error".to_string()),
        };
        let s = String::from_utf8_lossy(&ret);

        if !ret.is_empty() {
            highgui::destroy_window("QR Capture").unwrap();
            return Ok(s.to_string());
        }

        match highgui::imshow("QR Capture", &img) {
            Ok(_) => (),
            Err(_) => return Err("GUI Error".to_string()),
        };

        let _ = highgui::wait_key(1);

        match highgui::get_window_property("QR Capture", highgui::WND_PROP_VISIBLE) {
            Ok(v) => {
                if v == 0.0 {
                    break;
                }
            }
            Err(_) => return Err("GUI Error".to_string()),
        };
    }
    Err("No QR code detected".to_string())
}

fn handle_image_file(path: &str) -> Result<String, String> {
    let image = match imgcodecs::imread(path, imgcodecs::IMREAD_ANYCOLOR) {
        Ok(i) => i,
        Err(_) => return Err("Unable To Read Image".to_string()),
    };
    let qr_detector = match objdetect::QRCodeDetector::default() {
        Ok(d) => d,
        Err(_) => return Err("QR Detection Error".to_string()),
    };

    let mut res = types::VectorOfPoint::new();
    let mut recqr = Mat::default();

    let ret = match qr_detector.detect_and_decode(&image, &mut res, &mut recqr) {
        Ok(r) => r,
        Err(_) => return Err("QR Detection Error".to_string()),
    };
    let s = String::from_utf8_lossy(&ret);

    if !ret.is_empty() {
        return Ok(s.to_string());
    }

    Err("No QR code detected".to_string())
}

fn handle_pem_file(path: &str) -> Result<String, String> {
    let mut contents: Vec<u8> = vec![];
    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return Err("Unable To Read File".to_string()),
    };

    match file.read_to_end(&mut contents) {
        Ok(_) => (),
        Err(_) => return Err("Unable To Read File".to_string()),
    };

    let (type_label, data) = match pem_rfc7468::decode_vec(contents.as_slice()) {
        Ok((t, d)) => (t, d),
        Err(_) => return Err("Unable To Decode PEM".to_string()),
    };

    if type_label != "KEY SHARD" {
        return Err("Invalid PEM Type".to_string());
    }

    let base64 = URL_SAFE_NO_PAD.encode(data.as_slice());
    Ok(base64)
}

#[tauri::command]
async fn uploadfile() -> Result<String, String> {
    let file_path = blocking::FileDialogBuilder::new()
        .add_filter("PEM Files", &["pem", "key", "txt"])
        .add_filter("Image Files", &["jpg", "png", "bmp"])
        .pick_file();
    if let Some(path) = file_path {
        let extension = Path::new(&path)
            .extension()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("");

        if extension.eq_ignore_ascii_case("jpg")
            || extension.eq_ignore_ascii_case("png")
            || extension.eq_ignore_ascii_case("bmp")
        {
            // Handle image file
            return handle_image_file(path.as_os_str().to_str().unwrap());
        } else if extension.eq_ignore_ascii_case("pem")
            || extension.eq_ignore_ascii_case("key")
            || extension.eq_ignore_ascii_case("txt")
        {
            // Handle PEM file
            return handle_pem_file(path.as_os_str().to_str().unwrap());
        } else {
            // This case should not occur since the file dialog
            // is filtered to only allow image and PEM/KEY/TXT files.
            return Err(format!("Unsupported file format: {}", extension));
        }
    }

    Err("No File Picked.".to_string())
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
            exportaspem,
            exportasqr,
            bytelength,
            scanqr,
            uploadfile,
            check_shares,
            building_shares::build_shares_base,
            building_shares::build_shares_bip,
            building_shares::build_shares_base_predefined,
            building_shares::build_shares_aead,
            building_shares::build_shares_aead_predefined,
            building_shares::build_shares_bip_aead,
            building_shares::build_shares_bip_predefined,
            building_shares::build_shares_bip_aead_predefined,
            building_shares::generate_predefined,
            building_secret::build_secret
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
