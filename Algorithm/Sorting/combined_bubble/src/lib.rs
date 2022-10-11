pub fn combined_bubble_sort<T: Ord>(array: &mut [T]) {
    let length = array.len();

    for i in 0..length {
        for j in i + 1..length {
            if array[i] > array[j] {
                array.swap(i, j);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combined_bubble_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        combined_bubble_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
