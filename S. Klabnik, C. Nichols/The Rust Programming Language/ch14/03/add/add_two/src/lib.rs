//! # Add Two
//!
//! `add_two` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds two to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_two::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(x: i32) -> i32 {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(292, add_two(290));
    }
}
