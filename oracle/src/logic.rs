use crate::crypto;
use crate::wrapper::wrapper;

///Each Challenge is represented using this trait
///The output can differ and has to respect AsRef<[u8]>
///The cryptographic key is derived from the Output = run_code()
pub trait ChallengeVerifier {

    type Output : AsRef<[u8]>; //This type can be hashed

    fn id() -> &'static str;
    fn run_code(path : &str) -> Self::Output;
    fn check_code(path: &str) -> bool;
    // Default implementation for compilation purposes
    // &[0] is later replaced with the encrypted flag   
    fn secret_data() -> &'static [u8]{&[0]}

    fn run_external(path: &str, wrapper_main: &str) -> String {
        wrapper(path, wrapper_main, Self::id())
    }
}

pub fn solve<V : ChallengeVerifier>(path: &str) -> Option<String>{
    if !V::check_code(path){ 
        println!("Tests not passed.");
        return None
    }
    let out = V::run_code(path);
    let key = blake3::hash(out.as_ref()); 
    let nonce = [0x42u8; 24]; //Static
    let cipher = V::secret_data();
    let decrypted = crypto::decrypt(key.as_bytes(), &nonce, cipher)?;
    let d_flag = String::from_utf8(decrypted).ok();
    d_flag
}