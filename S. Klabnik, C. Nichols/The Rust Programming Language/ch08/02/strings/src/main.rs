use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Строки хранятся в кодировке UTF-8
    // Создание новых строк
    let mut _s = String::new();

    let data = "initial contents";
    let _s = data.to_string();

    // the method also works on a literal directly:
    let _s = "initial contents".to_string();

    let _s = String::from("initial contents");

    // Обновление строковых данных
    // Присоединение к строке с помощью push_str и push
    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}", s1); // s2 is foobar
    println!("s2 is {}", s2); // s2 is bar

    let mut _s = String::from("lo");
    _s.push('l'); // lol

    // Объединение строк с помощью оператора +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be usedcr
    println!("s2 is {}", s2); // s2 is world!
    println!("s3 is {}", s3); // s2 is Hello, world!

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // Объединение строк с помощью макроса format!
    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("s is {}", s); // s is tic-tac-toe

    // Срезы строк
    let hello = "Здравствуйте";

    let s = &hello[0..4];
    println!("s is {}", s); // s is Зд

    // Методы для перебора строк
    // Скалярные Unicode значения
    for (i, c) in "नमस्ते".chars().enumerate() {
        println!("{}: {}", i + 1, c);
    }
    // Байты u8
    for (i, b) in "नमस्ते".bytes().enumerate() {
        println!("{}: {}", i + 1, b);
    }
    // Кластеры графем
    for (i, g) in "नमस्ते्".graphemes(true).enumerate() {
        println!("{}: {}", i + 1, g);
    }
}
