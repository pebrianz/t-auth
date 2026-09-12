use aes_gcm::{
    Aes256Gcm, Key,
    aead::{self, Aead, AeadCore, KeyInit, Nonce},
};
use argon2::Argon2;

pub fn kdf(argon2: Argon2, password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key: [u8; 32] = [0u8; 32];
    argon2
        .hash_password_into(password.as_bytes(), salt, &mut key)
        .unwrap();
    key
}

pub fn encrypt(key: &[u8; 32], data: &[u8]) -> (Vec<u8>, Nonce<Aes256Gcm>) {
    let key: &Key<Aes256Gcm> = key.into();
    let chiper = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut aead::OsRng);
    (chiper.encrypt(&nonce, data).expect("encrypt failed"), nonce)
}

pub fn decrypt(key: &[u8], nonce: &[u8], ciphertext: &[u8]) -> Option<Vec<u8>> {
    let key: &Key<Aes256Gcm> = key.into();
    let nonce: &Nonce<Aes256Gcm> = nonce.into();
    let chiper = Aes256Gcm::new(key);
    chiper.decrypt(nonce, ciphertext).ok()
}
