extern crate cargo_kcov_test;

fn main() {
    assert!(cargo_kcov_test::test_this_in_fifth());
}

#[test]
fn fifth_test() {
    main();
}

