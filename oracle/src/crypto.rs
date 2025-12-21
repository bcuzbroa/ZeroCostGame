use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
};

pub fn decrypt(
    key: &[u8; 32],
    nonce: &[u8; 24],
    cipher: &[u8],
) -> Option<Vec<u8>> {
    let cipher_algo = XChaCha20Poly1305::new(key.into());
    cipher_algo.decrypt(nonce.into(), cipher).ok()
}
