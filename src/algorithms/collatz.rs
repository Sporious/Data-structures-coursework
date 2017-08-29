pub fn collatz(mut a: i32) -> i32 {

    let mut i = 0;
    while {
        collatz_impl(&mut a);
        i += 1;
        a != 1

    }
    {}
    i
}
fn collatz_impl(i: &mut i32) {
    if *i % 2 == 0 {
        *i /= 2;
    } else {
        *i = 3 * *i + 1;
    }
}
