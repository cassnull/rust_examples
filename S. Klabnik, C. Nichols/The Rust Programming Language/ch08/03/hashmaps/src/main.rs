use std::collections::HashMap;

fn main() {
    // Создание новой хеш-карты
    let mut _scores: HashMap<String, i32> = HashMap::new();

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let _scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // Обновление хеш-карты
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Хеш-карты и владение
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // Доступ к данным в HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name); // Some(&10)

    // Перебор HashMap
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Обновление данных
    // Перезапись старых значений
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores); // {"Blue": 25}

    // Вставка значения только в том случае, когда ключ не имеет значения
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    // Вставит ключ для команды "Yellow" со значением 50, потому что для жёлтой команды ещё не имеется значения в HashMap
    let new_value = scores.entry(String::from("Yellow")).or_insert(50);
    println!("{}", new_value); // 50
    // Не изменит хеш-карту, потому что для ключа команды "Blue" уже имеется значение 10
    let old_value = scores.entry(String::from("Blue")).or_insert(50);
    println!("{}", old_value); // 10

    println!("{:?}", scores); // {"Blue": 10, "Yellow": 50}

    // Создание нового значения на основе старого значения
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map); // {"wonderful": 1, "world": 2, "hello": 1}

}
