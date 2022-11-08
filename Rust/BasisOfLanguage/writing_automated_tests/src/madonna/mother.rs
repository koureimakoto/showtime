use crate::madonna::internal_adder;

#[test]
fn internal() {
    assert_eq!(4, internal_adder(2, 2));
}
