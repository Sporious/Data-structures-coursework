#[test]
fn test_selection_sort() {
    let mut a = vec![-1, -2, 3, 6, 3];
    super::selection_sort(&mut a);
    assertions(&mut a);
}
#[test]
fn test_bubble_sort() {
    let mut a = vec![-1, -2, 3, 6, 3];
    super::bubble_sort(&mut a);
    assertions(&mut a);
}
#[test]
fn test_insertion_sort() {
    let mut a = vec![-1, -2, 3, 6, 3];
    super::insertion_sort(&mut a);
    assertions(&mut a);
}
#[test]
fn test_quicksort_left_to_right() {
    let mut a = vec![-1, -2, 3, 6, 3];
    super::quicksort_1(0, a.len(), &mut a);
    assertions(&mut a);
}
#[test]
fn test_quicksort_right_to_left() {
    let mut a = vec![-1, -2, 3, 6, 3];
    super::quicksort_2(0, a.len() - 1, &mut a);
    assertions(&mut a);
}

fn assertions(a: &mut Vec<i32>) {
    assert!(a[0] == -2, "{:?}", a);
    assert!(a[1] == -1, "{:?}", a);
    assert!(a[2] == 3, "{:?}", a);
    assert!(a[3] == 3, "{:?}", a);
    assert!(a[4] == 6, "{:?}", a);

}
