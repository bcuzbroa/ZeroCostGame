/* =========================================================
EXERCISE: TRAITS
=========================================================
*/

/* 1. The "Square" Trait
Below is the Square trait definition.
It aims to multiply by itself (square)
any struct that implements this trait.
*/
trait Square {
    fn square(self) -> Self;
}


/* 2. Implementation for i32
Task: Implement the Square trait for the standard i32 type.
Example: 
let i: i32 = 30;
assert_eq!(i.square(), 900);
*/

// TODO: Implement Square for i32


/* 3. Implementation for Complex Numbers
Task: 
- Create a struct named "Complex" with two fields: re (f64) and im (f64).
- Implement the Square trait for Complex.


*/

// TODO: Define the struct and implement Square for Complex


/* 4. Implementation for Vec2
Task: 
- Create a struct named "Vec2" with fields: x (f32) and y (f32).
- Implement the Square trait for Vec2 using a "scalar product" logic.

Logic: Each component is squared individually.
- new_x = x * x
- new_y = y * y

*/

// TODO: Define the struct and implement Square for Vec2

// Main Section
fn main() {
   //Implement your tests here
}