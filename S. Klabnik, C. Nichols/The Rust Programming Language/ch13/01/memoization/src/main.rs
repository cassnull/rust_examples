use std::collections::HashMap;
use std::hash::Hash;
use std::thread;
use std::time::Duration;

struct Cacher<F, U, V>
where
    F: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    calculation: F,
    values: HashMap<U, V>,
}

impl<F, U, V> Cacher<F, U, V>
where
    F: Fn(U) -> V,
    U: Eq + Hash + Copy,
    V: Copy,
{
    fn new(calculation: F) -> Cacher<F, U, V> {
        Cacher {
            calculation,
            values: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let value_entry = self.values.entry(arg);
        *value_entry.or_insert_with_key(|&key| (self.calculation)(key))
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else if random_number == 3 {
        println!("Take a break today! Remember to stay hydrated!");
    } else {
        println!(
            "Today, run for {} minutes!",
            expensive_result.value(intensity)
        );
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v1, 1);
        assert_eq!(v2, 2);
    }

    #[test]
    fn test_cacher_basically_works() {
        let mut word_len = Cacher::new(|word: &str| word.len());
        let hello = word_len.value("hello");

        // Test function returns correctly
        assert_eq!(hello, 5);

        // Test HashMap is correct length
        assert_eq!(word_len.values.len(), 1);

        // Test HashMap has correct value after one insert
        let mut test_map = HashMap::new();
        test_map.insert("hello", 5);
        assert_eq!(word_len.values, test_map);

        // Test HashMap has correct value after duplicate insert
        word_len.value("hello");
        assert_eq!(word_len.values, test_map);

        // Test HashMap has correct values after unique input
        word_len.value("wazzup");
        test_map.insert("wazzup", 6);
        assert_eq!(word_len.values, test_map);
    }

    #[test]
    fn test_cacher_speed() {
        // Simulate a function that takes 1 second to complete
        let mut func = Cacher::new(|x| {
            std::thread::sleep(std::time::Duration::from_millis(100));
            x * x
        });

        // Would take 10 minutes without caching
        for _ in 0..6000 {
            assert_eq!(25, func.value(5));
        }
    }
}
