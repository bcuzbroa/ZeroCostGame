// OPTION — BEGINNER LEVEL

// The given output is an Option<i32>
// Task : Returns the contained value or 0 if there is None
// The use of "match" is recommanded

pub fn value_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(v) => v,
        None => 0,
    }
}
