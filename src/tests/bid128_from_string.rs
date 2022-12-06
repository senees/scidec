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
  bid128_nan(IN[0], false);
}

#[test]
fn _0002() {
  bid128_fin(IN[1], 0x3040000000000000, 0x000000000000000c);
}

#[test]
fn _0003() {
  bid128_fin(IN[2], 0x303c000000000000, 0x000000000000000c);
}

#[test]
#[ignore]
fn _0004() {
  bid128_fin(IN[3], 0x303e000000000000, 0x0000000000000000);
}

#[test]
#[ignore]
fn _0005() {
  bid128_fin(IN[4], 0x303e000000000000, 0x0000000000000000);
}

#[test]
#[ignore]
fn _0006() {
  bid128_fin(IN[5], 0x3040000000000000, 0x0000000000000000);
}

#[test]
#[ignore]
fn _0007() {
  bid128_fin(IN[6], 0x303e000000000000, 0x000000000000000a);
}

/*
#[test]
fn _0008() {
  bid128_fin(IN[7], _);
}

#[test]
fn _0009() {
  bid128_fin(IN[8], _);
}

#[test]
fn _0010() {
  bid128_fin(IN[9], _);
}

#[test]
fn _0011() {
  bid128_fin(IN[10], _);
}

#[test]
fn _0012() {
  bid128_fin(IN[11], _);
}

#[test]
fn _0013() {
  bid128_fin(IN[12], _);
}

#[test]
fn _0014() {
  bid128_fin(IN[13], _);
}

#[test]
fn _0015() {
  bid128_fin(IN[14], _);
}

#[test]
fn _0016() {
  bid128_fin(IN[15], _);
}

#[test]
fn _0017() {
  bid128_fin(IN[16], _);
}

#[test]
fn _0018() {
  bid128_fin(IN[17], _);
}

#[test]
fn _0019() {
  bid128_fin(IN[18], _);
}

#[test]
fn _0020() {
  bid128_fin(IN[19], _);
}

#[test]
fn _0021() {
  bid128_fin(IN[20], _);
}

#[test]
fn _0022() {
  bid128_fin(IN[21], _);
}

#[test]
fn _0023() {
  bid128_fin(IN[22], _);
}

#[test]
fn _0024() {
  bid128_fin(IN[23], _);
}

#[test]
fn _0025() {
  bid128_fin(IN[24], _);
}

#[test]
fn _0026() {
  bid128_fin(IN[25], _);
}

#[test]
fn _0027() {
  bid128_fin(IN[26], _);
}

#[test]
fn _0028() {
  bid128_fin(IN[27], _);
}

#[test]
fn _0029() {
  bid128_fin(IN[28], _);
}

#[test]
fn _0030() {
  bid128_fin(IN[29], _);
}

#[test]
fn _0031() {
  bid128_fin(IN[30], _);
}

#[test]
fn _0032() {
  num_inf(IN[31], _);
}

#[test]
fn _0033() {
  num_inf(IN[32], _);
}

#[test]
fn _0034() {
  num_inf(IN[33], _);
}

#[test]
fn _0035() {
  num_inf(IN[34], _);
}

#[test]
fn _0036() {
  num_inf(IN[35], _);
}

#[test]
fn _0037() {
  num_inf(IN[36], _);
}

#[test]
fn _0038() {
  num_inf(IN[37], _);
}

#[test]
fn _0039() {
  num_nan(IN[38], _);
}

#[test]
fn _0040() {
  num_nan(IN[39], _);
}

#[test]
fn _0041() {
  num_nan(IN[40], _);
}

#[test]
fn _0042() {
  num_nan(IN[41], _);
}

#[test]
fn _0043() {
  num_nan(IN[42], _);
}

#[test]
fn _0044() {
  num_nan(IN[43], _);
}

#[test]
fn _0045() {
  num_nan(IN[44], _);
}

#[test]
fn _0046() {
  num_nan(IN[45], _);
}

#[test]
fn _0047() {
  num_nan(IN[46], _);
}

#[test]
fn _0048() {
  num_nan(IN[47], _);
}

#[test]
fn _0049() {
  num_nan(IN[48], _);
}

#[test]
fn _0050() {
  num_nan(IN[49], _);
}

#[test]
fn _0051() {
  num_nan(IN[50], _);
}

#[test]
fn _0052() {
  num_nan(IN[51], _);
}

#[test]
fn _0053() {
  num_nan(IN[52], _);
}

#[test]
fn _0054() {
  num_nan(IN[53], _);
}

#[test]
fn _0055() {
  num_nan(IN[54], _);
}

#[test]
fn _0056() {
  num_nan(IN[55], _);
}

#[test]
fn _0057() {
  num_nan(IN[56], _);
}

#[test]
fn _0058() {
  num_nan(IN[57], _);
}

#[test]
fn _0059() {
  num_nan(IN[58], _);
}

#[test]
fn _0060() {
  num_nan(IN[59], _);
}

#[test]
fn _0061() {
  num_nan(IN[60], _);
}

#[test]
fn _0062() {
  num_nan(IN[61], _);
}

#[test]
fn _0063() {
  num_nan(IN[62], _);
}

#[test]
fn _0064() {
  num_nan(IN[63], _);
}

#[test]
fn _0065() {
  num_nan(IN[64], _);
}

#[test]
fn _0066() {
  num_nan(IN[65], _);
}

#[test]
fn _0067() {
  num_nan(IN[66], _);
}

#[test]
fn _0068() {
  num_nan(IN[67], _);
}

#[test]
fn _0069() {
  num_nan(IN[68], _);
}

#[test]
fn _0070() {
  num_nan(IN[69], _);
}

#[test]
fn _0071() {
  bid128_fin(IN[70], _);
}

#[test]
fn _0072() {
  bid128_fin(IN[71], _);
}

#[test]
fn _0073() {
  bid128_fin(IN[72], _);
}

#[test]
fn _0074() {
  bid128_fin(IN[73], _);
}

#[test]
fn _0075() {
  bid128_fin(IN[74], _);
}

#[test]
fn _0076() {
  bid128_fin(IN[75], _);
}

#[test]
fn _0077() {
  bid128_fin(IN[76], _);
}

#[test]
fn _0078() {
  bid128_fin(IN[77], _);
}
*/
