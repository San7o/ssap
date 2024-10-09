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
        ssap::crypto::encrypt_aes128_ige(plaintext, key, iv).unwrap()
    );
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

