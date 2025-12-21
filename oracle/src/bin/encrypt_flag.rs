use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
};
use blake3;

fn main() {
    let key = b"Hello world !";
    let flag = b"FLAG{NIC3_FiR5t_Ch4113n9E}";

    // clé identique à celle dérivée par solve()
    let key = blake3::hash(key);
    
    let nonce = [0x42u8; 24]; // fix nonce is ok offline
    
}

fn crypt_flag(key : &str, flag: &[u8]) -> ([u8; 24], Vec<u8>){

    let nonce = [0x42u8; 24];
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    let ciphertext = cipher.encrypt(&nonce.into(), flag.as_ref()).unwrap();
    
    println!("nonce  = {:?};", nonce);
    println!("cipher = {:?};", ciphertext);
    (nonce, ciphertext)
}