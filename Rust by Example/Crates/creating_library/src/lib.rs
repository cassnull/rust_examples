pub fn public_function() {
    println!("called creating_library's `public_function()`");
}

fn private_function() {
    println!("called creating_library's `private_function()`");
}

pub fn indirect_access() {
    print!("called creating_library's `indirect_access()`, that\n> ");

    private_function();
}
