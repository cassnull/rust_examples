#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

#[derive(Debug)]
// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(
    Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right:
            Point {
                x: bottom_right_x,
                y: bottom_right_y,
            },
    }: &Rectangle,
) -> f32 {
    (bottom_right_x - top_left_x) * (bottom_right_y - top_left_y)
}

fn square(
    Point {
        x: top_left_x,
        y: top_left_y,
    }: Point,
    size: f32,
) -> Rectangle {
    Rectangle {
        top_left: Point {
            x: top_left_x,
            y: top_left_y,
        },
        bottom_right: Point {
            x: top_left_x + size,
            y: top_left_y + size,
        },
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point {
        x: left_edge,
        y: top_edge,
    } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point {
            x: left_edge,
            y: top_edge,
        },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: 2., y: 3. },
        bottom_right: Point { x: 10., y: 5. },
    };
    println!("the rectangle area is {}", rect_area(&rectangle));
    let rectangle = square(point, 4.0);
    println!(
        "the rectangle {:?} area is {}",
        rectangle,
        rect_area(&rectangle)
    );
}
