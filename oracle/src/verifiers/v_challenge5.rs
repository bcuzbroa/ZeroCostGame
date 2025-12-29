use crate::logic::ChallengeVerifier;

pub struct Verifier5;

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