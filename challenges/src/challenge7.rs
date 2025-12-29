// Traits

pub trait Square{
    fn square(self) -> Self;
}


impl Square for i32 {
    fn square(self) -> Self {
        self*self
    }
}


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

