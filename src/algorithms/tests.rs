//Fibonacci
#[test]
fn test_fibonacci_nth() {
    let mut a = super::fibonacci::Fibonacci::default();
    assert_eq!(a.get_nth_fibonacci(6), 8);
}
#[test]
fn test_fibonacci_output() {
    let mut a = super::fibonacci::Fibonacci::default();
    assert_eq!(a.out_n_fibonacci(6).trim(), "1 1 2 3 5 8".trim());
}
#[test]
fn test_fibonacci_first_over() {
    let mut a = super::fibonacci::Fibonacci::default();
    let (a, b) = a.first_fibonacci_over(1000);
    assert_eq!(a, 1597);
    assert_eq!(b, 17);
}

//Collatz
#[test]
fn test_collatz() {
    assert_eq!(super::collatz::collatz(97), 118);
}

//Pythagoras
#[test]
fn test_pythagoras_triangletype() {
    use super::pythagoras::Triangle;
    let triangle: Triangle<u32> = Triangle::new(16, 12, 22);
    assert_eq!(triangle.get_triangle_type(), 'O');
    let triangle: Triangle<u16> = Triangle::new(9, 12, 15);
    assert_eq!(triangle.get_triangle_type(), 'R');
    let triangle: Triangle<u64> = Triangle::new(6, 8, 9);
    assert_eq!(triangle.get_triangle_type(), 'A');

}

//Lin Random
#[test]
fn test_linear_random() {
    assert_eq!(super::linear_random::random(3, 7, 12, 1, 2), 1);
    assert_eq!(super::linear_random::random(2, 3, 15, 8, 10), 11);
}
//Luhns - Incomplete
#[test]
fn test_luhns_valid_number() {
    use super::luhn::*;
    let a = Card::new("79927398713");
    assert!(a.valid);
}
#[test]
fn test_luhns_invalid_number() {
    use super::luhn::*;
    let a = Card::new("79927398714");
    assert!(!a.valid);
}
