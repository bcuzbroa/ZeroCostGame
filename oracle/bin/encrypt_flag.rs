use chacha20poly1305::{
    aead::{Aead, KeyInit},
    XChaCha20Poly1305,
};
use blake3;
use csv::WriterBuilder;
use std::{env,
    error::Error,
    fs::OpenOptions,
    path::PathBuf
};

use oracle::logic::*;
use oracle::verifier::*;

fn main(){


    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: oracle <id_challenge> <challenge_dir_path>");
        return;
    }

    let challenged_id = match args.get(1){
        Some(id) => id.as_str(),
        None => {
            println!("Missing argument!\nUsage: cargo run --bin ecrypt_flag -- <id> <flag>");
            return;
        }
    };

    let flag = match args.get(2){
        Some(f) => f.as_str(),
        None => {
            println!("Missing argument!\nUsage: cargo run --bin ecrypt_flag -- <id> <flag>");
            return;
        }
    };
    
    let base_path = match args.get(3){
        Some(path) => path.as_str(),
        None => {
            println!("Missing argument!\nUsage: cargo run --bin ecrypt_flag -- <id> <flag>");
            return;
        }
    };
    

    let ciphertext = match challenged_id {
        "0" => crypt_flag::<Verifier0>(flag, base_path),
        "1" => crypt_flag::<Verifier1>(flag, base_path),
        "2" => crypt_flag::<Verifier2>(flag, base_path),
        "3" => crypt_flag::<Verifier3>(flag, base_path),
        "4" => crypt_flag::<Verifier4>(flag, base_path),
        "5" => crypt_flag::<Verifier5>(flag, base_path),
        "6" => crypt_flag::<Verifier6>(flag, base_path),
        "7" => crypt_flag::<Verifier7>(flag, base_path),
        "8" => crypt_flag::<Verifier8>(flag, base_path),
        /*
        "9" => crypt_flag::<Verifier9>(flag),
         */
        _ => {
            eprintln!("Unknown challenge id");
            return;
        }
    };

    write_flag_to_csv(challenged_id, flag).unwrap();
    write_encrypted_flag_to_csv(challenged_id, ciphertext).unwrap();
    
}



fn crypt_flag<V : ChallengeVerifier>(flag :&str, path: &str) -> Vec<u8>{

    let out = V::run_code(path);
    let key = blake3::hash(out.as_ref());
    let nonce = [0x42u8; 24]; //Here is a little vulnerabily, let them use it ! (static nonce)
    let cipher = XChaCha20Poly1305::new(key.as_bytes().into());
    let ciphertext = cipher.encrypt(&nonce.into(), flag.as_ref()).unwrap();

    ciphertext
}

fn write_encrypted_flag_to_csv(id : &str, ciphertext : Vec<u8>) -> Result<(), Box<dyn Error>>{

    let base_path = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(base_path);
    file_path.push("src");
    file_path.push("flags");
    file_path.push("encrypted_flag_list.csv");
    
    let file = OpenOptions::new()
        .write(true)      // Authorise writing
        .append(true)     // Set the cursor at the end of the file
        .create(true)     // Creates the file if it doesnt exist
        .open(file_path)?;


    let mut wtr = WriterBuilder::new()
        .delimiter(b';')
        .from_writer(file);

    let cipher_string = format!("{:?}", ciphertext);
    wtr.write_record(&[id, &cipher_string])?;

    wtr.flush()?;

    Ok(())
}

fn write_flag_to_csv(id : &str, flag :&str) -> Result<(), Box<dyn Error>>{

    let base_path = env!("CARGO_MANIFEST_DIR");
    let mut file_path = PathBuf::from(base_path);
    file_path.push("src");
    file_path.push("flags");
    file_path.push("flag_list.csv");

    let file = OpenOptions::new()
        .write(true)      
        .append(true)    
        .create(true)  
        .open(file_path)?;

    let mut wtr = WriterBuilder::new()
        .delimiter(b',')
        .from_writer(file);

    wtr.write_record(&[id, flag])?;

    wtr.flush()?;

    Ok(())
}
