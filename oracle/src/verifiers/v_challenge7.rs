use crate::logic::ChallengeVerifier;

pub struct Verifier7;

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
                let t1 = 5.square() == 25;
                let t2 = (-5).square() == 25;

                // Test 2 : Complex
                // (1 + 2i)² = (1*1 - 2*2) + (2*1*2)i = -3 + 4i
                let c = Complex { re: 1.0, im: 2.0 };
                let c_res = c.square();
                
                // Little tolerance (epsilon)
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



