/* ======================================================
CHALLENGE 8: GENERICS & COMPARISON TRAITS
=========================================================
*/

/* 1. The "largest" function
@args:    &[T] (a slice of generic type T)
@returns: &T (a reference to the largest element)

Task: 
Find and return the largest element in the provided list.

Requirements:
- The type T must implement the "PartialOrd" trait(explain why ?)
- The function should return a reference to the item.
*/

// TODO: Write the largest<T> function here


/* 2. The "Book" Struct & Comparison
Task A: 
Define a struct "Book" with a lifetime 'a.
Fields:
- author: &'a str
- title: &'a str
- pages: i32
*/

// TODO: Define the Book struct here


/* Task B: Implement PartialEq for Book
Two books are considered "equal" ONLY if they have 
the same number of pages.
*/

// TODO: Implement PartialEq for Book


/* Task C: Implement PartialOrd for Book
Task: Compare books based on pages. 
If the number of pages is equal, compare them alphabetically by title
such as Beta > Alpha (increasing order )

Hint: Use the ".then()" method on the result of a comparison 
to chain the title check.
Some(
    `COMPARING THE PAGES`
    .then(`COMPARING THE TITLES`)
)
*/

// TODO: Implement PartialOrd for Book


// Main Section
fn main() {
   //Implement your tests here
}