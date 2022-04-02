use std::fmt;
use std::ops::{Deref, DerefMut};

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

impl Deref for Wrapper {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Wrapper {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<Wrapper> for Vec<String> {
    fn from(wrapper: Wrapper) -> Self {
        wrapper.0
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
    println!("First element: {}", w[0]);

    let v: Vec<String> = w.into();
    println!("{:?}", v);
}
