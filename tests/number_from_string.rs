use scidec::{number_from_string, Number};

#[test]
fn test_number_from_string() {
  assert_eq!(Number::Finite(false, 0, 3, -7), number_from_string("0.00003E-02"));
}
