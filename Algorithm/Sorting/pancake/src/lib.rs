pub fn pancake_sort<T: Ord>(array: &mut [T]) {
    for i in (0..array.len()).rev() {
        let pos = max_value_pos(array, i);

        if pos == i {
            continue;
        }

        if pos != 0 {
            flip(array, pos + 1);
        }

        flip(array, i + 1);
    }
}

fn max_value_pos<T: Ord>(array: &mut [T], end: usize) -> usize {
    let mut pos = end;
    for j in 0..end {
        if array[j] > array[pos] {
            pos = j;
        }
    }
    pos
} 

fn flip<T>(array: &mut [T], pos: usize) {
    let mut n = pos;
    let mut i = 0;
    while i < n {
        n -= 1;
        array.swap(i, n);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pancake_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        pancake_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
