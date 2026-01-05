/* =========================================================
EXERCISE: LIFETIMES
=========================================================
*/

/* 1. Implement the "longest" function
@args:    Two string slices (&str)
@returns: One string slice (&str)

Task: 
Compare the length of the two input strings and return the 
longest one. If they are equal, return the second one.

Focus:
You will need to use Lifetime Annotations ('a) to tell the 
compiler how the lifetime of the return value relates to 
the lifetime of the arguments.

Think about:
Why can't the compiler figure out the lifetime by itself here?
What happens if 'x' and 'y' have different lifetimes?
*/

// TODO: Write the longest function here with appropriate lifetime annotations

// Main Section
fn main() {
   //Implement your tests here
}
/* Bonus Question for Students:
Look at the return type and the argument types. 
What is the link between them? 
Why is the 'a annotation necessary for the borrow checker?
*/