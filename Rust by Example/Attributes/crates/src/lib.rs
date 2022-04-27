// This crate is a library
#![crate_type = "lib"]
// The library is named "crates"
#![crate_name = "crates"]

pub fn public_function() {
    println!("called crates's `public_function()`");
}

fn private_function() {
    println!("called crates's `private_function()`");
}

pub fn indirect_access() {
    print!("called crates's `indirect_access()`, that\n> ");

    private_function();
}
