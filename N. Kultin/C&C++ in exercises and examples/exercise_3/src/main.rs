/// Определить исходные данные и объявить переменные, необходимые для вычисления доходов по вкладу.

fn main() {
    let summa = 0.;
    let srok = 0;
    let stavka = 0;
    let dohod = 0.;

    println!("{}", type_of(&summa));
    println!("{}", type_of(&srok));
    println!("{}", type_of(&stavka));
    println!("{}", type_of(&dohod));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
