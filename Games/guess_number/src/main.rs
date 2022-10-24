use rand::Rng;
use std::num::ParseIntError;

type ParseResult<T> = Result<T, ParseIntError>;

fn main() {
    let (min, max) = (1, 20);

    println!("Привет! Как тебя зовут?");
    let my_name = get_string();

    let mut rng = rand::thread_rng();
    let number = rng.gen_range(min..=max);
    println!(
        "Что ж, {}, я загадываю число от {} до {}.",
        my_name, min, max
    );

    let mut guess = 0;
    let mut guesses_taken = 0;

    for i in 0..6 {
        println!("Попробуй угадать.");
        guess = get_i32().unwrap();

        guesses_taken = i + 1;

        if guess < number {
            println!("Твое число слишком маленькое.");
        }

        if guess > number {
            println!("Твое число слишком большое.");
        }

        if guess == number {
            break;
        }
    }
    if guess == number {
        println!(
            "Отлично, {}! Ты справился за {} попытки!",
            my_name, guesses_taken
        );
    }

    if guess != number {
        println!("Увы. Я загадала число {}.", number);
    }
}

fn get_i32() -> ParseResult<i32> {
    get_input().trim().parse()
}

fn get_string() -> String {
    get_input().trim().to_string()
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer
}
