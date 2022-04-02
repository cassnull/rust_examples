#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

fn main() {
    let origin = Point { x: 16, y: 25, z: 292 };

    let Point { y, .. } = origin;
    println!("y is {}", y);

    let numbers = (2, 4, 8, 16, 32);

    let (first, .., last) = numbers;
    println!("Some numbers: {}, {}", first, last);
}
