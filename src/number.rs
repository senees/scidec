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

/// Parsed number.
#[derive(Eq, PartialEq)]
pub enum Number {
  /// Variant representing a finite number.
  Fin(
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
  Inf(
    /// Flag indicating if the infinity is signed,
    /// `true` positive infinity, `false` negative infinity.
    bool,
  ),
  /// Variant representing an invalid number.
  NaN(
    /// Flag indicating if this is a signalling NaN,
    /// `true` signaling, `false` quiet.
    bool,
  ),
}

/// States of the finite state machine used for parsing the input text.
enum State {
  BeginNumber,
  LeadingZeros,
  DigitsBefore,
  DigitsAfter,
  ExponentSign,
  ExponentLeadingZeros,
  ExponentDigits,
  Inf2n,
  Inf3f,
  Inf4i,
  Inf5n,
  Inf6i,
  Inf7t,
  Inf8y,
  Nan1n,
  Nan2a,
  Nan3n,
}

/// Multiplies the value by 10 and adds the numer represented by a character.
macro_rules! mul_add {
  ($v:expr, $c:expr, $t:ty) => {{
    $v = $v * 10 + (($c as u8) - b'0') as $t
  }};
}

/// Returns not-a-number variant of the number.
macro_rules! nan {
  () => {
    return Number::NaN(false)
  };
}

/// Parses a number from scientific text.
///
/// Minimum parsed number is:
/// ```text
/// -340282366920938463463374607431768211455E-2147483647
/// ```
///
/// Maximum parsed number is:
/// ```text
/// +340282366920938463463374607431768211455E+2147483647
/// ```
///
/// # Panics
///
/// This function panics when the parsed number is less than the minimum value
/// or greater than the maximum value (overflow error is raised).
/// Overflows are not checked to get the maximum performance.
///
/// # Examples
///
/// Input text represents a finite number.
/// ```
/// use scidec::{Number, number_from_string};
///
/// let result = number_from_string("1234.5678e-2");
/// match result {
///   Number::Fin(false, 0, 12345678, -6) => {}
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
///   Number::Inf(false) => {}
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
///   Number::Inf(true) => {}
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
///   Number::NaN(false) => {}
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
///   Number::NaN(true) => {}
///   _ => panic!()
/// }
/// ```
pub fn number_from_string(input: &str) -> Number {
  if input.is_empty() {
    nan!();
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
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => nan!(),
      },
      State::LeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          mul_add!(value, ch, u128);
          state = State::DigitsBefore;
        }
        '.' => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => nan!(),
      },
      State::DigitsBefore => match ch {
        '0'..='9' => mul_add!(value, ch, u128),
        '.' => state = State::DigitsAfter,
        'E' | 'e' => state = State::ExponentSign,
        _ => nan!(),
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          exponent -= 1;
          mul_add!(value, ch, u128);
        }
        'E' | 'e' if position < last => state = State::ExponentSign,
        _ => nan!(),
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
        _ => nan!(),
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          mul_add!(exponent_base, ch, i32);
          state = State::ExponentDigits;
        }
        _ => nan!(),
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          mul_add!(exponent_base, ch, i32);
        }
        _ => nan!(),
      },
      State::Inf2n => match ch {
        'n' | 'N' => state = State::Inf3f,
        _ => nan!(),
      },
      State::Inf3f => match ch {
        'f' | 'F' if position == last => infinity = true,
        'f' | 'F' => state = State::Inf4i,
        _ => nan!(),
      },
      State::Inf4i => match ch {
        'i' | 'I' => state = State::Inf5n,
        _ => nan!(),
      },
      State::Inf5n => match ch {
        'n' | 'N' => state = State::Inf6i,
        _ => nan!(),
      },
      State::Inf6i => match ch {
        'i' | 'I' => state = State::Inf7t,
        _ => nan!(),
      },
      State::Inf7t => match ch {
        't' | 'T' => state = State::Inf8y,
        _ => nan!(),
      },
      State::Inf8y => match ch {
        'y' | 'Y' if position == last => infinity = true,
        _ => nan!(),
      },
      State::Nan1n => match ch {
        'n' | 'N' => state = State::Nan2a,
        _ => nan!(),
      },
      State::Nan2a => match ch {
        'a' | 'A' => state = State::Nan3n,
        _ => nan!(),
      },
      State::Nan3n => match ch {
        'n' | 'N' if position == last => nan = true,
        _ => nan!(),
      },
    }
  }
  if infinity {
    return Number::Inf(sign);
  }
  if nan {
    return Number::NaN(signaling);
  }
  Number::Fin(
    sign,
    (value >> 64) as u64,
    value as u64,
    exponent + exponent_sign * exponent_base,
  )
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    assert!((Number::Fin(false, 0, 0, 0) == Number::Fin(false, 0, 0, 0)));
    assert!((Number::Fin(false, 0, 0, 0) != Number::Inf(false)));
    assert!((Number::Inf(true) != Number::Inf(false)));
    assert!((Number::Inf(true) == Number::Inf(true)));
    assert!((Number::NaN(true) != Number::NaN(false)));
    assert!((Number::NaN(false) == Number::NaN(false)));
    Number::Inf(false).assert_receiver_is_total_eq();
  }
}
