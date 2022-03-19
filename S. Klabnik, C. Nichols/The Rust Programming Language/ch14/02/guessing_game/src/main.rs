use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let mut rng = rand::thread_rng(); // специальный генератор случайных чисел
    let secret_number = rng.gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        let stdin = io::stdin(); // дескриптор стандартного ввода для вашего терминала
        let io_read_line_result = stdin.read_line(&mut guess);
        io_read_line_result.expect("Failed to read line"); // количество байтов в пользовательском вводе

        let guess_parse_result = guess
            .trim() // удалит любые пробельные символы в начале и конце строки
            .parse(); // преобразует строку в некоторое число

        let guess: u32 = match guess_parse_result {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
