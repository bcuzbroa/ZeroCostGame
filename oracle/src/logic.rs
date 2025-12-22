use crate::crypto;

use challenges::*;

pub trait ChallengeVerifier {

    type Output : AsRef<[u8]>; //This type can be hashed

    fn run_code() -> Self::Output;
    fn secret_data() -> &'static [u8] ;// cyphertext

}


pub fn solve<V : ChallengeVerifier>() -> Option<String>{

    let out = V::run_code();
    let key = blake3::hash(out.as_ref()); 
    let nonce = [0x42u8; 24];
    let cipher = V::secret_data();
    let decrypted = crypto::decrypt(key.as_bytes(), &nonce, cipher)?;
    String::from_utf8(decrypted).ok()
}

pub struct Verifier0;
impl ChallengeVerifier for Verifier0{
    type Output = String;
    
    fn run_code() -> Self::Output {hello()}

    fn secret_data() -> &'static [u8] {
        &[136, 81, 139, 8, 111, 9, 11, 99, 195, 22, 59, 119, 11, 215, 110, 253, 185, 9, 150, 31, 126, 170, 30, 75, 51, 67, 25, 69, 254, 71, 88, 37, 22, 110, 24, 47, 1, 192, 79, 206, 177, 114]
    }
                        
}

pub struct Verifier1;
impl ChallengeVerifier for Verifier1{
    type Output = String;
    
    fn run_code() -> Self::Output {
        challenge_1("input".to_string())
    }



    fn secret_data() -> &'static [u8] {
        &[57, 79, 227, 77, 191, 91, 63, 11, 158, 102, 125, 229, 140, 192, 240, 157, 111, 212, 50, 31, 159, 116, 201, 130, 87, 148, 154, 37, 167, 61, 104, 150, 154, 216, 52, 94, 139, 192, 18, 195, 242, 217] 
    }
                        
}

pub struct Verifier2;

pub struct Verifier3;

pub struct Verifier4;

pub struct Verifier5;

pub struct Verifier6;

pub struct Verifier7;

pub struct Verifier8;

pub struct Verifier9;