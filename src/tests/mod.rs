/*
 * MIT License
 *
 * Copyright (c) 2022 Dariusz Depta
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Utility functions for unit tests.

use crate::bid128_from_string;

const BID128_INPUT: &str = include_str!("test_cases.in");

#[test]
fn test_input_cases() {
  for (i, mut line) in BID128_INPUT.lines().enumerate() {
    line = line.trim();
    if !line.is_empty() && !line.starts_with('#') {
      let mut columns = line.split(' ');
      let rounding = columns.next().unwrap().parse::<u8>().unwrap();
      let input = columns.next().unwrap().trim_matches('"');
      let mut bid = columns.next().unwrap().trim_matches('[').trim_matches(']').split(',');
      let expected_w1 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
      let expected_w0 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
      let expected_status = u32::from_str_radix(columns.next().unwrap(), 16).unwrap();
      let (actual, actual_status) = bid128_from_string(input);
      let actual_w1 = actual.w[1];
      let actual_w0 = actual.w[0];
      let line_no = i + 1;
      assert_eq!(
        rounding, 0,
        "[{line_no}] rounding:\nexpected: 0\n  actual: {rounding}\n",
      );
      assert_eq!(
        expected_w1, actual_w1,
        "[{line_no}] w1:\nexpected: {expected_w1:016x}\n  actual: {actual_w1:016x}\n",
      );
      assert_eq!(
        expected_w0, actual_w0,
        "[{line_no}] w0:\nexpected: {expected_w0:016x}\n  actual: {actual_w0:016x}\n",
      );
      assert_eq!(
        expected_status, actual_status,
        "[{line_no}] status:\nexpected: {expected_status:02x}\n  actual: {actual_status:02x}\n",
      );
    }
  }
}

#[test]
fn test_check() {
  let s = "-1000000000000000000000000000000000e-6211";
  let (actual, actual_status) = bid128_from_string(s);
  assert_eq!(0x8000000000000000, actual.w[1]);
  assert_eq!(0x0000000000000000, actual.w[0]);
  assert_eq!(0x30, actual_status);
}
