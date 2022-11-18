// &'static is a "lifetime specifier", something you'll learn more about later
pub fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    println!("{}", hello());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]

    fn test_hello_world() {
        assert_eq!("Hello, World!", hello());
    }
}
