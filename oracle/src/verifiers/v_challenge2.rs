use crate::logic::ChallengeVerifier;

pub struct Verifier2;

impl ChallengeVerifier for Verifier2 {
    type Output = String;
    fn id() -> &'static str { "2" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"fn main() { print!("{}", cocktail("Mojito".to_string())); }"#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let s = "     A Test Sentence  ".to_string();
                print!("{}", cocktail(s));
            }
        "#;
        Self::run_external(path, test_wrapper) == "a test sentence"
    }

    fn secret_data() -> &'static [u8] {
        &[21, 173, 206, 69, 217, 82, 211, 8, 83, 239, 42, 220, 206, 241, 160, 3, 143, 250, 230, 109, 197, 43, 182, 211, 207, 246, 180, 127, 245, 159, 138, 100, 206, 108, 221, 31, 206, 42, 72, 135, 137, 115, 67, 162, 219, 22, 247]
    }
}