/* =========================================================
EXERCISE: ITERATORS & ADAPTORS
=========================================================
*/

/* 1. Implement the "scramble" function
@args:    Vec<u32>
@returns: impl Iterator<Item = u32>

Task: Create a lazy iterator transformation chain that:
1. Filters out even numbers.
2. Filters out numbers divisible by 3 (n % 3 == 0).
3. Multiplies the remaining numbers by 2.

Requirements:
- Use .into_iter() to consume the vector.
- DO NOT use .collect(). Return the iterator itself.

Example: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10] 
-> Filter even: [1, 3, 5, 7, 9]
-> Filter div by 3: [1, 5, 7]
-> Multiply by 2: [2, 10, 14]
*/

// TODO: Write the scramble function here



/* 2. Implement the "sumup" function
@args:    impl Iterator<Item = u32> (the result from scramble)
@returns: u32

Task: Consume the iterator and return the sum of all its elements.
*/

// TODO: Write the sumup function here



/* 3. Implement the "collect_and_sort" function
@args:    impl Iterator<Item = u32>
@returns: Vec<u32>

Task: 
1. Collect the iterator into a Vector.
2. Sort the vector in ascending order.
3. Return the sorted vector.
*/

// TODO: Write the collect_and_sort function here