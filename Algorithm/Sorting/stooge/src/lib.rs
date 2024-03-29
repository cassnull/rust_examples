pub fn stooge_sort<T: Ord + Copy>(array: &mut [T]) {
    sort(array, 0, array.len() - 1)
}

fn sort<T: Ord + Copy>(array: &mut [T], start_index: usize, end_index: usize) {
    if array[start_index] > array[end_index] {
        array.swap(start_index, end_index);
    }

    if end_index - start_index <= 1 {
        return;
    }

    let len = (end_index - start_index + 1) / 3;
    sort(array, start_index, end_index - len);
    sort(array, start_index + len, end_index);
    sort(array, start_index, end_index - len);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stooge_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        stooge_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
