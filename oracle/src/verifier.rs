use crate::logic::ChallengeVerifier;
use challenges::*;


pub struct Verifier0;
pub struct Verifier4;
pub struct Verifier1;
pub struct Verifier2;
pub struct Verifier3;
pub struct Verifier5;
pub struct Verifier6;
pub struct Verifier7;
pub struct Verifier8;
pub struct Verifier9;



impl ChallengeVerifier for Verifier0{
    
    type Output = String;
    fn run_code() -> Self::Output {hello()}
    fn check_code() -> bool {
        hello() == String::from("Hello world !")
    }
    
    fn secret_data() -> &'static [u8] {
        &[136, 81, 139, 8, 111, 9, 11, 99, 195, 22, 59, 119, 11, 215, 110, 253, 185, 9, 150, 31, 126, 170, 30, 75, 51, 67, 25, 69, 254, 71, 88, 37, 22, 110, 24, 47, 1, 192, 79, 206, 177, 114]
    }                   
}

impl ChallengeVerifier for Verifier1{
    
    type Output = String;
    fn run_code() -> Self::Output {challenge_1("input".to_string())}
    fn check_code() -> bool {
        challenge_1(String::from("x45àg")) == String::from("x45àg processed")
    }
    fn secret_data() -> &'static [u8] {
        &[57, 79, 227, 77, 191, 91, 63, 11, 158, 102, 125, 229, 140, 192, 240, 157, 111, 212, 50, 31, 159, 116, 201, 130, 87, 148, 154, 37, 167, 61, 104, 150, 154, 216, 52, 94, 139, 192, 18, 195, 242, 217] 
    }
}



impl ChallengeVerifier for Verifier2{
    
    type Output = String;
    fn run_code() -> Self::Output {coktail("Mojito".to_string())}
    fn check_code() -> bool {
        let s = String::from("     A Test Sentence  ");
        coktail(s) == "a test sentence".to_string()
    }
    fn secret_data() -> &'static [u8] {
        &[21, 173, 206, 69, 217, 82, 211, 8, 83, 239, 42, 220, 206, 241, 160, 3, 143, 250, 230, 109, 197, 43, 182, 211, 207, 246, 180, 127, 245, 159, 138, 100, 206, 108, 221, 31, 206, 42, 72, 135, 137, 115, 67, 162, 219, 22, 247]
    }
}


impl ChallengeVerifier for Verifier3 {
    type Output = String;

    fn run_code() -> Self::Output {
        value_or_zero(Some(42)).to_string()
    }

    fn check_code() -> bool {
        let a = value_or_zero(Some(7));
        let b = value_or_zero(None);

        a == 7 && b == 0
    }
    fn secret_data() -> &'static [u8] {
        &[119, 153, 58, 233, 153, 98, 115, 66, 177, 246, 152, 135, 37, 135, 122, 169, 16, 98, 103, 72, 121, 161, 204, 76, 73, 147, 12, 154, 48, 147, 34, 49, 124, 177, 166, 229, 86, 40, 20, 174, 57, 59, 225, 53, 30, 180]
    }
}

impl ChallengeVerifier for Verifier4 {
    type Output = String;
    
    fn run_code() -> Self::Output {
        let command = "foo!bar";
        valid_command(command).unwrap()
    }

fn check_code() -> bool {
    
    let test1 = valid_command("foo!bar").is_ok_and(|c| c == "foobar");
    let test2 = matches!(check("!"), Err(CommandError::ContainsBang));
    let test3 = matches!(check("/"), Err(CommandError::ContainsSlash));
    let test4 = valid_command("foobar").is_ok_and(|c| c == "foobar");
    let test5 = matches!(valid_command("c/"), Err(CommandError::ContainsSlash));

    test1 && test2 && test3  && test4 && test5
}

    fn secret_data() -> &'static [u8] {
        &[139, 213, 44, 245, 26, 55, 60, 77, 147, 125, 120, 5, 13, 225, 168, 204, 227, 97, 173, 100, 175, 113, 132, 152, 203, 91, 182, 108, 225, 242, 48, 168, 49, 10, 117, 159, 70, 132, 229, 165, 166, 237]
    }
}
