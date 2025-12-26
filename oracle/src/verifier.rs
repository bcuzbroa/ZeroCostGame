use crate::logic::ChallengeVerifier;

pub struct Verifier0;
pub struct Verifier1;
pub struct Verifier2;
pub struct Verifier3;
pub struct Verifier4;
pub struct Verifier5;
/*
pub struct Verifier6;
pub struct Verifier7;
pub struct Verifier8;
pub struct Verifier9;
*/

impl ChallengeVerifier for Verifier0{
    
    type Output = String;
    fn id() -> &'static str {"0"}

    fn run_code(path :&str) -> Self::Output {
        let test_wrapper = r#"
                fn main() {
                    print!("{}", hello());
                }
            "#;
        Self::run_external(path,test_wrapper)
    }

    fn check_code(path: &str) -> bool {
        Self::run_code(path) == String::from("Hello world !")
    }
    
    fn secret_data() -> &'static [u8] {
        &[136, 81, 139, 8, 111, 9, 11, 99, 195, 22, 59, 119, 11, 215, 110, 253, 185, 9, 150, 31, 126, 170, 30, 75, 51, 67, 25, 69, 254, 71, 88, 37, 22, 110, 24, 47, 1, 192, 79, 206, 177, 114]
    }                   
}
impl ChallengeVerifier for Verifier1{
    
    type Output = String;

    fn id() -> &'static str {"1"}
    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() {
                print!("{}", challenge_1("input".to_string()));
            }
        "#;
        Self::run_external(path, wrapper)
    }
    
    fn check_code(path: &str) -> bool {
        // Validation check with the special string "x45àg"
        let test_wrapper = r#"
            fn main() {
                print!("{}", challenge_1("x45àg".to_string()));
            }
        "#;
        Self::run_external(path, test_wrapper) == "x45àg processed"
    }

    fn secret_data() -> &'static [u8] {
        &[57, 79, 227, 77, 191, 91, 63, 11, 158, 102, 125, 229, 140, 192, 240, 157, 111, 212, 50, 31, 159, 116, 201, 130, 87, 148, 154, 37, 167, 61, 104, 150, 154, 216, 52, 94, 139, 192, 18, 195, 242, 217] 
        }
}
            
            


impl ChallengeVerifier for Verifier2 {
    type Output = String;
    fn id() -> &'static str { "2" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"fn main() { print!("{}", coktail("Mojito".to_string())); }"#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let s = "     A Test Sentence  ".to_string();
                print!("{}", coktail(s));
            }
        "#;
        Self::run_external(path, test_wrapper) == "a test sentence"
    }

    fn secret_data() -> &'static [u8] {
        &[21, 173, 206, 69, 217, 82, 211, 8, 83, 239, 42, 220, 206, 241, 160, 3, 143, 250, 230, 109, 197, 43, 182, 211, 207, 246, 180, 127, 245, 159, 138, 100, 206, 108, 221, 31, 206, 42, 72, 135, 137, 115, 67, 162, 219, 22, 247]
    }
}

impl ChallengeVerifier for Verifier3 {
    type Output = String;
    fn id() -> &'static str { "3" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"fn main() { print!("{}", value_or_zero(Some(42))); }"#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let a = value_or_zero(Some(7));
                let b = value_or_zero(None);
                print!("{}|{}", a, b);
            }
        "#;
        Self::run_external(path, test_wrapper) == "7|0"
    }

    fn secret_data() -> &'static [u8] {
        &[119, 153, 58, 233, 153, 98, 115, 66, 177, 246, 152, 135, 37, 135, 122, 169, 16, 98, 103, 72, 121, 161, 204, 76, 73, 147, 12, 154, 48, 147, 34, 49, 124, 177, 166, 229, 86, 40, 20, 174, 57, 59, 225, 53, 30, 180]
    }
}

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

