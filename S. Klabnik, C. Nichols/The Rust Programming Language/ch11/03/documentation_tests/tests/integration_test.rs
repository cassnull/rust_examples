use documentation_tests;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, documentation_tests::add_two(2));
}
