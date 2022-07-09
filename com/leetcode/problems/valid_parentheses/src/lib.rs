use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let chars = s.chars();
        let mut stack = VecDeque::with_capacity(s.len() as usize);
        let open_brackets = ['(', '{', '['];
        for bracket in chars {
            if open_brackets.contains(&bracket) || stack.is_empty() {
                stack.push_front(bracket);
                continue;
            }
            let open_bracket = match bracket {
                ')' => '(',
                '}' => '{',
                ']' => '[',
                _ => panic!("Invalid character"),
            };
            if open_bracket != stack[0] {
                return false;
            }
            stack.pop_front();
        }
        stack.is_empty()
    }

    pub fn is_valid_2(s: String) -> bool {
        let mut stack = Vec::new();
        for i in s.chars() {
            match i {
                '{' => stack.push('}'),
                '(' => stack.push(')'),
                '[' => stack.push(']'),
                _ => {
                    if Some(i) != stack.pop() {
                        return false;
                    }
                }
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::is_valid("()".into()), true);
        assert_eq!(Solution::is_valid_2("()".into()), true);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::is_valid("()[]{}".into()), true);
        assert_eq!(Solution::is_valid_2("()[]{}".into()), true);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::is_valid("(]".into()), false);
        assert_eq!(Solution::is_valid_2("(]".into()), false);
    }
}
