//! Объявить необходимые переменные и записать в виде инструкции
//! присваивания формулу вычисления стоимости покупки, состоящей
//! из нескольких тетрадей, обложек к ним и карандашей.

fn main() {
    let (ctetr, cobl, ckar) = (25., 4., 5.);
    let (ntetr, nkar) = (12, 4);
    let sum = ntetr as f64 * (ctetr + cobl) + nkar as f64 * ckar;
    println!("{}", sum);
}