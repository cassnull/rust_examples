fn main() {
    let tup = (500, 6.4, 1);

    let (_x, y, _z) = tup; // деструктуризация

    println!("The value of y is: {}", y);

    let _five_hundred = tup.0;

    let _six_point_four = tup.1;

    let _one = tup.2;
}
