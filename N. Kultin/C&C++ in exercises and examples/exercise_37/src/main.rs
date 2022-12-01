//! Написать программу, которая выводит на экран имена детей
//! Гомера и Мардж Симпсонов по-русски.

use std::io::Write;

fn main() {
    println!("Барт Симпсон");
    println!("Лиза Симпсон");
    println!("Мегги Симпсон");
    get_string("\nДля завершения нажмите <Enter>");
}

fn get_string(msg: &str) -> String {
    print!("{}", msg);
    std::io::stdout().flush().unwrap();
    get_input().trim().to_string()
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer
}