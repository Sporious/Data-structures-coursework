extern crate round;

#[test]
fn test_binary_search() {
    test_binary_search_f32();
    test_binary_search_f64();
}

fn test_binary_search_f32() {
    let (a, b, c, d, min, max) = (
        0.59912051_f32,
        0.64030348_f32,
        263.33721367_f32,
        387.92069617_f32,
        0,
        100,
    );
    let result: f32 = super::binary_search(a, b, c, d, min, max).trunc();
    assert_eq!(result, 73_f32);

}
fn test_binary_search_f64() {
    let (a, b, c, d, min, max) = (
        0.59912051_f64,
        0.64030348_f64,
        263.33721367_f64,
        387.92069617_f64,
        0,
        100,
    );
    let result = round::round(super::binary_search(a, b, c, d, min, max), 5);
    assert_eq!(result, round::round(73.59536855416_f64, 5));

}
