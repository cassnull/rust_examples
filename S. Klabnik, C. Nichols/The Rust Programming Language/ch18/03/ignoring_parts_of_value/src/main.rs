fn main() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);

    let numbers = (2, 4, 8, 16, 32);
    let (first, _, third, _, fifth) = numbers;
    println!("Some numbers: {}, {}, {}", first, third, fifth);

    let s = Some(String::from("Hello!"));

    // Синтаксис _x привязывает значение к переменной
    if let Some(_s) = s {
        println!("found a string");
    }
}
