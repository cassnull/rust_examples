fn foo(_x: i32) {
    // code goes here
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    foo(292);

    let point = (3, 5);
    print_coordinates(&point);
}
