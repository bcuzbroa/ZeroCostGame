use crate::logic::ChallengeVerifier;

pub struct Verifier4;

impl ChallengeVerifier for Verifier4 {
    type Output = String;
    fn id() -> &'static str { "4" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"fn main() { print!("{}", valid_command("foo!bar").unwrap()); }"#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let t1 = valid_command("foo!bar").is_ok_and(|c| c == "foobar");
                let t2 = matches!(check("!"), Err(CommandError::ContainsBang));
                let t3 = matches!(check("/"), Err(CommandError::ContainsSlash));
                let t4 = valid_command("foobar").is_ok_and(|c| c == "foobar");
                let t5 = matches!(valid_command("c/"), Err(CommandError::ContainsSlash));
                print!("{}", t1 && t2 && t3 && t4 && t5);
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[139, 213, 44, 245, 26, 55, 60, 77, 147, 125, 120, 5, 13, 225, 168, 204, 227, 97, 173, 100, 175, 113, 132, 152, 203, 91, 182, 108, 225, 242, 48, 168, 49, 10, 117, 159, 70, 132, 229, 165, 166, 237]
    }
}
