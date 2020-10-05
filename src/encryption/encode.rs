use sha2::Sha256;
use digest::Digest;
use hex::ToHex;

fn hash_token<D: Digest>(key: &str, output: &mut [u8]) {
    let mut hasher = D::new();
    hasher.update(key.as_bytes());

    output.copy_from_slice(&hasher.finalize())
}

pub fn sha256_token(key: &str) -> String {
    let mut buf = [0u8; 32];
    hash_token::<Sha256>(key, &mut buf);

    (&buf[..]).to_vec().encode_hex::<String>()
}