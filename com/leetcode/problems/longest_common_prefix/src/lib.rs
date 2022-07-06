pub struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
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

    pub fn longest_common_prefix_2(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        fn func(strs: &Vec<String>, l: usize, r: usize) -> String {
            if l == r {
                return strs[l].clone();
            }
            let mid = (l + r) / 2;
            let lcp_left: Vec<char> = func(strs, l, mid).chars().collect();
            let lcp_right: Vec<char> = func(strs, mid + 1, r).chars().collect();
            let min = lcp_left.len().min(lcp_right.len());
            for i in 0..min {
                if lcp_left[i] != lcp_right[i] {
                    return lcp_left[0..i].iter().collect();
                }
            }
            lcp_left[0..min].iter().collect()
        }

        func(&strs, 0, strs.len() - 1)
    }

    pub fn longest_common_prefix_3(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }

        fn func(strs: &[String], len: usize) -> bool {
            let str1 = strs[0][0..len].to_string();
            println!("{}", str1);
            strs.iter().all(|str| str.starts_with(&str1))
        }

        let min = strs.iter().map(|str| str.len()).min().unwrap();

        let mut low = 0;
        let mut hight = min;
        while low <= hight {
            let middle = (low + hight) / 2;
            if func(&strs, middle) {
                low = middle + 1;
            } else {
                hight = if middle == 0 { 0 } else { middle - 1 };
            }
        }
        strs[0][0..(low + hight) / 2].to_string()
    }

    pub fn longest_common(strs: Vec<String>) -> String {
        if strs.is_empty() {
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
        assert_eq!(
            Solution::longest_common_prefix_2(vec![
                "flower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            "fl"
        );
        assert_eq!(
            Solution::longest_common_prefix_3(vec![
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
        assert_eq!(
            Solution::longest_common_prefix_2(vec![
                "dog".to_string(),
                "racecar".to_string(),
                "car".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix_3(vec![
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
        assert_eq!(
            Solution::longest_common_prefix_2(vec![
                "reflower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            ""
        );
        assert_eq!(
            Solution::longest_common_prefix_3(vec![
                "reflower".to_string(),
                "flow".to_string(),
                "flight".to_string()
            ]),
            ""
        );
    }

    #[test]
    fn example_4() {
        assert_eq!(Solution::longest_common_prefix(vec!["a".to_string()]), "a");
        assert_eq!(
            Solution::longest_common_prefix_2(vec!["a".to_string()]),
            "a"
        );
        assert_eq!(
            Solution::longest_common_prefix_3(vec!["a".to_string()]),
            "a"
        );
    }
}
