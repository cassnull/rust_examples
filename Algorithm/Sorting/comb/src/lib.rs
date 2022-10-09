pub fn comb_sort<T: Ord + Copy>(array: &mut [T]) {
    let array_length = array.len();
    let mut current_step = array_length;
    let mut swap_flag = false;
    while current_step > 1 || swap_flag {
        current_step = get_next_step(current_step);
        let mut i = 0;
        let mut j = current_step;
        swap_flag = false;
        while j < array_length {
            if array[i] > array[j] {
                array.swap(i, j);
                swap_flag = true;
            }
            i += 1;
            j = i + current_step;
        }
    }
}

fn get_next_step(current_step: usize) -> usize {
    let gap = current_step as f64 / 1.2473309;
    match gap {
        _ if gap < 1. => 1,
        _ => gap as usize,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_comb_sort() {
        let mut v = vec![4, 6, 1, 8, 11, 13, 3];
        comb_sort(&mut v);
        assert_eq!(v, vec![1, 3, 4, 6, 8, 11, 13])
    }
}
