use super::*;

#[test]
fn _0001() {
  num_nan("", false);
}

#[test]
fn _0002() {
  num_finite("12", false, 0, 12, 0);
}

#[test]
fn _0003() {
  num_finite("12e321", false, 0, 12, 321);
}

#[test]
fn _0004() {
  num_finite("938475E-03", false, 0, 938475, -3);
}

#[test]
fn _0005() {
  num_finite("+12", false, 0, 12, 0);
}

#[test]
fn _0006() {
  num_finite("-12", true, 0, 12, 0);
}

#[test]
fn _0007() {
  num_finite("000001", false, 0, 1, 0);
}

#[test]
fn _0008() {
  num_finite("+000001", false, 0, 1, 0);
}

#[test]
fn _0009() {
  num_finite("-000001", true, 0, 1, 0);
}

#[test]
fn _0010() {
  num_finite("0.3", false, 0, 3, -1);
}

#[test]
fn _0011() {
  num_finite("0.3E2", false, 0, 3, 1);
}

#[test]
fn _0012() {
  num_finite("0.3e2", false, 0, 3, 1);
}

#[test]
fn _0013() {
  num_finite("0.3E02", false, 0, 3, 1);
}

#[test]
fn _0014() {
  num_finite("0.3E+02", false, 0, 3, 1);
}

#[test]
fn _0015() {
  num_finite("0.3E-02", false, 0, 3, -3);
}

#[test]
fn _0016() {
  num_finite("0.00003E-02", false, 0, 3, -7);
}

#[test]
fn _0017() {
  num_finite("9999999999999999999999999999999999", false, 0x1ed09bead87c0, 0x378d8e63ffffffff, 0);
}

#[test]
fn _0018() {
  num_inf("inf", false);
}

#[test]
fn _0019() {
  num_inf("+inf", false);
}

#[test]
fn _0020() {
  num_inf("-inf", true);
}

#[test]
fn _0021() {
  num_inf("infinity", false);
}

#[test]
fn _0022() {
  num_inf("+infinity", false);
}

#[test]
fn _0023() {
  num_inf("-infinity", true);
}

#[test]
fn _0024() {
  num_inf("-inFINity", true);
}

#[test]
fn _0025() {
  num_nan("NaN", false);
}

#[test]
fn _0026() {
  num_nan("nan", false);
}

#[test]
fn _0027() {
  num_nan("NAN", false);
}

#[test]
fn _0028() {
  num_nan("+NaN", false);
}

#[test]
fn _0029() {
  num_nan("-NaN", false);
}

#[test]
fn _0030() {
  num_nan("SNaN", true);
}

#[test]
fn _0031() {
  num_nan("+SNaN", true);
}

#[test]
fn _0032() {
  num_nan("-SNaN", true);
}

#[test]
fn _0033() {
  num_nan("-SNAN", true);
}

#[test]
fn _0034() {
  num_nan("qNAN", false);
}

#[test]
fn _0035() {
  num_nan("+A132", false);
}

#[test]
fn _0036() {
  num_nan("1.1P2", false);
}

#[test]
fn _0037() {
  num_nan("1.123P12E", false);
}

#[test]
fn _0038() {
  num_nan("0.1E", false);
}

#[test]
fn _0039() {
  num_nan("0.1EP", false);
}

#[test]
fn _0040() {
  num_nan("0.1E0P", false);
}

#[test]
fn _0041() {
  num_nan("0.1E123P", false);
}

#[test]
fn _0042() {
  num_nan("IE0", false);
}

#[test]
fn _0043() {
  num_nan("InE0", false);
}

#[test]
fn _0044() {
  num_nan("Infinety", false);
}

#[test]
fn _0045() {
  num_nan("Infinizy", false);
}

#[test]
fn _0046() {
  num_nan("Infinitz", false);
}

#[test]
fn _0047() {
  num_nan("Infizity", false);
}

#[test]
fn _0048() {
  num_nan("Infa", false);
}

#[test]
fn _0049() {
  num_nan("nana", false);
}

#[test]
fn _0050() {
  num_nan("nun", false);
}

#[test]
fn _0051() {
  num_nan("snana", false);
}

#[test]
fn _0051_() {
  num_nan("infinitya", false);
}

#[test]
fn _0052() {
  num_nan("sun", false);
}

#[cfg(not(feature = "coverage"))]
#[test]
#[should_panic(expected = "attempt to multiply with overflow")]
fn _0053() {
  num_finite("99999999999999999999999999999999999999999999999999999999999999999", false, 0, 0, 0);
}
