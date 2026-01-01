/* /!\ HARD ONE /!\
Write a select_best function such as :

Context:
You are given a slice of `items` of any type `T`.
You must return a reference to the "best" item according to a ranking key.

The ranking is not based on `T` itself.
Instead, the caller provides a function `key` that maps each `&T`
to a value `K`, and this value is what determines which item is better.

`K` must implement `Ord`, so it can be compared.
`T` itself does NOT need to implement any comparison trait.

Goal:
- Iterate over the slice
- Compare items using their computed keys
- Return a reference to the item with the maximum key

If the slice is empty, return `None`.

Important constraints:
- Do NOT allocate new collections
- Do NOT clone or copy items
- The returned reference must point to an element of the input slice
- The function must work for any type `T`
- The function must work with any key type `K` that implements `Ord`

Hint:
If this function feels similar to something from the standard library,
that is not a coincidence.
*/
fn select_best <'a, T, K, F>(
    items: &'a [T],
    key: F,
) -> Option<&'a T>
where 
    F : Fn(&T) -> K,
    K : Ord,
{

    let mut iter = items.iter();
    let mut best = iter.next()?;
    let mut best_key = key(best);

    for t in iter {
        let k = key(t);
        if k > best_key{
            best = t;
            best_key = k
        }
    }
    
    Some(best)
}

/*

Write a select_best_of_each_window function

Context:
You are given a slice of items of any type `T`.
Instead of selecting a single best item from the entire slice,
you must select the best item from each sliding window of fixed size.

A sliding window is a contiguous sub-slice of the input:
- window 0: items[0..window_size]
- window 1: items[1..1+window_size]
- window 2: items[2..2+window_size]
- etc.

For each window, you must select the "best" item according to a ranking key.

The ranking is defined by the `key` function, which maps `&T` to a value `K`.
Only `K` must implement `Ord`. The type `T` itself does not need to.

Goal:
- Iterate over all sliding windows of size `window_size`
- For each window, select the best element
- Collect the results into a vector of references

Important constraints:
- Do NOT allocate or clone items
- Do NOT return owned values
- The returned references must point to elements of the input slice
- The function must work for any type `T`
- The function must reuse existing abstractions when possible

Edge cases:
- If `window_size` is 0, the behavior is undefined
- If `window_size` is greater than the slice length, return an empty vector

Example:

items = [a, b, c, d]
window_size = 3

Windows are:
[a, b, c]
[b, c, d]

Mental model:
This function answers the question:
"What would `select_best` return for each sliding window?"

Hint:
The standard library already provides a way to iterate over sliding windows.
Return a Vec<_> (replace _ with the proper type)

Rules :
window_size must be of type `usize`
*/


fn select_best_of_each_window<'a, T, K, F>(
    items: &'a [T],
    key: F,
    window_size : usize,
) -> Vec<&'a T>
where 
    K : Ord,
    F : Fn(&T) -> K,
{
    if window_size == 0 || window_size > items.len()  {
        return Vec::new()
    }

    let mut best_of_each_window : Vec<&T> = Vec::new();
    
    for window in items.windows(window_size){
        
        best_of_each_window.push(select_best(window, &key).unwrap());
    }
    best_of_each_window
}