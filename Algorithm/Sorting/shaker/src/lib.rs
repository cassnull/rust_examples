pub fn snaker_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    let mut swap_flag = false;
    for i in 0..len / 2 {
        for j in i..len - i - 1 {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
                swap_flag = true;
            }
        }

        for j in (i + 1..len - i - 1).rev() {
            if array[j - 1] > array[j] {
                array.swap(j - 1, j);
                swap_flag = true;
            }
        }
        if !swap_flag {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snaker_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        snaker_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
