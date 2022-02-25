use term_table::row::Row;
use term_table::table_cell::TableCell;

fn main() {
    let mut table = term_table::Table::new();
    table.add_row(Row::new(vec![
        TableCell::new("Размер"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("1 байт"),
    ]));
    println!("{}", table.render());

    let _t = true; // занимает в памяти один байт

    let _f: bool = false; // with explicit type annotation
}
