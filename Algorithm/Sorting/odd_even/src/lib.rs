pub fn odd_even_sort<T: Ord>(array: &mut [T]) {
    let mut flag = false;

    while !flag {
        flag = true;

        for i in (0..array.len() - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                flag = false;
            }
        }

        for i in (1..array.len() - 1).step_by(2) {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                flag = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_odd_even_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        odd_even_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
