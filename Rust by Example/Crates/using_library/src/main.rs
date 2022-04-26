// extern crate creating_library; // May be required for Rust 2015 edition or earlier

fn main() {
    creating_library::public_function();

    // Error! `private_function` is private
    // creating_library::private_function();

    creating_library::indirect_access();
}
