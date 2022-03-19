//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
///
/// # Panics
/// Сценарии, в которых документированная функция может вызывать панику.
/// Вызывающие функцию, которые не хотят, чтобы их программы паниковали,
/// должны убедиться, что они не вызывают функцию в этих ситуациях.
///
/// # Errors
/// Если функция возвращает Result, описание типов ошибок, которые могут
/// произойти и какие условия могут привести к тому, что эти ошибки могут
/// быть возвращены, может быть полезным для вызывающих, так что они могут
/// написать код для обработки различных типов ошибок разными способами.
///
/// # Safety
///  Если функция является unsafe для вызова, должен быть раздел, объясняющий,
/// почему функция небезопасна и охватывающий инварианты, которые функция
/// ожидает от вызывающих сторон.
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}