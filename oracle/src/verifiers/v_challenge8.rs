use crate::logic::ChallengeVerifier;
pub struct Verifier8;

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