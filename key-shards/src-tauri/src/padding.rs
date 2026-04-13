use block_padding::{Pkcs7, RawPadding};

pub fn pkcs7_pad_bip(msg: &[u8]) -> Vec<u8> {
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

pub fn pkcs7_unpad(input: Vec<u8>) -> Vec<u8> {
    match Pkcs7::raw_unpad(input.as_slice()) {
        Ok(v) => v.to_vec(),
        Err(_) => input.to_vec(),
    }
}
