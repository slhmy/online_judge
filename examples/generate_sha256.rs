use sha2::Sha256;
use digest::Digest;
use hex::ToHex;

fn hash_token<D: Digest>(key: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(key.as_bytes());
    output.copy_from_slice(&hasher.finalize())
}

fn main() {
    let mut buf = [0u8; 32];
    hash_token::<Sha256>("YOUR_TOKEN_HERE", &mut buf);
    println!("hex_string: {}", (&buf[..]).to_vec().encode_hex::<String>());
}