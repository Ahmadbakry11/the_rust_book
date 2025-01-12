use test_organization::{adds_two, add};
mod common;

// to show output in common::setup just run
// cargo test --test integtaion_test --  --show-output

#[test]
fn step_two() {
    common::setup();
    let result = adds_two(7);
    assert_eq!(result, 9);
}

#[test]
fn adder() {
    common::setup();
    let result = add(2, 7);
    assert_eq!(result, 9);
}