pub struct Solution;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut result = 0;
        let mut prev_char = ' ';
        for ch in s.chars() {
            result += match ch {
                'I' => 1,
                'V' if prev_char == 'I' => 3,
                'V' => 5,
                'X' if prev_char == 'I' => 8,
                'X' => 10,
                'L' if prev_char == 'X' => 30,
                'L' => 50,
                'C' if prev_char == 'X' => 80,
                'C' => 100,
                'D' if prev_char == 'C' => 300,
                'D' => 500,
                'M' if prev_char == 'C' => 800,
                'M' => 1000,
                _ => 0,
            };
            prev_char = ch;
        }
        result
    }

    pub fn roman_to_int_2(s: String) -> i32 {
        s.chars().rfold(0, |acc, ch| {
            acc + match ch {
                'I' if acc >= 5 => -1,
                'I' => 1,
                'V' => 5,
                'X' if acc >= 50 => -10,
                'X' => 10,
                'L' => 50,
                'C' if acc >= 500 => -100,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::roman_to_int("III".into()), 3);
        assert_eq!(Solution::roman_to_int_2("III".into()), 3);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::roman_to_int("LVIII".into()), 58);
        assert_eq!(Solution::roman_to_int_2("LVIII".into()), 58);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::roman_to_int("MCMXCIV".into()), 1994);
        assert_eq!(Solution::roman_to_int_2("MCMXCIV".into()), 1994);
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::roman_to_int("IV".into()), 4);
        assert_eq!(Solution::roman_to_int_2("IV".into()), 4);
    }
}
