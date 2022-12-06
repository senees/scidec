/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the IN[]), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED IN[], WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Unit tests for parsing numbers.

use super::*;

#[test]
fn _0001() {
  num_nan("", false);
}

#[test]
fn _0002() {
  num_fin("12", false, 0, 12, 0);
}

#[test]
fn _0003() {
  num_fin(".12", false, 0, 12, -2);
}

#[test]
fn _0004() {
  num_fin("000.0", false, 0, 0, -1);
}

#[test]
fn _0005() {
  num_fin("0.0", false, 0, 0, -1);
}

#[test]
fn _0006() {
  num_fin("0.", false, 0, 0, 0);
}

#[test]
fn _0007() {
  num_fin("1.0", false, 0, 10, -1);
}

#[test]
fn _0008() {
  num_fin("1.", false, 0, 1, 0);
}

#[test]
fn _0009() {
  num_fin("1.0e2", false, 0, 10, 1);
}

#[test]
fn _0010() {
  num_fin("1.00e2", false, 0, 100, 0);
}

#[test]
fn _0011() {
  num_fin("1.00e354", false, 0, 100, 352);
}

#[test]
#[ignore]
fn _0012() {
  num_fin("1.000000000000000000000000000000000000000000e2", false, 0, 0, -31);
}

#[test]
fn _0013() {
  num_fin(IN[12], false, 0, 100, 0);
}

#[test]
fn _0014() {
  num_fin(IN[13], false, 0, 1, 5);
}

#[test]
fn _0015() {
  num_fin(IN[14], false, 0, 1001, -67);
}

#[test]
fn _0016() {
  num_fin(IN[15], false, 0, 12345678, -6);
}

#[test]
fn _0017() {
  num_fin(IN[16], false, 0, 12, 321);
}

#[test]
fn _0018() {
  num_fin(IN[17], false, 0, 938475, -3);
}

#[test]
fn _0019() {
  num_fin(IN[18], false, 0, 12, 0);
}

#[test]
fn _0020() {
  num_fin(IN[19], true, 0, 12, 0);
}

#[test]
fn _0021() {
  num_fin(IN[20], false, 0, 1, 0);
}

#[test]
fn _0022() {
  num_fin(IN[21], false, 0, 1, 0);
}

#[test]
fn _0023() {
  num_fin(IN[22], true, 0, 1, 0);
}

#[test]
fn _0024() {
  num_fin(IN[23], false, 0, 3, -1);
}

#[test]
fn _0025() {
  num_fin(IN[24], false, 0, 3, 1);
}

#[test]
fn _0026() {
  num_fin(IN[25], false, 0, 3, 1);
}

#[test]
fn _0027() {
  num_fin(IN[26], false, 0, 3, 1);
}

#[test]
fn _0028() {
  num_fin(IN[27], false, 0, 3, 1);
}

#[test]
fn _0029() {
  num_fin(IN[28], false, 0, 3, -3);
}

#[test]
fn _0030() {
  num_fin(IN[29], false, 0, 3, -7);
}

#[test]
fn _0031() {
  num_fin(IN[30], false, 0x1ed09bead87c0, 0x378d8e63ffffffff, 0);
}

#[test]
fn _0032() {
  num_inf(IN[31], false);
}

#[test]
fn _0033() {
  num_inf(IN[32], false);
}

#[test]
fn _0034() {
  num_inf(IN[33], true);
}

#[test]
fn _0035() {
  num_inf(IN[34], false);
}

#[test]
fn _0036() {
  num_inf(IN[35], false);
}

#[test]
fn _0037() {
  num_inf(IN[36], true);
}

#[test]
fn _0038() {
  num_inf(IN[37], true);
}

#[test]
fn _0039() {
  num_nan(IN[38], false);
}

#[test]
fn _0040() {
  num_nan(IN[39], false);
}

#[test]
fn _0041() {
  num_nan(IN[40], false);
}

#[test]
fn _0042() {
  num_nan(IN[41], false);
}

#[test]
fn _0043() {
  num_nan(IN[42], false);
}

#[test]
fn _0044() {
  num_nan(IN[43], true);
}

#[test]
fn _0045() {
  num_nan(IN[44], true);
}

#[test]
fn _0046() {
  num_nan(IN[45], true);
}

#[test]
fn _0047() {
  num_nan(IN[46], true);
}

#[test]
fn _0048() {
  num_nan(IN[47], false);
}

#[test]
fn _0049() {
  num_nan(IN[48], false);
}

#[test]
fn _0050() {
  num_nan(IN[49], false);
}

#[test]
fn _0051() {
  num_nan(IN[50], false);
}

#[test]
fn _0052() {
  num_nan(IN[51], false);
}

#[test]
fn _0053() {
  num_nan(IN[52], false);
}

#[test]
fn _0054() {
  num_nan(IN[53], false);
}

#[test]
fn _0055() {
  num_nan(IN[54], false);
}

#[test]
fn _0056() {
  num_nan(IN[55], false);
}

#[test]
fn _0057() {
  num_nan(IN[56], false);
}

#[test]
fn _0058() {
  num_nan(IN[57], false);
}

#[test]
fn _0059() {
  num_nan(IN[58], false);
}

#[test]
fn _0060() {
  num_nan(IN[59], false);
}

#[test]
fn _0061() {
  num_nan(IN[60], false);
}

#[test]
fn _0062() {
  num_nan(IN[61], false);
}

#[test]
fn _0063() {
  num_nan(IN[62], false);
}

#[test]
fn _0064() {
  num_nan(IN[63], false);
}

#[test]
fn _0065() {
  num_nan(IN[64], false);
}

#[test]
fn _0066() {
  num_nan(IN[65], false);
}

#[test]
fn _0067() {
  num_nan(IN[66], false);
}

#[test]
fn _0068() {
  num_nan(IN[67], false);
}

#[test]
fn _0069() {
  num_nan(IN[68], false);
}

#[test]
fn _0070() {
  num_nan(IN[69], false);
}

#[test]
fn _0071() {
  num_fin(IN[70], false, 0x68db8bac710cb, 0x295e9e1b089a0275, 0);
}

#[test]
fn _0072() {
  num_fin(IN[71], true, 0x68db8bac710cb, 0x295e9e1b089a0275, 0);
}

#[test]
fn _0073() {
  num_fin(IN[72], false, 0x68db8bac710cb, 0x295e9e1b089a0275, 0x7FFFFFFF);
}

#[test]
fn _0074() {
  num_fin(IN[73], true, 0x68db8bac710cb, 0x295e9e1b089a0275, -0x7FFFFFFF);
}

#[test]
fn _0075() {
  num_fin(IN[74], false, 0x13426172c74d82, 0x2b878fe7ffffffff, 0);
}

#[test]
fn _0076() {
  num_fin(IN[75], true, 0x13426172c74d82, 0x2b878fe7ffffffff, 0);
}

#[test]
fn _0077() {
  num_fin(IN[76], false, 0, 1, 2147483647);
}

#[test]
fn _0078() {
  num_fin(IN[77], true, 0, 1, -2147483647);
}
