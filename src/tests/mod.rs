use crate::parse;
use crate::parser::Value;

fn num(input: &str, sign: bool, w1: u64, w0: u64, exp: i32) {
  match parse(input) {
    Value::Number(actual_sign, actual_w1, actual_w0, actual_exponent) => {
      assert_eq!(sign, actual_sign);
      assert_eq!(w1, actual_w1);
      assert_eq!(w0, actual_w0);
      assert_eq!(exp, actual_exponent);
    }
    other => panic!("expected number, actual value is {:?}", other),
  }
}

fn inf(input: &str, sign: bool) {
  match parse(input) {
    Value::Infinity => assert!(!sign),
    Value::NegativeInfinity => assert!(sign),
    other => panic!("expected infinity, actual value is {:?}", other),
  }
}

fn nan(input: &str) {
  match parse(input) {
    Value::NaN => {}
    other => panic!("expected NaN, actual value is {:?}", other),
  }
}

fn snan(input: &str) {
  match parse(input) {
    Value::SNaN => {}
    other => panic!("expected SNaN, actual value is {:?}", other),
  }
}

#[test]
fn _0001() {
  num("12", false, 0, 12, 0);
}

#[test]
fn _0002() {
  num("+12", false, 0, 12, 0);
}

#[test]
fn _0003() {
  num("-12", true, 0, 12, 0);
}

#[test]
fn _0004() {
  num("000001", false, 0, 1, 0);
}

#[test]
fn _0005() {
  num("+000001", false, 0, 1, 0);
}

#[test]
fn _0006() {
  num("-000001", true, 0, 1, 0);
}

#[test]
fn _0007() {
  num("0.3", false, 0, 3, -1);
}

#[test]
fn _0008() {
  num("0.3E2", false, 0, 3, 1);
}

#[test]
fn _0009() {
  num("0.3e2", false, 0, 3, 1);
}

#[test]
fn _0010() {
  num("0.3E02", false, 0, 3, 1);
}

#[test]
fn _0011() {
  num("0.3E+02", false, 0, 3, 1);
}

#[test]
fn _0012() {
  num("0.3E-02", false, 0, 3, -3);
}

#[test]
fn _0013() {
  num("0.00003E-02", false, 0, 3, -7);
}

#[test]
fn _0014() {
  num("9999999999999999999999999999999999", false, 0x1ed09bead87c0, 0x378d8e63ffffffff, 0);
}

#[test]
#[ignore]
fn _0015() {
  num(
    "99999999999999999999999999999999999999999999999999999999999999999",
    false,
    0x1ed09bead87c0,
    0x378d8e63ffffffff,
    0,
  );
}

#[test]
fn _0050() {
  inf("inf", false);
}

#[test]
fn _0051() {
  inf("+inf", false);
}

#[test]
fn _0051_() {
  inf("-inf", true);
}

#[test]
fn _0052() {
  inf("infinity", false);
}

#[test]
fn _0053() {
  inf("+infinity", false);
}

#[test]
fn _0054() {
  inf("-infinity", true);
}

#[test]
fn _0056() {
  inf("-inFINity", true);
}

#[test]
fn _0057() {
  nan("NaN");
}

#[test]
fn _00571() {
  nan("nan");
}

#[test]
fn _0057_2() {
  nan("NAN");
}

#[test]
fn _00572() {
  nan("+NaN");
}

#[test]
fn _00572_1() {
  nan("-NaN");
}

#[test]
fn _0058() {
  snan("SNaN");
}

#[test]
fn _0059() {
  snan("+SNaN");
}

#[test]
fn _0060() {
  snan("-SNaN");
}

#[test]
fn _0061() {
  snan("-SNAN");
}
