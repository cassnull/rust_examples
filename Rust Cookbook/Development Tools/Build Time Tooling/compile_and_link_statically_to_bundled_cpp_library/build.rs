fn main() {
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .flag("-std=c++20")
        .shared_flag(true)
        .warnings(false)
        .compile("foo");
}
