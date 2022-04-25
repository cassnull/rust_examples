mod deeply {
    pub mod nested {
        pub fn my_first_function() {
            println!("called `deeply::nested::my_first_function()`");
        }

        #[allow(dead_code)]
        pub fn my_second_function() {
            println!("called `deeply::nested::my_second_function()`");
        }

        #[allow(dead_code)]
        pub fn and_a_trait_type() {
            println!("called `deeply::nested::and_a_trait_type()`");
        }

        pub fn function() {
            println!("called `deeply::nested::function()`");
        }
    }
}

#[allow(unused_imports)]
use crate::deeply::nested::{and_a_trait_type, my_first_function, my_second_function};

// Bind the `deeply::nested::function` path to `other_function`.
use deeply::nested::function as other_function;

fn function() {
    println!("called `function()`");
}

fn main() {
    my_first_function();

    // Easier access to `deeply::nested::function`
    other_function();

    println!("Entering block");
    {
        // This is equivalent to `use deeply::nested::function as function`.
        // This `function()` will shadow the outer one.
        use crate::deeply::nested::function;

        // `use` bindings have a local scope. In this case, the
        // shadowing of `function()` is only in this block.
        function();

        println!("Leaving block");
    }

    function();
}
