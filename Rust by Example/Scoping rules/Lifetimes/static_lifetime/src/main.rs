// 'static as part of a trait bound:
#[allow(dead_code)]
fn generic<T>(_x: T)
where
    T: 'static,
{
}

fn main() {
    // A reference with 'static lifetime:
    let _s: &'static str = "hello world";
}
