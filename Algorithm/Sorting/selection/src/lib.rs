pub fn selection_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len {
        let mut min = i;
        for j in i + 1..len {
            if array[j] < array[min] {
                min = j;
            }
        }
        array.swap(i, min);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_selection_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        selection_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
