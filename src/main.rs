use openssl::aes::{aes_ige, AesKey, KeyError};
use openssl::symm::Mode;

#[derive(Debug)]
enum SsapError {
    InvalidKey,
    InvalidCiphertext,
}

struct Ssap {
    // TODO: Add fields here
}

fn main() {
    let key = b"secret".to_vec();
    let plaintext = b"Ciaone".to_vec();
    // TODO: generate random first iv
    let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F".to_vec();
    println!("starting iv: {:?}", iv);
    println!(
        "Ciphertext: {:?}",
        encrypt_aes128(plaintext, key, iv).unwrap()
    );
}

/// Encrypt an arbitrary number of bytes into a cypher text with the same
/// length using the provided key for encryption.
///
/// # Arguments
/// * `plaintext` - The plaintext to encrypt
/// * `key` - The key to use for encryption
/// * `iv` - The initial vector to use for encryption
/// # Returns
/// * The ciphertext
fn encrypt_aes128(
    mut plaintext: Vec<u8>,
    mut key: Vec<u8>,
    mut iv: Vec<u8>,
) -> Result<Vec<u8>, SsapError> {
    if plaintext.len() % 16 != 0 {
        plaintext.resize(plaintext.len() / 16 * 16 + 16, 0 as u8);
    }
    if key.len() % 16 != 0 {
        key.resize(16, 0 as u8);
    }

    let mut ciphertext: Vec<u8> = Vec::new();
    for i in (0..plaintext.len()).step_by(16) {
        let block = &plaintext[i..i + 16];
        let aes_key_r = AesKey::new_encrypt(key.as_ref());
        if aes_key_r.is_err() {
            return Err(SsapError::InvalidKey);
        }
        let mut out: &mut [u8] = &mut [0; 16];
        aes_ige(block, &mut out, &aes_key_r.unwrap(), &mut iv, Mode::Encrypt);
        ciphertext.append(&mut Vec::from(out));
    }

    return Ok(ciphertext);
}

/// Decrypt ciphertext into plaintext using the provided key for decryption.
/// The ciphertext must be a multiple of 16 bytes, otherwise an error is
/// returned.
///
/// # Arguments
/// * `ciphertext` - The ciphertext to decrypt
/// * `key` - The key to use for decryption
/// * `start_iv` - The initial vector to use for decryption
/// # Returns
/// * The plaintext
fn decrypt_aes128(
    mut ciphertext: Vec<u8>,
    mut key: Vec<u8>,
    mut start_iv: Vec<u8>,
) -> Result<Vec<u8>, SsapError> {

    if ciphertext.len() % 16 != 0 {
        return Err(SsapError::InvalidCiphertext);
    }
    if key.len() % 16 != 0 {
        key.resize(16, 0 as u8);
    }

    let mut plaintext: Vec<u8> = Vec::new();
    for i in (0..ciphertext.len()).step_by(16) {
        let block = &ciphertext[i..i + 16];
        let aes_key_r = AesKey::new_decrypt(key.as_ref());
        if aes_key_r.is_err() {
            return Err(SsapError::InvalidKey);
        }
        let mut out: &mut [u8] = &mut [0; 16];
        aes_ige(block, &mut out, &aes_key_r.unwrap(), &mut start_iv, Mode::Decrypt);
        plaintext.append(&mut Vec::from(out));
    }

    return Ok(plaintext);
}

fn help() {
    // TODO
}

fn store() {
    // TODO
}

fn read_disk() {
    // TODO
}

fn new_iv() {
    // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F".to_vec();
        let plaintext =
            b"\x12\x34\x56\x78\x90\x12\x34\x56\x12\x34\x56\x78\x90\x12\x34\x56".to_vec();
        let mut iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F".to_vec();

        let output = encrypt_aes128(plaintext, key, iv);
        assert_eq!(
            output.unwrap(),
            b"\xa6\xad\x97\x4d\x5c\xea\x1d\x36\xd2\xf3\x67\x98\x09\x07\xed\x32"
        );
    }

    #[test]
    fn test_decrypt() {
        let key = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F".to_vec();
        let ciphertext =
            b"\xa6\xad\x97\x4d\x5c\xea\x1d\x36\xd2\xf3\x67\x98\x09\x07\xed\x32".to_vec();
        let iv = b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0A\x0B\x0C\x0D\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1B\x1C\x1D\x1E\x1F".to_vec();

        let output = decrypt_aes128(ciphertext, key, iv);
        assert_eq!(
            output.unwrap(),
            b"\x12\x34\x56\x78\x90\x12\x34\x56\x12\x34\x56\x78\x90\x12\x34\x56"
        );
    }
}
