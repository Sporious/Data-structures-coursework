#[cfg(test)]
mod tests;

extern crate num;

use self::num::NumCast;
use std::f64::consts::E;
///Binary search
///Solve A * x + B * sqrt(x ^ 3) - C * exp(-x / 50) - D = 0
///Where 0 =< x <= 100
#[allow(dead_code)]
fn binary_search<T, U>(a_in: T, b_in: T, c_in: T, d_in: T, min_in: U, max_in: U) -> T
where
    T: NumCast,
    U: NumCast,
{
    let a: f64 = num::cast(a_in).unwrap();
    let b: f64 = num::cast(b_in).unwrap();
    let c: f64 = num::cast(c_in).unwrap();
    let d: f64 = num::cast(d_in).unwrap();

    let mut min: f64 = num::cast(min_in).unwrap();
    let mut max: f64 = num::cast(max_in).unwrap();

    let mut mid = get_mid(&min, &max);
    let target = 0_f64;
    let mut work;
    while {
        work = calc(mid, &a, &b, &c, &d);
        if work < target {
            min = mid;
        } else if work > target {
            max = mid;
        }
        mid = get_mid(&min, &max);
        cust_round(work) != target
    }
    {}
    let out: T = num::cast(mid).unwrap();
    out
}


#[allow(dead_code)]
pub fn cust_round(x: f64) -> f64 {
    let a = 10_i32.pow(9) as f64;
    (x * a).round().trunc() / a
}

#[allow(dead_code)]
fn get_mid(min: &f64, max: &f64) -> f64 {
    (max + min) / 2_f64
}

#[allow(dead_code)]
fn calc(x: f64, a: &f64, b: &f64, c: &f64, d: &f64) -> f64 {
    (a * x) + (b * x.powf(3_f64).sqrt()) - (c * E.powf((-x) / 50_f64)) - d
}
