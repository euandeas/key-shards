use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use opencv::{highgui, imgcodecs, objdetect, prelude::*, types, videoio, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tauri::api::dialog::blocking;

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
pub async fn uploadfile() -> Result<String, String> {
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
pub fn scanqr() -> Result<String, String> {
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
