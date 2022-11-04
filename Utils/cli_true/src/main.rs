/// Возвращает истинное значение.
fn main() {
    std::process::exit(0);
}

#[cfg(test)]
mod tests {
    #[test]
    fn true_ok() {
        use assert_cmd::Command;

        let mut cmd = Command::cargo_bin("true").unwrap();
        cmd.assert().success();
    }
}
