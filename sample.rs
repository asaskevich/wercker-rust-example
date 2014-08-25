fn return_two() -> int {
    2
}

#[test]
fn return_two_test() {
    let x = return_two();
    assert!(x == 3);
}
