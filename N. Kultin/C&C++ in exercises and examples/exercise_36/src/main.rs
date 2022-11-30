//! Написать программу, которая выводит на экран имя детей Гомера и
//! Мардж Симпсонов (имя каждого - с новой строки).

use std::io::Write;

fn main() {
    println!("Bart Simpson");
    println!("Lisa Simpson");
    println!("Maggi Simpson");
    get_string("To exit press <Enter>");
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
