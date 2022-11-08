use writing_automated_tests;
mod common;


#[test]
fn it_add_two() {
    common::setup();
    assert_eq!(4, writing_automated_tests::madonna::add_two(2));
}

