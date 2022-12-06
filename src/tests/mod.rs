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

use crate::{bid128_from_string, number_from_string, Number};

mod bid128_from_string;
mod inputs;
mod number_from_string;

use crate::bid128::*;
pub use inputs::IN;

const BID128_INPUT: &str = include_str!("bid128.in");

#[test]
fn a() {
  for (i, line) in BID128_INPUT.lines().enumerate() {
    let mut columns = line.trim().split(" ");
    let rounding = columns.next().unwrap().parse::<u8>().unwrap();
    let input = columns.next().unwrap().trim_matches('"');
    let mut bid = columns.next().unwrap().trim_matches('[').trim_matches(']').split(',');
    let ew1 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
    let ew0 = u64::from_str_radix(bid.next().unwrap(), 16).unwrap();
    let status = u64::from_str_radix(columns.next().unwrap(), 16).unwrap();
    let actual = bid128_from_string(input);
    let aw1 = actual.w[1];
    let aw0 = actual.w[0];
    assert_eq!(
      rounding, 0,
      "[{}] rounding:\nexpected: {}\n  actual: {}\n",
      i, 0, rounding
    );
    assert_eq!(
      ew1, aw1,
      "[{}] w1:\nexpected: {:016x}\n  actual: {:016x}\n",
      i, ew1, aw1
    );
    assert_eq!(
      ew0, aw0,
      "[{}] w0:\nexpected: {:016x}\n  actual: {:016x}\n",
      i, ew0, aw0
    );
    assert_eq!(
      status, 0,
      "[{}] status:\nexpected: {:02x}\n  actual: {:02x}\n",
      i, 0, status
    );
  }
}

fn num_fin(input: &str, sign: bool, w1: u64, w0: u64, exp: i32) {
  match number_from_string(input) {
    Number::Finite(actual_sign, actual_w1, actual_w0, actual_exp) => {
      assert_eq!(sign, actual_sign);
      assert_eq!(w1, actual_w1, "w1\nexpected: {:x}\n  actual: {:x}\n", w1, actual_w1);
      assert_eq!(w0, actual_w0, "w0\nexpected: {:x}\n  actual: {:x}\n", w0, actual_w0);
      assert_eq!(exp, actual_exp);
    }
    Number::Infinite(false) => panic!("expected number, actual value is +Inf"),
    Number::Infinite(true) => panic!("expected number, actual value is -Inf"),
    Number::NotANumber(false) => panic!("expected number, actual value is NaN"),
    Number::NotANumber(true) => panic!("expected number, actual value is SNaN"),
  }
}

fn num_inf(input: &str, sign: bool) {
  match number_from_string(input) {
    Number::Infinite(actual_sign) => assert_eq!(sign, actual_sign),
    Number::NotANumber(false) => panic!("expected number, actual value is NaN"),
    Number::NotANumber(true) => panic!("expected number, actual value is SNaN"),
    Number::Finite(_, _, _, _) => panic!("expected infinity, actual value is finite number"),
  }
}

fn num_nan(input: &str, signaling: bool) {
  match number_from_string(input) {
    Number::NotANumber(actual_signaling) => assert_eq!(signaling, actual_signaling),
    Number::Infinite(false) => panic!("expected number, actual value is +Inf"),
    Number::Infinite(true) => panic!("expected number, actual value is -Inf"),
    Number::Finite(_, _, _, _) => panic!("expected infinity, actual value is finite number"),
  }
}

fn bid128_fin(input: &str, w1: u64, w0: u64) {
  let actual = bid128_from_string(input);
  assert_eq!(w1, actual.w[1]);
  assert_eq!(w0, actual.w[0]);
}

fn bid128_nan(input: &str, signaling: bool) {
  let actual = bid128_from_string(input);
  if signaling {
    assert!((BID128_SNAN == actual));
  } else {
    assert!((BID128_NAN == actual));
  }
}
