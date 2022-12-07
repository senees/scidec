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

//! # Number parser

use crate::fsm::{recognize, Value};

/// Parsed number.
#[derive(Eq, PartialEq)]
pub enum Number {
  /// Variant representing a finite number.
  Finite(
    /// Flag indicating if the number is signed,
    /// `true` signed (`-` minus), `false` unsigned (`+` plus).
    bool,
    /// Higher 64-bits of the number.
    u64,
    /// Lower 64-bits of the number.
    u64,
    /// Exponent.
    i32,
  ),
  /// Variant representing an infinity.
  Infinite(
    /// Flag indicating if the infinity is signed,
    /// `true` positive infinity, `false` negative infinity.
    bool,
  ),
  /// Variant representing an invalid number.
  NotANumber(
    /// Flag indicating if the value is signed,
    /// `true` positive infinity, `false` negative infinity.
    bool,
    /// Flag indicating if this is a signalling NaN,
    /// `true` signaling, `false` quiet.
    bool,
  ),
}

/// Parses a number properties from text in scientific notation.
///
/// # Examples
///
/// Input text represents a finite number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("1234.5678e-2");
/// match result {
///   Number::Finite(false, 0, 12345678, -6) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents a positive infinity.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("inf");
/// match result {
///   Number::Infinite(false) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents a negative infinity.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("-Infinity");
/// match result {
///   Number::Infinite(true) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents quiet not-a-number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("NaN");
/// match result {
///   Number::NotANumber(false, false) => {}
///   _ => panic!()
/// }
/// ```
///
/// Input text represents signaling not-a-number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("SNaN");
/// match result {
///   Number::NotANumber(false, true) => {}
///   _ => panic!()
/// }
/// ```
pub fn number_from_string(input: &str) -> Number {
  match recognize(input, 34) {
    Value::Finite(sign, value, exponent) => Number::Finite(sign, (value >> 64) as u64, value as u64, exponent),
    Value::Infinity(sign) => Number::Infinite(sign),
    Value::Nan(sign, signaling) => Number::NotANumber(sign, signaling),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    assert!((Number::Finite(false, 0, 0, 0) == Number::Finite(false, 0, 0, 0)));
    assert!((Number::Finite(false, 0, 0, 0) != Number::Infinite(false)));
    assert!((Number::Infinite(true) != Number::Infinite(false)));
    assert!((Number::Infinite(true) == Number::Infinite(true)));
    assert!((Number::NotANumber(true, true) != Number::NotANumber(false, false)));
    assert!((Number::NotANumber(false, false) == Number::NotANumber(false, false)));
    Number::Infinite(false).assert_receiver_is_total_eq();
  }
}
