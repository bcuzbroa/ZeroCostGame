use crate::logic::ChallengeVerifier;

pub struct Verifier3;

impl ChallengeVerifier for Verifier3 {
    type Output = String;
    fn id() -> &'static str { "3" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() { 
                let data = vec![Some(10), None, Some(32)];
                print!("{}", sum_options(data)); 
            }
        "#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let t1 = value_or_zero(Some(10)) == 10 && value_or_zero(None) == 0;
                
                let list = vec![Some(5), None, Some(15), None];
                let sum = sum_options(list);
                
                let empty_sum = sum_options(vec![]);

                let t2 = sum == 20;
                let t3 = empty_sum == 0;     

                print!("{}", t1 && t2 && t3 );
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[119, 153, 58, 233, 153, 98, 115, 66, 177, 246, 152, 135, 37, 135, 122, 169, 16, 98, 103, 72, 121, 161, 204, 76, 73, 147, 12, 154, 48, 147, 34, 49, 124, 177, 166, 229, 86, 40, 20, 174, 57, 59, 225, 53, 30, 180]
    }
}