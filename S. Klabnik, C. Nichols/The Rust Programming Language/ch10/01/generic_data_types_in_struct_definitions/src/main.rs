#[allow(dead_code)]
struct Point<T, U> {
    x: T,
    y: U,
}

fn main() {
    let _both_integer = Point { x: 5, y: 10 };
    let _both_float = Point { x: 1.0, y: 4.0 };
    let _integer_and_float = Point { x: 5, y: 4.0 };
}
