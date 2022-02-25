use term_table::row::Row;
use term_table::table_cell::TableCell;

fn main() {
    let mut table = term_table::Table::new();
    table.add_row(Row::new(vec![
        TableCell::new("Размер"),
        TableCell::new("Знаковый"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("32 бит"),
        TableCell::new("f32"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("64 бит"),
        TableCell::new("f64"),
    ]));
    println!("{}", table.render());

    let _x = 2.0; // f64

    let _y: f32 = 3.0; // f32
}
