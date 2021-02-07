use num::traits::Float;
use std::fmt;
use std::ops::{Add, AddAssign};

struct Vec2<T: Float> {
    x: T,
    y: T,
}

impl<T: Float> Add for Vec2<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: Float> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T: fmt::Display + Float> fmt::Display for Vec2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    for _i in 0..10 {
        let a = Vec2 {
            x: 100_f32,
            y: 100_f32,
        };
        let b = Vec2 {
            x: 200_f32,
            y: 200_f32,
        };
        let c = a + b;
        println!("{}", c);
    }
}
