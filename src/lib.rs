extern crate num_traits;

use num_traits::Signed;
use std::fmt;
use std::ops::{Mul, Sub};

pub struct Bounds<T>(pub Point<T>, pub Point<T>);

impl<T: Copy> Bounds<T> {
    pub fn new(x1: T, y1: T, x2: T, y2: T) -> Self {
        let a = Point { x: x1, y: y1 };
        let b = Point { x: x2, y: y2 };
        Bounds(a, b)
    }
}

impl<T> Bounds<T>
where
    T: Copy + Mul<Output = T> + Sub<Output = T>,
{
    pub fn area(&self) -> T {
        let dx = self.1.x - self.0.x;
        let dy = self.1.y - self.0.y;
        dx * dy
    }
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl<T: fmt::Display> fmt::Display for Point<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Copy + Signed> Point<T> {
    pub fn manhattan_distance(&self, other: &Self) -> T {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        dx.abs() + dy.abs()
    }
}
