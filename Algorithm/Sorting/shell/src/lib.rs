pub fn shell_sort<T: Ord>(array: &mut [T]) {
    let mut d = array.len() / 2;
    while d >= 1 {
        for i in d..array.len() {
            let mut j = i;
            while (j >= d) && (array[j - d] > array[j]) {
                array.swap(j, j - d);
                j -= d;
            }
        }

        d /= 2;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        shell_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
