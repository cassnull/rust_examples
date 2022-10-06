pub fn quick_sort<T: Ord + Copy>(array: &mut [T]) {
    sort(array, 0, array.len() - 1);
}

fn sort<T: Ord + Copy>(array: &mut [T], min_index: usize, max_index: usize) {
    if min_index < max_index {
        let pivot_index = partition(array, min_index, max_index);
        sort(array, min_index, pivot_index - 1);
        sort(array, pivot_index + 1, max_index);
    }
}

fn partition<T: Ord + Copy>(array: &mut [T], min_index: usize, max_index: usize) -> usize {
    let pivot = array[max_index];

    let mut i = min_index as i32 - 1;

    for j in min_index..max_index {
        if array[j] < pivot {
            i += 1;
            array.swap(i as usize, j);
        }
    }
    array.swap(i as usize + 1, max_index);
    i as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        quick_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
