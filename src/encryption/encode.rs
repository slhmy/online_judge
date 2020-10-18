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

use argon2rs::argon2i_simple;

pub fn make_salt() -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    const PASSWORD_LEN: usize = 128;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
        .map(|_| {
            let idx = rng.gen_range(0, CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}

pub fn make_hash(password: &str, salt: &str) -> [u8; argon2rs::defaults::LENGTH] {
    argon2i_simple(password, salt)
}