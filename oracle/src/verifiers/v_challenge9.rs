use crate::logic::ChallengeVerifier;

pub struct Verifier9;


impl ChallengeVerifier for Verifier9 {
    type Output = String;
    fn id() -> &'static str {"9"}

    fn run_code(path: &str) -> Self::Output {
        todo!()
    }

    fn check_code(path: &str) -> bool {
        todo!()
    }

    fn secret_data() -> &'static [u8] {
        &[0]
    }                   
}