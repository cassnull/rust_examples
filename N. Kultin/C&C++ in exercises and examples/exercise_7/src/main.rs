/// Объявить переменные, необходимые для вычисления стоимости покупки,
/// состоящей из нескольких тетрадей, карандашей и линейки.

fn main() {
    let cena_tetr = 0.;
    let kol_tetr = 0;
    let cena_kar = 0.;
    let kol_kar = 0;
    let cena_lin = 0.;
    let summa = 0.;
    println!("{}", type_of(&cena_tetr));
    println!("{}", type_of(&kol_tetr));
    println!("{}", type_of(&cena_kar));
    println!("{}", type_of(&kol_kar));
    println!("{}", type_of(&cena_lin));
    println!("{}", type_of(&summa));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
