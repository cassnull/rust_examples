#[macro_export]
macro_rules! vec {
    // шаблоны макроса сопоставляются со структурами кода
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    let _v: Vec<u32> = vec![1, 2, 3];
}
