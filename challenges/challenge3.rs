/* =========================================================
EXERCISE: OPTIONS - BEGINNER LEVEL
=========================================================
*/

/* 1. Implement the "value_or_zero" function
@args:    Option<i32>
@returns: i32

Task:
Extract the value contained in the Option.
- If it is "Some(v)", return the value v.
- If it is "None", return 0.

Hint: Using a "match" statement is the most idiomatic way 
for a beginner to handle this.
*/

// TODO: Write the value_or_zero function here


/* 2. The "sum_options" function
@args:    Vec<Option<i32>> (a vector of optional integers)
@returns: i32

Task:
Calculate the total sum of all the numbers in the vector.

Requirements:
- Iterate through the vector.
- Use your "value_or_zero" function to handle each element.
- Return the final sum.

Example:
[Some(10), None, Some(5)] -> Should return 15
*/

// TODO: Write the sum_options function here