fn main() {
    let favorite_color: Option<&str> = Some("orange");

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    }

    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
