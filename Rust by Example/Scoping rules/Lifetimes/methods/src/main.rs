struct Owner(i32);

impl Owner {
    // Annotate lifetimes as in a standalone function.
    #[allow(clippy::needless_lifetimes)]
    fn add_one<'a>(&'a mut self) {
        self.0 += 1;
    }
    #[allow(clippy::needless_lifetimes)]
    fn print<'a>(&'a self) {
        println!("`print`: {}", self.0);
    }
}

fn main() {
    let mut owner = Owner(18);

    owner.add_one();
    owner.print();
}
