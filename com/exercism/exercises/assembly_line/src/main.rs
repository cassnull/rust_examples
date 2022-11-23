// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// Calculate hourly production rate at speed.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    let rate = match speed {
        0 => 0.,
        n if n < 5 => 1.,
        n if n < 9 => 0.9,
        9 | 10 => 0.77,
        _ => panic!(),
    };
    speed as f64 * 221. * rate
}

/// Calculate the amount of working items at speed.
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.) as u32
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    fn process_rate_per_hour(speed: u8, expected_rate: f64) {
        let actual_rate = production_rate_per_hour(speed);
        let actual_rate = (actual_rate * 100.0).round() / 100.0;
        assert!((actual_rate - expected_rate).abs() < f64::EPSILON);
    }

    fn process_rate_per_minute(speed: u8, expected_rate: u32) {
        assert_eq!(working_items_per_minute(speed), expected_rate);
    }

    #[test]
    fn production_rate_per_hour_at_speed_zero() {
        process_rate_per_hour(0, 0.0);
    }

    #[test]
    fn production_rate_per_hour_at_speed_one() {
        process_rate_per_hour(1, 221.0);
    }

    #[test]
    fn production_rate_per_hour_at_speed_four() {
        process_rate_per_hour(4, 884.0);
    }

    #[test]
    fn production_rate_per_hour_at_speed_six() {
        process_rate_per_hour(6, 1193.4);
    }

    #[test]
    fn production_rate_per_hour_at_speed_seven() {
        process_rate_per_hour(7, 1392.3);
    }

    #[test]
    fn production_rate_per_hour_at_speed_nine() {
        process_rate_per_hour(9, 1531.53);
    }

    #[test]
    fn production_rate_per_minute_at_speed_zero() {
        process_rate_per_minute(0, 0);
    }

    #[test]
    fn production_rate_per_minute_at_speed_one() {
        process_rate_per_minute(1, 3);
    }

    #[test]
    fn production_rate_per_minute_at_speed_five() {
        process_rate_per_minute(5, 16);
    }

    #[test]
    fn production_rate_per_minute_at_speed_six() {
        process_rate_per_minute(6, 19);
    }

    #[test]
    fn production_rate_per_minute_at_speed_eight() {
        process_rate_per_minute(8, 26);
    }

    #[test]
    fn production_rate_per_minute_at_speed_ten() {
        process_rate_per_minute(10, 28);
    }
}
