use crate::logic::ChallengeVerifier;

pub struct Verifier0;

impl ChallengeVerifier for Verifier0 {
    type Output = String;
    fn id() -> &'static str {"0"}

    fn run_code(path: &str) -> Self::Output {
        let test_wrapper = r#"fn main() { print!("{}", hello()); }"#;
        Self::run_external(path, test_wrapper)
    }

    fn check_code(path: &str) -> bool {
        Self::run_code(path) == String::from("Hello world !")
    }

    fn secret_data() -> &'static [u8] {
        &[136, 81, 139, 8, 111, 9, 11, 99, 195, 22, 59, 119, 11, 215, 110, 253, 185, 9, 150, 31, 126, 170, 30, 75, 51, 67, 25, 69, 254, 71, 88, 37, 22, 110, 24, 47, 1, 192, 79, 206, 177, 114]
    }                   
}