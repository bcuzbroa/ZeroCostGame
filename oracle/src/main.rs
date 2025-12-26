mod crypto;
mod logic;
mod verifier;

use crate::verifier::*;
use crate::logic::{ChallengeVerifier, solve};

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();
    let challenged_id = match args.get(1){
        Some(id) => id.as_str(),
        None => {
            println!("Usage: cargo run -p oracle -- <id>");
            return;
        }
    };

    match challenged_id {
        "0" => run_challenge::<Verifier0>("Sanity Check"),
        "1" => run_challenge::<Verifier1>("Ownership"),
        "2" => run_challenge::<Verifier2>("Borrowing"),
        "3" => run_challenge::<Verifier3>("Optionnal"),
        "4" => run_challenge::<Verifier4>("Result"),
        "5" => run_challenge::<Verifier5>("Iterator"),
        /*
        "6" => run_challenge::<Verifier6>("Borrowing"),
        "7" => run_challenge::<Verifier7>("Borrowing"),
        "8" => run_challenge::<Verifier8>("Borrowing"),
        "9" => run_challenge::<Verifier9>("Borrowing"),
        "10" => run_challenge::<Verifier10>("Borrowing"),
        "11" => run_challenge::<Verifier11>("Borrowing"),
        "12" => run_challenge::<Verifier12>("Borrowing"),
        */
        _ => println!("Challenge id {} does not exist yet", challenged_id)
    }
}

fn run_challenge<V: ChallengeVerifier>(name: &str){
    match solve::<V>(){ //TURBOFISH
        Some(flag) => println!("✨ {} ✨ verified: {}", name, flag),
        None  => println!("❌❌ {}: NOPE ❌❌ Have you even tried ??", name)
    }
}