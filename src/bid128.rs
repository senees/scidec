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

use crate::common::{mul_add, State};

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
pub fn bid128_from_string(input: &str) -> Bid128 {
  if input.is_empty() {
    return BID128_NAN;
  }
  let mut state = State::BeginNumber;
  let mut sign = false;
  let mut exponent = 0_i32;
  let mut exponent_base = 0_i32;
  let mut exponent_sign = 1_i32;
  let mut value = 0_u128;
  let mut infinity = false;
  let mut nan = false;
  let mut signaling = false;
  let last = input.len() - 1;
  for (position, ch) in input.chars().enumerate() {
    match state {
      State::BeginNumber => match ch {
        '-' => {
          sign = true;
          state = State::LeadingZeros;
        }
        '+' | '0' => state = State::LeadingZeros,
        '1'..='9' => {
          mul_add!(value, ch, u128);
          state = State::DigitsBefore;
        }
        '.' if position < last => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return BID128_NAN,
      },
      State::LeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          mul_add!(value, ch, u128);
          state = State::DigitsBefore;
        }
        '.' if position == last => {
          exponent -= 1;
          mul_add!(value, b'0', u128);
        }
        '.' => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return BID128_NAN,
      },
      State::DigitsBefore => match ch {
        '0'..='9' => mul_add!(value, ch, u128),
        '.' if position == last => {
          exponent -= 1;
          mul_add!(value, b'0', u128);
        }
        '.' => state = State::DigitsAfter,
        'E' | 'e' => state = State::ExponentSign,
        _ => return BID128_NAN,
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          exponent -= 1;
          mul_add!(value, ch, u128);
        }
        'E' | 'e' if position < last => state = State::ExponentSign,
        _ => return BID128_NAN,
      },
      State::ExponentSign => match ch {
        '+' | '0' if position < last => state = State::ExponentLeadingZeros,
        '-' if position < last => {
          exponent_sign = -1_i32;
          state = State::ExponentLeadingZeros;
        }
        '1'..='9' => {
          mul_add!(exponent_base, ch, i32);
          state = State::ExponentDigits;
        }
        _ => return BID128_NAN,
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          mul_add!(exponent_base, ch, i32);
          state = State::ExponentDigits;
        }
        _ => return BID128_NAN,
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          mul_add!(exponent_base, ch, i32);
        }
        _ => return BID128_NAN,
      },
      State::Inf2n => match ch {
        'n' | 'N' => state = State::Inf3f,
        _ => return BID128_NAN,
      },
      State::Inf3f => match ch {
        'f' | 'F' if position == last => infinity = true,
        'f' | 'F' => state = State::Inf4i,
        _ => return BID128_NAN,
      },
      State::Inf4i => match ch {
        'i' | 'I' => state = State::Inf5n,
        _ => return BID128_NAN,
      },
      State::Inf5n => match ch {
        'n' | 'N' => state = State::Inf6i,
        _ => return BID128_NAN,
      },
      State::Inf6i => match ch {
        'i' | 'I' => state = State::Inf7t,
        _ => return BID128_NAN,
      },
      State::Inf7t => match ch {
        't' | 'T' => state = State::Inf8y,
        _ => return BID128_NAN,
      },
      State::Inf8y => match ch {
        'y' | 'Y' if position == last => infinity = true,
        _ => return BID128_NAN,
      },
      State::Nan1n => match ch {
        'n' | 'N' => state = State::Nan2a,
        _ => return BID128_NAN,
      },
      State::Nan2a => match ch {
        'a' | 'A' => state = State::Nan3n,
        _ => return BID128_NAN,
      },
      State::Nan3n => match ch {
        'n' | 'N' if position == last => nan = true,
        _ => return BID128_NAN,
      },
    }
  }
  if infinity {
    return BID128_NAN;
  }
  if nan {
    return BID128_NAN;
  }
  // Bid128::Fin(
  //   sign,
  //   (value >> 64) as u64,
  //   value as u64,
  //   exponent + exponent_sign * exponent_base,
  // )
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
