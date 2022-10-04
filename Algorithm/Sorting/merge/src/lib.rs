pub fn merge_sort<T: Ord + Copy>(array: &mut [T]) {
    sort(array, 0, array.len() - 1);
}

fn sort<T: Ord + Copy>(array: &mut [T], left: usize, right: usize) {
    if left < right {
        let middle = left + (right - left) / 2;
        sort(array, left, middle);
        sort(array, middle + 1, right);
        merge(array, left, middle, right);
    }
}

fn merge<T: Ord + Copy>(array: &mut [T], left: usize, middle: usize, right: usize) {
    let left_array_length = middle - left + 1;
    let right_array_length = right - middle;
    let mut left_temp_array = Vec::with_capacity(left_array_length);
    let mut right_temp_array = Vec::with_capacity(right_array_length);
    for i in 0..left_array_length {
        left_temp_array.push(array[left + i]);
    }
    for j in 0..right_array_length {
        right_temp_array.push(array[middle + 1 + j]);
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = left;
    while i < left_array_length && j < right_array_length {
        if left_temp_array[i] <= right_temp_array[j] {
            array[k] = left_temp_array[i];
            k += 1;
            i += 1;
        } else {
            array[k] = right_temp_array[j];
            k += 1;
            j += 1;
        }
    }
    while i < left_array_length {
        array[k] = left_temp_array[i];
        k += 1;
        i += 1;
    }
    while j < right_array_length {
        array[k] = right_temp_array[j];
        k += 1;
        j += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_merge_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        merge_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
