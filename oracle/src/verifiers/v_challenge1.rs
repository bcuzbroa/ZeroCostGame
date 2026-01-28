use crate::logic::ChallengeVerifier;

pub struct Verifier1;

impl ChallengeVerifier for Verifier1{
    
    type Output = String;

    fn id() -> &'static str {"1"}
    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() {
                print!("{}", concat("input".to_string()));
            }
        "#;
        Self::run_external(path, wrapper)
    }
    
    fn check_code(path: &str) -> bool {
        // Validation check with the special string "x45àg"
        let test_wrapper = r#"
            fn main() {
                print!("{}", concat("x45àg".to_string()));
            }
        "#;
        Self::run_external(path, test_wrapper) == "x45àg processed"
    }

    fn secret_data() -> &'static [u8] {
        &[57, 79, 227, 77, 191, 91, 63, 11, 158, 102, 125, 229, 140, 192, 240, 157, 111, 212, 50, 31, 159, 116, 201, 130, 87, 148, 154, 37, 167, 61, 104, 150, 154, 216, 52, 94, 139, 192, 18, 195, 242, 217] 
        }
}
