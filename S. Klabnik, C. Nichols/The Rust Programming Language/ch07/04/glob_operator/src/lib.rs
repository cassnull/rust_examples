pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*; // Оператор * (Glob)

    #[test]
    fn it_works() {
        assert_eq!(292, add_one(291));
    }
}
