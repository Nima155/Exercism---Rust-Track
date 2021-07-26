use std::fmt::Display;

use num::{Num, Zero}; // external num crate

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Num + Copy + PartialOrd + Display, // PartialOrd is for comparisons such as >= .. rest we know
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if !Triangle::validator(&sides) {
            return None;
        }
        Some(Self(sides[0], sides[1], sides[2]))
    }
    fn validator(sides: &[T; 3]) -> bool {
        if sides.iter().any(|f| Zero::is_zero(f)) {
            return false;
        }
        sides[0] + sides[1] >= sides[2]
            && sides[1] + sides[2] >= sides[0]
            && sides[2] + sides[0] >= sides[1]
    }

    pub fn is_equilateral(&self) -> bool {
        (self.0 == self.1) && self.1 == self.2
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.0 != self.2 && self.1 != self.2
    }

    pub fn is_isosceles(&self) -> bool {
        let slice = [self.0 == self.1, self.1 == self.2, self.0 == self.2];
        slice.iter().filter(|f| **f).count() != 0
    }
}
