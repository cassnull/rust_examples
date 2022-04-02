#[allow(dead_code)]
fn bar() -> ! {
    // --snip--
    print!("forever ");

    loop {
        print!("and ever ");
    }
}

#[allow(dead_code)]
enum MyOption<T> {
    None,
    Some(T),
}

#[allow(dead_code)]
impl<T> MyOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            MyOption::Some(val) => val,
            MyOption::None => panic!("called `MyOption::unwrap()` on a `None` value"),
        }
    }
}

#[allow(clippy::while_let_loop)]
fn main() {
    loop {
        let _result: u32 = match "error".trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };
    }
}
