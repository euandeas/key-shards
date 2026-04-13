use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use opencv::{objdetect, prelude::*};
use pem_rfc7468::LineEnding;
use std::fs::File;
use std::io::prelude::*;
use tauri::api::dialog::FileDialogBuilder;

#[tauri::command]
pub fn exportaspem(base64: &str) {
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
pub fn exportasqr(base64: &str) {
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
