/// Возвращает истинное значение.
fn main() {
    std::process::exit(1);
}

#[cfg(test)]
mod tests {
    #[test]
    fn true_ok() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("false").unwrap();
        cmd.assert().failure();
    }
}
