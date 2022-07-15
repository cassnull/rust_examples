/// Объявить переменные, необходимые для вычисления площади прямоугольника.

fn main() {
    let (a, b) = (0., 0.);
    let s: f64 = 0.;

    println!("{} {}", type_of(&a), type_of(&b));
    println!("{}", type_of(&s));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
