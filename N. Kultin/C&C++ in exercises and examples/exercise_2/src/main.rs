/// Объявить переменные, необходимые для пересчета веса из фунтов в килограммы.

fn main() {
    let funt = 0.;
    let kg = 0.;

    println!("{}", type_of(&funt));
    println!("{}", type_of(&kg));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
