/// Записать в виде инструкции присваивания, формулы вычисления
/// площади поверхности и объема цилиндара.

fn main() {
    let r = 12.;
    let h = 34.;
    let s = 2. * std::f64::consts::PI * r * (r + h);
    let v = std::f64::consts::PI * r.powi(2) * h;
    println!("{}", s);
    println!("{}", v);
}
