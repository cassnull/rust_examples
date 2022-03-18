fn main() {
    let x = 292;

    let equal_to_x = |z| z == x;

    let y = 292;

    assert!(equal_to_x(y));
}