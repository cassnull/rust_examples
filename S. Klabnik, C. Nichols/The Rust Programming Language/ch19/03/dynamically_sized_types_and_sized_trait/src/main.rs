#[allow(dead_code)]
fn generic<T: Sized>(_t: T) {
    // --snip--
}

#[allow(dead_code)]
fn generic_taking_unsized_param<T: ?Sized>(_t: &T) {
    // --snip--
}

fn main() {
    let s1 = &"Hello there";
    println!("s1 = {}", s1);
}
