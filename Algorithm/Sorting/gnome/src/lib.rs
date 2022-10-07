pub fn gnome_sort<T: Ord + Copy>(array: &mut [T]) {
    let mut index = 1_usize;
    let mut next_index = index + 1;

    while index < array.len() {
        if array[index - 1] < array[index] {
            index = next_index;
            next_index += 1;
        } else {
            array.swap(index - 1, index);
            index -= 1;
            if index == 0 {
                index = next_index;
                next_index += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gnome_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        gnome_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
