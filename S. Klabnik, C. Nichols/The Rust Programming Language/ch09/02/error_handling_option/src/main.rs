fn main() {
    last_char_of_first_line("Hello, world!");
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
