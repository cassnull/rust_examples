//! Записать в виде инструкции присваивания формулу пересчета
//! сопротивления электрической цепи из омов в килоомы.

fn main() {
    let r_om = 44.;
    let r_kom = r_om / 1000.;
    println!("r = {} kOm", r_kom);
}
