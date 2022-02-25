use term_table::row::Row;
use term_table::table_cell::TableCell;

fn main() {
    let mut table = term_table::Table::new();
    table.add_row(Row::new(vec![
        TableCell::new("Размер"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("4 байта"),
    ]));
    println!("{}", table.render());

    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';
    assert_eq!('\u{00e9}', 'é');
}
