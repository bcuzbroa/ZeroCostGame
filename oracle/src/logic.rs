use crate::crypto;

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

    /**********************************************
     *                                           /
     *                                          /
     *                WRAPPER                  /
     *                                        /
     *                                       /
     * **************************************
     */ 
    fn run_external(base_path: &str, wrapper_main: &str) -> String {

        let file_path = format!("{}/challenge{}.rs", base_path, Self::id());
        let player_code = std::fs::read_to_string(&file_path)
            .expect(&format!("Could not read {}", file_path));

        // Create a temporary source file including the player's code and our test main
        let temp_src = format!("temp_challenge{}.rs", Self::id());
        let full_code = format!("{}\n\n{}", player_code, wrapper_main);
        std::fs::write(&temp_src, full_code).unwrap();

        let bin_name = format!("./temp_bin_{}", Self::id()); //binary_name

        // I. Compiling the code to the temp directory
        let compile_output = std::process::Command::new("rustc")
            .arg(&temp_src) //path
            .arg("-o")
            .arg(&bin_name) //output name
            .output()
            .expect("Failed to run rustc");

        // II. Processing the compilation Error
        if !compile_output.status.success(){
            let err = String::from_utf8_lossy(&compile_output.stderr);
            println!("❌ Compilation Error ❌ in challenge {}:\n{}", Self::id(), err);
            std::fs::remove_file(temp_src).ok();
            std::process::exit(1);
        }

        // III. Running the binary
        let run_output = std::process::Command::new(&bin_name)
            .output()
            .expect("Failed to execute binary");

        
        // IV. Cleanup
        let _ = std::fs::remove_file(temp_src);
        let _ = std::fs::remove_file(&bin_name);

        String::from_utf8_lossy(&run_output.stdout).trim().to_string()
    }

}


pub fn solve<V : ChallengeVerifier>(path: &str) -> Option<String>{
    if !V::check_code(path){ 
        println!("Check not verified.");
        return None
    }
    let out = V::run_code(path);
    let key = blake3::hash(out.as_ref()); 
    let nonce = [0x42u8; 24];
    let cipher = V::secret_data();
    let decrypted = crypto::decrypt(key.as_bytes(), &nonce, cipher)?;
    let d_flag = String::from_utf8(decrypted).ok();
    d_flag
}
