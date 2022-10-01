pub fn insertion_sort<T: Ord + Copy>(array: &mut [T]) {
    let len = array.len();
    for i in 1..len {
        let key = array[i];
        let mut j = (i - 1) as i32;

        while j >= 0 && array[j as usize] > key {
            array[j as usize + 1] = array[j as usize];
            j -= 1;
        }

        array[(j + 1) as usize] = key;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        insertion_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
