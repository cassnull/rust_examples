pub fn heapify_sort<T: Ord>(array: &mut [T]) {
    for i in (0..array.len() / 2).rev() {
        sort(array, array.len(), i);
    }

    for i in (0..array.len()).rev() {
        array.swap(0, i);
        sort(array, i, 0);
    }
}

fn sort<T: Ord>(array: &mut [T], n: usize, i: usize) {
    let mut largest = i;
    let left = 2 * i + 1;
    let right = 2 * i + 2;

    if left < n && array[left] > array[largest] {
        largest = left;
    }
    if right < n && array[right] > array[largest] {
        largest = right;
    }
    if largest != i {
        array.swap(i, largest);
        sort(array, n, largest);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heapify_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        heapify_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
