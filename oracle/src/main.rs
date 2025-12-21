mod crypto;
mod flags;
mod logic;

use crate::logic::*;

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
        "0" => run_challenge::<Verifier0>("Challenge0"),
        _ => println!("Challenge id {} does not exist yet", challenged_id)
    }


}


fn run_challenge<V: ChallengeVerifier>(name: &str){
    match solve::<V>(){
        Some(flag) => println!("✨ {} verified: {}", name, flag),
        None  => println!("NOPE ??? Have you even tried ??")
    }
}