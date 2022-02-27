// В один момент времени, может существовать либо одна изменяемая ссылочная переменная, либо любое количество неизменяемых ссылочных переменных.
// Все ссылки должны быть действительными.

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");

    change(&mut s);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
