/// Объявить переменные, необходимые для вычисления площади кольца.

fn main() {
    let (r1, r2) = (0., 0.);
    let s = 0.;

    println!("{}", type_of(&r1));
    println!("{}", type_of(&r2));
    println!("{}", type_of(&s));
}

fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}
