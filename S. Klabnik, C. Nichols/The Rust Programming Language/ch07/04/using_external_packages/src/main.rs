use rand::Rng;
use std::collections::HashMap; // стандартная библиотека (std) является крейтом, который является внешним по отношению к нашему пакету, поставляемая с языком Rust

fn main() {
    let _secret_number = rand::thread_rng().gen_range(1..=100);
    let mut map = HashMap::new();
    map.insert(1, 2);
}
