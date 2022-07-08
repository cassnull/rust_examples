#[doc(inline)]
pub use bar::Bar;

#[doc(no_inline)]
pub use foo::Foo;

// Example from the futures-rs library
#[doc(hidden)]
pub use baz::Baz;

/// bar docs
mod bar {
    /// the docs for Bar
    pub struct Bar;
}

/// foo docs
mod foo {
    /// the docs for Foo
    pub struct Foo;
}

/// baz docs
mod baz {
    /// the docs for Baz
    pub struct Baz;
}
