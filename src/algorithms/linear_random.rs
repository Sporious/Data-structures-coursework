pub fn random(a: i32, c: i32, m: i32, mut x: i32, n: i32 ) -> i32 {
    let mut counter = 0;
    while counter < n {
        x = (x * a + c) % m;
        counter += 1;
    }
    x
}
