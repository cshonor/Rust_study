use test_organization_demo as demo;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, demo::add_two(2));
}

