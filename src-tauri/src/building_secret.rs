use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use bip39::Mnemonic;

#[tauri::command]
fn build_secret(list: Vec<String>) -> String{
    let mut shares_bytes = vec![vec![]; list.len()];
    for (i, share) in list.iter().enumerate() {
        match Mnemonic::parse_normalized(share.as_str()){
            Ok(m) => 
            {
                shares_bytes[i] = m.to_entropy().to_vec();
            },
            Err(_) => match URL_SAFE_NO_PAD.decode(share) {
                Ok(b) => {
                    shares_bytes[i] = b;
                },
                Err(_) => shares_bytes[i] = share.as_bytes().to_vec(),
            }
        }
    }

    // AEAD identifiable by same ciphertext but different start
    // attempt bip rebuild first and then normal

    "Hello World".to_string()
}