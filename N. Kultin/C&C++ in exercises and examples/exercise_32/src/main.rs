//! Записать в виде инструкции присваивания формулу вычисления сопротивления
//! электрической цепи, состоящей из двух параллельно соединенных резисторов:
//! r = \frac{r1 * r2}{r1 + r2}.

fn main() {
    let (r1, r2) = (44., 48.);

    let r = r1 * r2 / (r1 + r2);
    println!("r = {r}");
}
