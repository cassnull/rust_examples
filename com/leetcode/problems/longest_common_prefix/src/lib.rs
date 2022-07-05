pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let mut index = 0;
        for i in 1..strs.len() {
            if strs[i].len() < strs[index].len() {
                index = i
            }
        }
        let min_str = strs[index].clone();
        for i in 0..min_str.len() {
            let template = min_str[0..min_str.len() - i].to_string();
            if strs.iter().filter(|&s| s.starts_with(&template)).count() == strs.len() {
                return template;
            }
        }
        "".to_string()
    }

    pub fn longest_common(strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return "".to_string();
        }
        let mut index = 0;
        for i in 1..strs.len() {
            if strs[i].len() < strs[index].len() {
                index = i
            }
        }
        let min_str = strs[index].clone();
        for i in (1..=min_str.len()).rev() {
            for j in 0..=min_str.len() - i {
                let template = min_str[j..j + i].to_string();
                if strs.iter().filter(|&s| s.contains(&template)).count() == strs.len() {
                    return template;
                }
            }
        }
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
    }

    #[test]
    fn example_2() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
    }

    #[test]
    fn example_3() {
        assert_eq!(
            Solution::longest_common_prefix(vec![
                "reflower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            ""
        );
    }
}
