mod solves;
mod crypto;
mod flags;
use crate::solves::solve0::solve_and_get_flag;

fn main() {
}


fn flag(){
    match solve_and_get_flag() {
        Some(flag) => println!("What a good start : {}", flag),
        None => println!("❌ Challenge 0 non validé"),
    }

}

trait ChallengeVerifier {

    //A type that can be hashed.
    type Output: AsRef<[u8]>; 

    fn run_player_code() -> Self::Output;
    fn check_result(out: &Self::Output) -> bool;
    fn secret_data() -> (&'static [u8; 12], &'static [u8]);
}