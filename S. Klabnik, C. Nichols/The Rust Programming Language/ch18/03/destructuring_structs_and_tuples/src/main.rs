#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let ((_feet, _inches), Point { x: _, y: _ }) = ((3, 10), Point { x: 3, y: -10 });
}
