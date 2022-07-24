/// Объявить переменные, необходимые для вычисления стоимости покупки,
/// состоящей из нескольких тетрадей и такого же количества обложек.

fn main() {
    let cena_tetr = 0.;
    let cena_obl = 0.;
    let kol = 0;
    let summa = 0.;
    println!("{}", type_of(&cena_tetr));
    println!("{}", type_of(&cena_obl));
    println!("{}", type_of(&kol));
    println!("{}", type_of(&summa));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
