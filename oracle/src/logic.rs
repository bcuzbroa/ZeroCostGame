use crate::crypto;

use challenges::hello;

pub trait ChallengeVerifier {

    type Output : AsRef<[u8]>; //This type can be hashed

    fn run_code() -> Self::Output;
    fn check_result(out: &Self::Output) -> bool;
    fn secret_data() -> (&'static [u8]) ;//nonce, cyphertext

}


pub fn solve<T : ChallengeVerifier>() -> Option<String>{

    let out = T::run_code();
    let key = blake3::hash(out.as_ref()); 
    let nonce = [0x42u8; 24];
    let cipher = T::secret_data();
    let decrypted = crypto::decrypt(key.as_bytes(), &nonce, cipher)?;
    String::from_utf8(decrypted).ok()
}

pub struct Verifier0;
impl ChallengeVerifier for Verifier0{
    type Output = String;
    
    fn run_code() -> Self::Output {hello()}

    fn check_result(out: &Self::Output) -> bool {
        out == "Hello world !"
    }

    fn secret_data() -> &'static [u8] {
        &[136, 81, 139, 8, 111, 9, 11, 99, 195, 22, 59, 119, 11, 215, 110, 253, 185, 9, 150, 31, 126, 170, 30, 75, 51, 67, 25, 69, 254, 71, 88, 37, 22, 110, 24, 47, 1, 192, 79, 206, 177, 114]
    }
                        
}