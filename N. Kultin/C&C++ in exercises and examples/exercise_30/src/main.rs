/// Записать в виде инструкции присваивания формулу для
/// вычисления сопротивления электрической цепи по известным
/// значениям напряжения и силы тока.

fn main() {
    let (u, i) = (220., 44.);
    let r = u / i;
    println!("r = {r}");
}