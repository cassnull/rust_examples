use term_table::row::Row;
use term_table::table_cell::TableCell;

fn main() {
    let mut table = term_table::Table::new();
    table.add_row(Row::new(vec![
        TableCell::new("Размер"),
        TableCell::new("Знаковый"),
        TableCell::new("Беззнаковый"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("8-bit"),
        TableCell::new("i8"),
        TableCell::new("u8"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("16 бит"),
        TableCell::new("i16"),
        TableCell::new("u16"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("32 бита"),
        TableCell::new("i32"),
        TableCell::new("u32"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("64 бита"),
        TableCell::new("i64"),
        TableCell::new("u64"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("128 бит"),
        TableCell::new("i128"),
        TableCell::new("u128"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("arch"),
        TableCell::new("isize"),
        TableCell::new("usize"),
    ]));
    println!("{}", table.render());

    let mut table = term_table::Table::new();
    table.add_row(Row::new(vec![
        TableCell::new("Числовые литералы"),
        TableCell::new("Пример"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Десятичный"),
        TableCell::new("98_222"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Шестнадцатеричный"),
        TableCell::new("0xff"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Восьмеричный"),
        TableCell::new("0o77"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Бинарный"),
        TableCell::new("0b1111_0000"),
    ]));
    table.add_row(Row::new(vec![
        TableCell::new("Байтовый (только u8)"),
        TableCell::new("b'A'"),
    ]));
    println!("{}", table.render());

    // обработать возможность переполнения
    // wrapping_*
    assert_eq!(16u8.wrapping_add(9), 25);
    assert_eq!(1u8.wrapping_add(u8::MAX), 0);
    // checked_*
    assert_eq!(16u8.checked_add(9), Some(25));
    assert_eq!(u8::MAX.checked_add(1), None);
    // overflowing_*
    assert_eq!(16u8.overflowing_add(9), (25, false));
    assert_eq!(u8::MAX.overflowing_add(1), (0, true));
    // saturating_*
    assert_eq!(16u8.saturating_add(9), 25);
    assert_eq!(u8::MAX.saturating_add(1), u8::MAX);
}
