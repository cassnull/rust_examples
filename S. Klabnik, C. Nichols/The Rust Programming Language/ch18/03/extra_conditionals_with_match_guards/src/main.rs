fn main() {
    let num = Some(16);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = Some(25);
    let y = 292;

    match x {
        Some(16) => println!("Got 16"),
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {}", x, y);

    let x = 16;
    let y = false;

    match x {
        16 | 25 | 292 if y => println!("yes"),
        _ => println!("no"),
    }
}
