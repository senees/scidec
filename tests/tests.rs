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

//! # Smoke tests
//!
//! Functions exposed by this library are intensively tested using unit tests.
//! Smoke tests check only the correctness of the library interface.

use scidec::{bid128_from_string, number_from_string, Number};

#[test]
fn test_number_from_string() {
  assert!((Number::Finite(false, 0, 3, -7) == number_from_string("0.00003E-02")));
}

#[test]
fn test_bid128_from_string() {
  let (actual, status) = bid128_from_string("0.00003E-02");
  assert_eq!(0x3032000000000000, actual.w[1]);
  assert_eq!(0x0000000000000003, actual.w[0]);
  assert_eq!(0x0, status);
}
