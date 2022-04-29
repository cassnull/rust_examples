use std::fmt::Debug;
use std::fmt::Display;

// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
#[allow(dead_code)]
fn printer<T: Display>(t: T) {
    println!("{}", t);
}

struct S<T: Display>(T);

trait HasArea {
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

#[allow(dead_code)]
struct Triangle {
    length: f64,
    height: f64,
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
#[allow(dead_code)]
fn area<T: HasArea>(t: &T) -> f64 {
    t.area()
}

fn main() {
    // Error! `Vec<T>` does not implement `Display`. This
    // specialization will fail.
    // let _s = S(vec![1]);
    let _s = S(1);

    let rectangle = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    let _triangle = Triangle {
        length: 3.0,
        height: 4.0,
    };

    print_debug(&rectangle);
    println!("Area: {}", rectangle.area());

    // print_debug(&_triangle);
    // println!("Area: {}", _triangle.area());
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`
}
