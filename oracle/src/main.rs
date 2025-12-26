mod crypto;
mod logic;
mod verifier;

use crate::verifier::*;
use crate::logic::{ChallengeVerifier, solve};
use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: oracle <id_challenge> <challenge_dir_path>");
        return;
    }
    let challenge_id = &args[1];
    let base_path: &str = &args[2];
    // Verify the directory exists before proceeding
    if !std::path::Path::new(base_path).exists() {
        println!("Error: The directory '{}' does not exist.", base_path);
        return;
    }

    match challenge_id.as_str() {
        "0" => run_challenge::<Verifier0>("Sanity Check", base_path),
        "1" => run_challenge::<Verifier1>("Ownership", base_path),
        "2" => run_challenge::<Verifier2>("Borrowing", base_path),
        "3" => run_challenge::<Verifier3>("Optionnal", base_path),
        "4" => run_challenge::<Verifier4>("Result", base_path),
        "5" => run_challenge::<Verifier5>("Iterator", base_path),
        /*
        "6" => run_challenge::<Verifier6>(""),
        "7" => run_challenge::<Verifier7>(""),
        "8" => run_challenge::<Verifier8>(""),
        "9" => run_challenge::<Verifier9>(""),
        */
        _ => println!("Challenge id {} does not exist yet", challenge_id)
    }
}

fn run_challenge<V: ChallengeVerifier>(name: &str, path: &str){
    match solve::<V>(path){ //TURBOFISH
        Some(flag) => println!("✨ {} ✨ verified: {}", name, flag),
        None  => println!("❌❌ {}: NOPE ❌❌ Have you even tried ??", name)
    }
}