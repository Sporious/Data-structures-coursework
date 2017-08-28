#[cfg(test)]
mod tests;

extern crate num;
extern crate round;

use self::num::NumCast;
pub fn f_to_c<T>(i: T) -> f64
where T: NumCast {
    let i: f64 = num::cast(i).unwrap();
    round::round((i - 32_f64) * 5_f64/9_f64, 2)
}
pub fn c_to_f<T>(i: T) -> f64
where T: NumCast {
    let i: f64 = num::cast(i).unwrap();
    round::round(i * 9_f64/5_f64 + 32_f64, 2)
}
