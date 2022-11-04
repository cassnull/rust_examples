fn main() {
    println!("Что получится, если скрестить снеговика с вампиром?");
    get_input();
    println!("Обморожение!");
    println!();
    println!("Что дантисты называют \"черной дырой\"?");
    get_input();
    println!("Кариес!");
    println!();
    println!("Тук-тук.");
    get_input();
    println!("Кто там?");
    get_input();
    println!("Невежливая корова.");
    get_input();
    print!("Невежливая корова?");
    println!("-МУУУ!");
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");
    buffer
}
