const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // Константы

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x); // => The value of x is: 5
    x = 6;
    println!("The value of x is: {}", x); // => The value of x is: 6

    // Затенение
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x); // => The value of x in the inner scope is: 12
    }

    println!("The value of x is: {}", x); // => The value of x is: 6

    let spaces = "   ";
    let spaces = spaces.len();
}
