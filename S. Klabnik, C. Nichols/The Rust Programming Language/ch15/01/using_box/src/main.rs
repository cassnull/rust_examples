#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("box = {:?}", list);
}