impl ChallengeVerifier for Verifier5 {
    type Output = Vec<u8>;
    fn id() -> &'static str { "5" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() {
                let nums: Vec<u32> = (1..=255).collect();
                let res: Vec<u8> = scramble(nums).flat_map(|x| x.to_le_bytes()).collect();
                // Print as a comma-separated list of bytes: "1,2,3,..."
                let output: Vec<String> = res.iter().map(|b| b.to_string()).collect();
                print!("{}", output.join(","));
            }
        "#;
        
        let raw_output = Self::run_external(path, wrapper);
        
        // Parse the comma-separated string back into a Vec<u8>
        raw_output.split(',')
            .filter_map(|s| s.parse::<u8>().ok())
            .collect()
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                let numbers: Vec<u32> = (1..=255).collect();
                let sorted_target: [u32;85] = [2, 10, 14, 22, 26, 34, 38, 46, 50, 58, 62, 70, 74, 82, 86, 94, 98, 106, 110, 118, 122, 130, 134, 142, 146, 154, 158, 166, 170, 178, 182, 190, 194, 202, 206, 214, 218, 226, 230, 238, 242, 250, 254, 262, 266, 274, 278, 286, 290, 298, 302, 310, 314, 322, 326, 334, 338, 346, 350, 358, 362, 370, 374, 382, 386, 394, 398, 406, 410, 418, 422, 430, 434, 442, 446, 454, 458, 466, 470, 478, 482, 490, 494, 502, 506];
                let is_sum_ok = sumup(scramble(numbers.clone())) == 21674;
                let is_sort_ok = sorted_target == *collect_and_sort(scramble(numbers));
                if is_sum_ok && is_sort_ok { print!("true"); } else { print!("false"); }
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[123, 59, 213, 113, 31, 233, 174, 100, 145, 62, 123, 85, 102, 125, 189, 74, 117, 129, 18, 15, 163, 121, 154, 178, 48, 143, 83, 201, 255, 192, 110, 157, 45, 147, 113, 232, 119, 35, 249, 98, 250, 207, 75, 179, 72, 234, 38, 177, 37, 98, 60]
    }
}


/*
impl ChallengeVerifier for Verifier6{
    
type Output = String; //This type can be hashed

fn run_code() -> Self::Output {
    todo!()
}
fn check_code() -> bool {
    todo!()
}
// Default implementation for compilation purposes
// &[0] is later replaced with the encrypted flag   
fn secret_data() -> &'static [u8]{
    &[0]
}

}

impl ChallengeVerifier for Verifier7{
    
type Output = String; //This type can be hashed

fn run_code() -> Self::Output {
    todo!()
}
fn check_code() -> bool {
    todo!()
}
// Default implementation for compilation purposes
// &[0] is later replaced with the encrypted flag   
fn secret_data() -> &'static [u8]{
    &[0]
}

}

impl ChallengeVerifier for Verifier8{
    
type Output = String; //This type can be hashed

fn run_code() -> Self::Output {
    todo!()
}
fn check_code() -> bool {
    todo!()
}
// Default implementation for compilation purposes
// &[0] is later replaced with the encrypted flag   
fn secret_data() -> &'static [u8]{
    &[0]
}

}

impl ChallengeVerifier for Verifier9{
    
type Output = String; //This type can be hashed

fn run_code() -> Self::Output {
    todo!()
}
fn check_code() -> bool {
    todo!()
}
// Default implementation for compilation purposes
// &[0] is later replaced with the encrypted flag   
fn secret_data() -> &'static [u8]{
    &[0]
}

}
*/

/* Implementation Template
impl ChallengeVerifier for Verifier6{
    
type Output = String; //This type can be hashed

fn run_code() -> Self::Output {
    todo!()
    }
    fn check_code() -> bool {
        todo!()
        }
        // Default implementation for compilation purposes
        // &[0] is later replaced with the encrypted flag   
        fn secret_data() -> &'static [u8]{
            &[0]
            }
            
        }
        */