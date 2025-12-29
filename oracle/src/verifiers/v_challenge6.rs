use crate::logic::ChallengeVerifier;
pub struct Verifier6;

impl ChallengeVerifier for Verifier6 {
    type Output = String;
    fn id() -> &'static str { "6" }

    fn run_code(path: &str) -> Self::Output {
        // Test simple d'exécution
        let wrapper = r#"fn main() { print!("{}", longest("short", "very long string")); }"#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let t1 = longest("apple", "pie") == "apple";
                let t2 = longest("a", "banana") == "banana";

                let t4 = longest("", "a") == "a";
                let t5 = longest("", "") == "";

                let string1 = String::from("long string");
                let string2 = String::from("xyz");
                let result = longest(string1.as_str(), string2.as_str());
                let t6 = result == "long string";
                print!("{}", t1 && t2 && t4 && t5 && t6);
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[117, 28, 148, 209, 238, 247, 49, 11, 8, 191, 11, 126, 17, 30, 46, 38, 129, 111, 86, 44, 175, 186, 135, 70, 220, 82, 71, 70, 145, 123, 75, 74, 185, 13, 173, 219, 142, 1]
    }
}
