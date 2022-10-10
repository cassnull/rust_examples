pub fn counting_sort(array: &mut [i32]) {
    let n = array.len();
    let max = *array.iter().max().unwrap();
    let mut freq = vec![0; max as usize + 1];
    for i in 0..n {
        freq[array[i] as usize] += 1;
    }
    let mut j = 0;
    for i in 0..=max {
        while freq[i as usize] > 0 {
            array[j] = i;
            j += 1;
            freq[i as usize] -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counting_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        counting_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
