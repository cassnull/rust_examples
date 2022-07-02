pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, item) in nums.iter().enumerate() {
            map.insert(item, i);
        }
        for (i, item) in nums.iter().enumerate() {
            let complement = target - item;
            if let Some(&other) = map.get(&complement) {
                if other != i {
                    return vec![i as i32, other as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for (i, item) in nums.iter().enumerate() {
            let complement = target - item;
            if let Some(&other) = map.get(&complement) {
                if other != i {
                    return vec![other as i32, i as i32];
                }
            }
            map.insert(item, i);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_2(vec![2, 7, 11, 15], 9), vec![0, 1]);
        assert_eq!(Solution::two_sum_3(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }

    #[test]
    fn example_2() {
        assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_2(vec![3, 2, 4], 6), vec![1, 2]);
        assert_eq!(Solution::two_sum_3(vec![3, 2, 4], 6), vec![1, 2]);
    }

    #[test]
    fn example_3() {
        assert_eq!(Solution::two_sum(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum_2(vec![3, 3], 6), vec![0, 1]);
        assert_eq!(Solution::two_sum_3(vec![3, 3], 6), vec![0, 1]);
    }
}
