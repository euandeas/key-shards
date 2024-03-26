use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;
use rand_core::{OsRng, RngCore};
use sha3::{
    digest::{ExtendableOutput, Update, XofReader},
    Shake256,
};
use shami_rs::aead as saead;
use shami_rs::base as sbase;
use shami_rs::bip39 as sbip39;

macro_rules! build_shares {
    ($func:path, $secret:expr, $threshold:expr, $totalshares:expr, $pad:expr) => {{
        $func(
            $secret.as_bytes(),
            $threshold as usize,
            $totalshares as usize,
            $pad,
        )
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
    }};
    ($func:path, $secret:expr, $preshares:expr, $threshold:expr, $totalshares:expr, $pad:expr) => {{
        $func(
            $secret.as_bytes(),
            $preshares,
            $threshold as usize,
            $totalshares as usize,
            $pad,
        )
        .unwrap()
        .into_iter()
        .map(|bytes| URL_SAFE_NO_PAD.encode(bytes))
        .collect()
    }};
}

pub fn hash_preshares(
    preshares: Vec<String>,
    secret: &str,
    aead: bool,
    isbip: bool,
    ispad: bool,
) -> Vec<Vec<u8>> {
    let sharelen: usize;
    if aead {
        sharelen = 32 + 1;
    } else if isbip {
        let m = match Mnemonic::parse_normalized(secret) {
            Ok(m) => m,
            Err(_) => return Vec::new(), // Return an empty vector if parsing fails
        };
        sharelen = m.to_entropy().len() + 1;
    } else if ispad {
        sharelen = (secret.len() + 31) / 32 * 32;
    } else {
        sharelen = secret.len() + 1;
    }

    let mut preshares_hashed = Vec::new();
    for share in &preshares {
        let shareparsed = match Mnemonic::parse_normalized(share) {
            Ok(m) => m.to_entropy(),
            Err(_) => return Vec::new(), // Return an empty vector if parsing fails
        };

        let mut hasher = Shake256::default();
        hasher.update(&shareparsed);
        let mut reader = hasher.finalize_xof();
        let mut share_hashed = vec![0u8; sharelen];
        reader.read(&mut share_hashed);
        preshares_hashed.push(share_hashed);
    }

    preshares_hashed
}

#[tauri::command]
pub fn build_shares_base(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    build_shares!(sbase::build_shares, secret, threshold, totalshares, pad)
}

#[tauri::command]
pub fn build_shares_bip(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    build_shares!(sbip39::build_shares, secret, threshold, totalshares, pad)
}

#[tauri::command]
pub fn build_shares_base_predefined(
    secret: &str,
    preshares: Vec<String>,
    threshold: u64,
    totalshares: u64,
    pad: bool,
) -> Vec<String> {
    let hashedpreshares = hash_preshares(preshares, secret, false, false, pad);

    build_shares!(
        sbase::build_shares_predefined,
        secret,
        hashedpreshares,
        threshold,
        totalshares,
        pad
    )
}

#[tauri::command]
pub fn build_shares_aead(secret: &str, threshold: u64, totalshares: u64, pad: bool) -> Vec<String> {
    build_shares!(saead::build_shares, secret, threshold, totalshares, pad)
}

#[tauri::command]
pub fn build_shares_aead_predefined(
    secret: &str,
    preshares: Vec<String>,
    threshold: u64,
    totalshares: u64,
    pad: bool,
) -> Vec<String> {
    let hashedpreshares = hash_preshares(preshares, secret, true, false, pad);

    build_shares!(
        saead::build_shares_predefined,
        secret,
        hashedpreshares,
        threshold,
        totalshares,
        pad
    )
}

#[tauri::command]
pub fn build_shares_bip_aead(
    secret: &str,
    threshold: u64,
    totalshares: u64,
    pad: bool,
) -> Vec<String> {
    build_shares!(
        sbip39::build_shares_aead,
        secret,
        threshold,
        totalshares,
        pad
    )
}

#[tauri::command]
pub fn build_shares_bip_predefined(
    secret: &str,
    preshares: Vec<String>,
    threshold: u64,
    totalshares: u64,
    pad: bool,
) -> Vec<String> {
    let hashedpreshares = hash_preshares(preshares, secret, false, true, pad);

    build_shares!(
        sbip39::build_shares_predefined,
        secret,
        hashedpreshares,
        threshold,
        totalshares,
        pad
    )
}

#[tauri::command]
pub fn build_shares_bip_aead_predefined(
    secret: &str,
    preshares: Vec<String>,
    threshold: u64,
    totalshares: u64,
    pad: bool,
) -> Vec<String> {
    let hashedpreshares = hash_preshares(preshares, secret, true, true, pad);

    build_shares!(
        sbip39::build_shares_aead_predefined,
        secret,
        hashedpreshares,
        threshold,
        totalshares,
        pad
    )
}

#[tauri::command]
pub fn generate_predefined(
    secret: &str,
    othershare: &str,
    aead: bool,
    isbip: bool,
    ispad: bool,
) -> String {
    let sharelen;
    if aead {
        sharelen = 32 + 1;
    } else if isbip {
        let m = match Mnemonic::parse_normalized(secret) {
            Ok(m) => m,
            Err(_) => return "".to_string(),
        };
        sharelen = m.to_entropy().len() + 1;
    } else if ispad {
        sharelen = (secret.len() + 31) / 32 * 32;
    } else {
        sharelen = secret.len() + 1;
    }

    let mut othershare_hashed = vec![0u8; sharelen];

    if !othershare.is_empty() {
        let others = match Mnemonic::parse_normalized(othershare) {
            Ok(m) => m.to_entropy(),
            Err(_) => return "".to_string(),
        };

        let mut hasher = Shake256::default();
        hasher.update(&others);
        let mut reader = hasher.finalize_xof();
        reader.read(&mut othershare_hashed);
    }

    let mut share = vec![0u8; 4];
    loop {
        OsRng.fill_bytes(&mut share);

        let mut hasher = Shake256::default();
        hasher.update(&share);
        let mut reader = hasher.finalize_xof();
        let mut share_hashed = vec![0u8; sharelen];
        reader.read(&mut share_hashed);

        if share_hashed[0] == 0 || othershare_hashed[0] == share_hashed[0] {
            continue;
        }
        break;
    }

    Mnemonic::from_entropy(&share).unwrap().to_string()
}
