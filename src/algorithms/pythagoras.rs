extern crate num;
use self::num::{Unsigned, NumCast};

pub struct Triangle<T> {
    a: T,
    b: T,
    c: T,
}

impl<T> Triangle<T>
where
    T: NumCast + Unsigned + PartialOrd + Copy,
{
    pub fn get_triangle_type(&self) -> char {
        let mut sorted = vec![self.a, self.b, self.c];
        ::sorts::bubble_sort(&mut sorted);
        let sorted_squared: Vec<T> = {
            sorted.iter().map(|x| *x * *x).collect()
        };
        if sorted_squared[2] > sorted_squared[0] + sorted_squared[1] {
            'O'
        } else if sorted_squared[2] == sorted_squared[0] + sorted_squared[1] {
            'R'
        } else {
            'A'
        }
    }

    pub fn new<U>(a: U, b: U, c: U) -> Self
    where
        U: NumCast,
    {
        let a: T = num::cast(a).unwrap();
        let b: T = num::cast(b).unwrap();
        let c: T = num::cast(c).unwrap();
        Triangle { a, b, c }
    }
}
