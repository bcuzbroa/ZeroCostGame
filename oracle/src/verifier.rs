use crate::logic::ChallengeVerifier;

pub struct Verifier0;
pub struct Verifier1;
pub struct Verifier2;
pub struct Verifier3;
pub struct Verifier4;
pub struct Verifier5;
pub struct Verifier6;
pub struct Verifier7;
pub struct Verifier8;
/*
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

impl ChallengeVerifier for Verifier7 {
    type Output = String;

    fn id() -> &'static str { "7" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"fn main() { print!("{}", 12.square()); }"#;
        Self::run_external(path, wrapper)
    }

    // Ici on teste la logique mathématique pour les 3 types
    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                // Test 1 : i32
                // 5² = 25 et (-5)² = 25
                let t1 = 5.square() == 25;
                let t2 = (-5).square() == 25;

                // Test 2 : Complex
                // (1 + 2i)² = (1*1 - 2*2) + (2*1*2)i = -3 + 4i
                let c = Complex { re: 1.0, im: 2.0 };
                let c_res = c.square();
                // On vérifie champ par champ avec une petite tolérance (epsilon)
                let t3 = (c_res.re - (-3.0)).abs() < 0.0001 && (c_res.im - 4.0).abs() < 0.0001;

                // Test 3 : Vec2
                // (2, 3) -> (4, 9)
                let v = Vec2 { x: 2.0, y: 3.0 };
                let v_res = v.square();
                let t4 = (v_res.x - 4.0).abs() < 0.0001 && (v_res.y - 9.0).abs() < 0.0001;

                print!("{}", t1 && t2 && t3 && t4);
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[81, 158, 82, 253, 103, 27, 140, 117, 30, 41, 229, 93, 203, 135, 193, 249, 40, 233, 246, 114, 66, 77, 32, 123, 32, 254, 214, 123, 4, 159, 35, 143, 176, 226, 1, 172, 59, 104, 219, 49, 173, 15, 246]
    }
}





impl ChallengeVerifier for Verifier8 {
    type Output = String;
    fn id() -> &'static str { "8" }

    // Run code: Demonstrates that the 'largest' function works on a simple list of integers
    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() { 
                let number_list = vec![34, 50, 25, 100, 65];
                let result = largest(&number_list);
                print!("{}", result); 
            }
        "#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                // --- Test 1: Generics with Integers ---
                let list_i32 = vec![10, 50, 100, 20];
                let t1 = *largest(&list_i32) == 100;

                // --- Test 2: Generics with Chars ---
                let list_char = vec!['a', 'z', 'm'];
                let t2 = *largest(&list_char) == 'z';

                // --- Setup for Book Tests ---
                // Book A: 100 pages, Title "Alpha"
                let b1 = Book { title: "Alpha", author: "Author 1", pages: 100 };
                // Book B: 100 pages, Title "Beta"
                let b2 = Book { title: "Beta",  author: "Author 2", pages: 100 };
                // Book C: 200 pages, Title "Gamma"
                let b3 = Book { title: "Gamma", author: "Author 3", pages: 200 };

                // --- Test 3: PartialEq Implementation ---
                // Your impl checks ONLY pages. b1 (100) and b2 (100) should be 'equal'
                let t3 = b1 == b2; 

                // --- Test 4: PartialOrd Implementation ---
                // Your impl checks pages first, THEN title.
                // b3 has more pages than b2, so b3 > b2.
                let t4 = b3 > b2; 
                
                // Tie-breaking check:
                // b1 and b2 have same pages (100).
                // "Beta" > "Alpha", so b2 should be greater than b1.
                let t5 = b2 > b1;

                // --- Test 5: Largest with Books ---
                // List: [Alpha(100), Beta(100), Gamma(200)]
                let library = vec![b1, b2, b3];
                let best_book = largest(&library);
                
                // The largest should be b3 (Gamma) because it has the most pages
                let t6 = best_book.title == "Gamma";

                // Tie-breaker list: [Alpha(100), Beta(100)]
                // Should return Beta because title "Beta" > "Alpha"
                let b_alpha = Book { title: "Alpha", author: "A", pages: 100 };
                let b_beta = Book { title: "Beta",  author: "A", pages: 100 };
                let library_tie = vec![b_alpha, b_beta];
                let t7 = largest(&library_tie).title == "Beta";

                print!("{}", t1 && t2 && t3 && t4 && t5 && t6 && t7);
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[117, 124, 36, 221, 182, 33, 215, 12, 10, 205, 118, 253, 231, 54, 229, 206, 166, 24, 1, 232, 193, 174, 79, 51, 115, 155, 181, 56, 103, 173, 174, 233, 6, 12, 242, 255, 92, 220, 176, 211, 253, 39, 207, 253, 251, 161, 2, 25, 138, 68, 252, 28, 8, 148, 194]
    }
}
                
/*
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