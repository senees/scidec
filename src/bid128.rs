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

//! # Number parser for 128-bit floating-point decimals.

/// 128-bit decimal in binary format.
#[derive(Eq, PartialEq)]
pub struct Bid128 {
  w: [u64; 2],
}

const BID128_NAN: Bid128 = Bid128 { w: [0, 2] };

/// Parses a 128-bit floating-point decimal from text in scientific notation.
///
/// Minimum parsed number is:
/// ```text
/// ?
/// ```
///
/// Maximum parsed number is:
/// ```text
/// ?
/// ```
///
/// # Examples
///
/// Input text represents a finite number.
/// ```
/// ```
///
/// Input text represents a positive infinity.
/// ```
/// ```
///
/// Input text represents a negative infinity.
/// ```
/// ```
///
/// Input text represents quiet not-a-number.
/// ```
/// ```
///
/// Input text represents signaling not-a-number.
/// ```
/// ```
pub fn bid128_from_string(_input: &str) -> Bid128 {
  BID128_NAN
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    Bid128 { w: [0, 2] }.assert_receiver_is_total_eq();
  }
}
