// Traits


trait Square{
    //Squares itself
    fn square(self) -> Self;
}

// Implement the square trait for i32
// ie for a given i: i32 = 30
// asserteq!(i.square(), 900)
impl Square for i32 {
    fn square(self) -> Self {
        self*self
    }
}

// Implement the square trait for a Complex number
struct Complex {
    re : f64,
    im : f64
}

impl Square for Complex{
    fn square(self) -> Self {
        Complex { 
            re: self.re * self.re - self.im * self.im, 
            im: 2.0*self.im*self.re
        }        
    }
}

//Implement the Square trait for a Vector (scalar product)
struct Vec2 {
    x: f32,
    y: f32,
}

impl Square for Vec2 {
    //Scalar product
    fn square(self) -> Self {
        Self {
            x: self.x * self.x,
            y: self.y * self.y,
        }
    }
}

