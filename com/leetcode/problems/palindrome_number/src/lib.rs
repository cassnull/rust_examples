pub struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        x.to_string() == x.to_string().chars().rev().collect::<String>()
    }

    pub fn is_palindrome_2(x: i32) -> bool {
        if x < 0 || x % 10 == 0 && x != 0 {
            return false;
        }
        if x < 10 {
            return true;
        }
        let mut d = 0;
        let mut x = x;
        while x > d {
            d = d * 10 + x % 10;
            x /= 10;
        }
        x == d || x == d / 10
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_palindrome(121), true);
        assert_eq!(Solution::is_palindrome_2(121), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_palindrome(-121), false);
        assert_eq!(Solution::is_palindrome_2(-121), false);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_palindrome(10), false);
        assert_eq!(Solution::is_palindrome_2(10), false);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::is_palindrome(0), true);
        assert_eq!(Solution::is_palindrome_2(0), true);
    }

    #[test]
    fn example_5() {
        assert_eq!(Solution::is_palindrome(1), true);
        assert_eq!(Solution::is_palindrome_2(1), true);
    }

    #[test]
    fn example_6() {
        assert_eq!(Solution::is_palindrome(11), true);
        assert_eq!(Solution::is_palindrome_2(11), true);
    }
}
