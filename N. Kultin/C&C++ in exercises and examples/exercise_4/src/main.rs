/// Объявить переменные, необходимые для вычисления площади круга.

fn main() {
    let r = 0.;
    let s = 0.;
    println!("{}", type_of(&r));
    println!("{}", type_of(&s));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
