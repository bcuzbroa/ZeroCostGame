use crate::logic::ChallengeVerifier;
pub struct Verifier9;

impl ChallengeVerifier for Verifier9 {
    type Output = String;
    fn id() -> &'static str {"9"}

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() { 
                let root = Rc::new(Folder { name: "/".into(), parent: None });
                let sub = create_child("bin", &root);
                print!("{}", get_depth(&sub)); 
            }
        "#;
        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                // Setup
                let root = Rc::new(Folder { name: "/".to_string(), parent: None });
                let level1 = Rc::new(create_child("home", &root));
                let level2 = create_child("user", &level1);
    
                let p = get_parent(&level1);
                
                let t1 = get_parent(&root).is_none();
                let t2 = p.is_some() && p.unwrap().name == "/";
                let t3 = get_depth(&root) == 0;
                let t4 = get_depth(&level2) == 2;
                let t5 = Rc::strong_count(&root) >= 2;

                print!("{}", t1 && t2 && t3 && t4 && t5);
            }
        "#;
        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[216, 189, 83, 173, 241, 157, 59, 77, 254, 27, 204, 162, 147, 113, 218, 88, 6, 150, 30, 90, 231, 213, 55, 104, 85, 134, 165, 247, 173, 177, 204, 171, 83, 209, 186, 53, 195, 112, 32, 14, 134, 108]
    }
}