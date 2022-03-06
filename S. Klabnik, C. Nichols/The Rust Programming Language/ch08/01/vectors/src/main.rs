#[allow(clippy::vec_init_then_push)]
fn main() {
    // Создание нового вектора
    let _v: Vec<i32> = Vec::new();

    let _v = vec![1, 2, 3];

    // Изменение вектора
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    {
        let _v = vec![1, 2, 3, 4];

        // do stuff with _v
    } // <- _v goes out of scope and is freed here

    // Чтение данных вектора
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Перебор значений в векторе
    let v = vec![100, 32, 57];

    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];

    for i in &mut v {
        *i += 50;
    }

    // Использование перечислений для хранения множества разных типов
    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
