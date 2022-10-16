pub fn cocktail_sort<T: Ord>(array: &mut [T]) {
    let mut start = 0;
    let mut end = array.len() - 1;
    let mut swapped = true;

    while swapped {
        swapped = false;

        for i in 0..end {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }

        swapped = false;
        end -= 1;

        for i in (start..end).rev() {
            if array[i] > array[i + 1] {
                array.swap(i, i + 1);
                swapped = true;
            }
        }

        start += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cocktail_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        cocktail_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
