mod crypto;
mod logic;
mod verifiers;
mod wrapper;

use logic::{ChallengeVerifier, solve};
use verifiers::*;

use clap::Parser;
use regex::Regex;




#[derive(Parser, Debug)]
#[command(author, version, about = "Zero Cost Game")]
struct Args {
    #[arg(short, long)]
    path: String,
}


fn main() {

    let args= Args::parse();

    let path = &args.path;

    // Verify the directory exists before proceeding
    if !std::path::Path::new(&path).exists() {
        println!("Error: The challenge path: '{}' does not exist.", path);
        return;
    }

    let re = Regex::new(r"(\d+)").unwrap();

    let challenge_id = re.captures(path)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str())
        .unwrap();
    
    match challenge_id {
        "0" => run_challenge::<Verifier0>("Sanity Check"    , path),
        "1" => run_challenge::<Verifier1>("Ownership"       , path),
        "2" => run_challenge::<Verifier2>("Borrowing"       , path),
        "3" => run_challenge::<Verifier3>("Optionnal"       , path),
        "4" => run_challenge::<Verifier4>("Result"          , path),
        "5" => run_challenge::<Verifier5>("Iterator"        , path),
        "6" => run_challenge::<Verifier6>("LifeTimer"       , path),
        "7" => run_challenge::<Verifier7>("Trait Master"    , path),
        "8" => run_challenge::<Verifier8>("Advanced User"   , path),
        "9" => run_challenge::<Verifier9>("Smart Pointer"   , path),
        "10" => run_challenge::<Verifier10>("Generic Master" , path),
        
        _ => println!("Challenge{} does not exist yet or is not implemented", challenge_id)
    }
}

fn run_challenge<V: ChallengeVerifier>(name: &str, path: &str){
    match solve::<V>(path){ //TURBOFISH
        Some(flag) => println!("✨ {} ✨ verified: {}", name, flag),
        None  => println!("❌❌ {}: NOPE ❌❌ Do you even try ??", name)
    }
}