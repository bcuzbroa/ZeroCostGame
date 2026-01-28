use crate::logic::ChallengeVerifier;
pub struct Verifier10;

impl ChallengeVerifier for Verifier10 {
    type Output = String;

    fn id() -> &'static str { "10" }

    fn run_code(path: &str) -> Self::Output {
        let wrapper = r#"
            fn main() {
                #[derive(Debug)]
                struct Item {
                    label: &'static str,
                    value: i32,
                }

                let items = [
                    Item { label: "a", value: 1 },
                    Item { label: "b", value: 3 },
                    Item { label: "c", value: 2 },
                    Item { label: "d", value: 5 },
                ];

                let best = select_best(&items, |i| i.value).unwrap();
                let windows = select_best_of_each_window(&items, |i| i.value, 2);

                print!("{} {}", best.label, windows.len());
            }
        "#;

        Self::run_external(path, wrapper)
    }

    fn check_code(path: &str) -> bool {
        let test_wrapper = r#"
            fn main() {
                #[derive(Debug)]
                struct Item {
                    label: &'static str,
                    value: i32,
                }

                let items = [
                    Item { label: "a", value: 1 },
                    Item { label: "b", value: 3 },
                    Item { label: "c", value: 2 },
                    Item { label: "d", value: 5 },
                ];

                let empty: [Item; 0] = [];

                let t1 = select_best(&empty, |i| i.value).is_none();

                let best = select_best(&items, |i| i.value);
                let t2 = best.is_some() && best.unwrap().label == "d";

                // key derived (not direct field)
                let best_len = select_best(&items, |i| i.label.len());
                let t3 = best_len.unwrap().label == "a";

 
                let r = select_best_of_each_window(&items, |i| i.value, 2);

                let t4 = r.len() == 3;
                let t5 = r[0].label == "b";
                let t6 = r[1].label == "b";
                let t7 = r[2].label == "d";

                // edge cases
                let r2 = select_best_of_each_window(&items, |i| i.value, 0);
                let r3 = select_best_of_each_window(&items, |i| i.value, 10);

                let t8 = r2.is_empty();
                let t9 = r3.is_empty();

                let ptr_original = &items[1] as *const Item;
                let ptr_result   = r[0] as *const Item;
                let t10 = ptr_original == ptr_result;

                print!("{}", t1 && t2 && t3 && t4 && t5 && t6 && t7 && t8 && t9 && t10);
            }
        "#;

        Self::run_external(path, test_wrapper) == "true"
    }

    fn secret_data() -> &'static [u8] {
        &[64, 32, 81, 109, 212, 221, 236, 153, 176, 118, 54, 139, 61, 61, 235, 188, 103, 231, 123, 144, 195, 137, 129, 79, 172, 64, 251, 37, 122, 240, 94, 159, 183, 216, 55, 116, 154, 239, 193, 75, 91, 108, 53, 119, 231, 197, 210]
    }
}
