use challenges::hello;
use blake3;

use crate::crypto;
use crate::flags::flag0;

pub fn solve_and_get_flag() -> Option<String> {
    let out = hello();

    if out != "Hello world !" {
        return None;
    }

    let key = blake3::hash(out.as_bytes());

    let decrypted = crypto::decrypt(
        key.as_bytes(),
        &flag0::NONCE,
        &flag0::CIPHER,
    )?;

    String::from_utf8(decrypted).ok()
}

