/* =========================================================
EXERCISE: HIGHER-ORDER FUNCTIONS & SLIDING WINDOWS
/!\ HARD /!\ /!\ HARD /!\ /!\ HARD /!\ /!\ HARD /!\ /!\ HARD /!\
=========================================================
*/

/* 1. The "select_best" function /!\ HARD /!\
@args: 
    - items: &'a [T] (a slice of any type T)
    - key: F (a function/closure that turns &T into a comparable key K)
@returns: Option<&'a T>

Goal:
Return a reference to the item that produces the maximum value when 
passed through the `key` function.

Constraints:
- T does NOT implement Ord, but the result of the key function (K) does.
- Do NOT clone or copy the items.
- If the slice is empty, return None.

Example logic:
If items are Books and the key function returns the number of pages,
this function returns the book with the most pages.
*/

// TODO: Write the select_best function here
// Hint: You will need to define generic types <'a, T, K, F>
// and use trait bounds (where F: Fn(&T) -> K, K: Ord)


/* 2. The "select_best_of_each_window" function
@args:
    - items: &'a [T]
    - key: F
    - window_size: usize
@returns: Vec<&'a T>

Goal:
Instead of looking at the whole slice, look at every "sliding window" 
of a specific size and find the best item in each.

Example:
items = [10, 2, 30, 4, 50], window_size = 3
Window 1: [10, 2, 30] -> Best is 30
Window 2: [2, 30, 4]  -> Best is 30
Window 3: [30, 4, 50] -> Best is 50
Result: [30, 30, 50]

Constraints:
- Return a Vector of references pointing to the original slice.
- If window_size is 0 or larger than the slice, return an empty Vector.
- Try to reuse your `select_best` function inside this one!

Hint: Look up the `.windows()` method for slices in the Rust docs.
*/

// TODO: Write the select_best_of_each_window function here