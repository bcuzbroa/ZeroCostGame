// OPTION — BEGINNER LEVEL

// The given output is an Option<i32>
// Task : Returns the contained value or 0 if there is None
// The use of "match" is recommanded

fn value_or_zero(n: Option<i32>) -> i32 {
    match n {
        Some(v) => v,
        None => 0,
    }
}

fn sum_options(numbers: Vec<Option<i32>>) -> i32 {
    let mut total = 0;
    for num in numbers {
        total += value_or_zero(num);
    }
    total
}