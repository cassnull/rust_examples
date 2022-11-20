// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// Return expected minutes in the oven.
pub fn expected_minutes_in_oven() -> i32 {
    40
}

/// Calculate remaining minutes in oven given actual minutes in oven.
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    expected_minutes_in_oven() - actual_minutes_in_oven
}

/// Calculate preparation time in minutes for number of layers.
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    number_of_layers * 2
}

/// Calculate elapsed time in minutes for number of layers and actual minutes in oven.
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn expected_minutes_in_oven_is_correct() {
        assert_eq!(40, expected_minutes_in_oven());
    }

    #[test]
    fn remaining_minutes_in_oven_after_fifteen_minutes() {
        assert_eq!(15, remaining_minutes_in_oven(25));
    }

    #[test]
    fn preparation_time_in_minutes_for_one_layer() {
        assert_eq!(2, preparation_time_in_minutes(1));
    }

    #[test]
    fn preparation_time_in_minutes_for_multiple_layers() {
        assert_eq!(8, preparation_time_in_minutes(4));
    }

    #[test]
    fn elapsed_time_in_minutes_for_one_layer() {
        assert_eq!(32, elapsed_time_in_minutes(1, 30));
    }

    #[test]
    fn elapsed_time_in_minutes_for_multiple_layers() {
        assert_eq!(16, elapsed_time_in_minutes(4, 8));
    }
}
