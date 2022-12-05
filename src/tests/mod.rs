use crate::{number_from_string, Number};

mod number_from_string;

fn num_finite(input: &str, sign: bool, w1: u64, w0: u64, exp: i32) {
  match number_from_string(input) {
    Number::Finite(actual_sign, actual_w1, actual_w0, actual_exponent) => {
      assert_eq!(sign, actual_sign);
      assert_eq!(w1, actual_w1);
      assert_eq!(w0, actual_w0);
      assert_eq!(exp, actual_exponent);
    }
    other => panic!("expected number, actual value is {:?}", other),
  }
}

fn num_inf(input: &str, sign: bool) {
  match number_from_string(input) {
    Number::Inf(actual_sign) => assert_eq!(sign, actual_sign),
    other => panic!("expected infinity, actual value is {:?}", other),
  }
}

fn num_nan(input: &str, signaling: bool) {
  match number_from_string(input) {
    Number::NaN(actual_signaling) => assert_eq!(signaling, actual_signaling),
    other => panic!("expected NaN, actual value is {:?}", other),
  }
}
