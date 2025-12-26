use crate::crypto;

///Each Challenge is represented using this trait
///The output can differ and has to respect AsRef<[u8]>
///The cryptographic key is derived from the Output = run_code()
pub trait ChallengeVerifier {

    type Output : AsRef<[u8]>; //This type can be hashed

    fn run_code() -> Self::Output;
    fn check_code() -> bool;
    // Default implementation for compilation purposes
    // &[0] is later replaced with the encrypted flag   
    fn secret_data() -> &'static [u8]{&[0]}

}


pub fn solve<V : ChallengeVerifier>() -> Option<String>{
    if !V::check_code(){ 
        println!("Check not verified.");
        return None
    }
    let out = V::run_code();
    let key = blake3::hash(out.as_ref()); 
    let nonce = [0x42u8; 24];
    let cipher = V::secret_data();
    let decrypted = crypto::decrypt(key.as_bytes(), &nonce, cipher)?;
    let d_flag = String::from_utf8(decrypted).ok();
    d_flag
}
