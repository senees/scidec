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

//! Finite state machine for recognizing numbers from scientific `E` notation.

/// States of the FSM used for parsing the input text.
enum State {
  BeginNumber,
  LeadingZerosBefore,
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

/// Parsed number.
#[derive(Eq, PartialEq)]
pub enum Value {
  /// Variant representing a finite number.
  Finite(
    /// Flag indicating if the number is signed, if `true` then signed.
    bool,
    /// Recognized value.
    u128,
    /// Exponent.
    i32,
  ),
  /// Variant representing an infinity.
  Infinite(
    /// Flag indicating if the infinity is signed, if `true` then signed.
    bool,
  ),
  /// Variant representing an invalid number.
  NotANumber(
    /// Flag indicating if this is a signalling NaN, if `true` then signaling.
    bool,
  ),
}

macro_rules! update_value {
  ($value:expr, $ch:expr, $digits:expr, $max_digits: expr) => {{
    if $digits <= $max_digits {
      $value = $value * 10 + (($ch as u8) - b'0') as u128;
      if $value > 0 {
        $digits += 1;
      }
    }
  }};
}

macro_rules! update_exponent {
  ($v:expr, $c:expr) => {{
    $v = $v.saturating_mul(10).saturating_add((($c as u8) - b'0') as i32);
  }};
}

pub fn recognize(input: &str, max_digits: u32) -> Value {
  if input.is_empty() {
    return Value::NotANumber(false);
  }
  let mut state = State::BeginNumber;
  let mut sign = false;
  let mut exp = 0_i32;
  let mut exp_base = 0_i32;
  let mut exp_sign = 1_i32;
  let mut val = 0_u128;
  let mut inf = false;
  let mut nan = false;
  let mut signaling = false;
  let mut digits = 0_u32;
  let last = input.len() - 1;
  for (position, ch) in input.chars().enumerate() {
    match state {
      State::BeginNumber => match ch {
        '-' => {
          sign = true;
          state = State::LeadingZerosBefore;
        }
        '+' | '0' => state = State::LeadingZerosBefore,
        '1'..='9' => {
          update_value!(val, ch, digits, max_digits);
          state = State::DigitsBefore;
        }
        '.' if position < last => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return Value::NotANumber(false),
      },
      State::LeadingZerosBefore => match ch {
        '0' => {}
        '1'..='9' => {
          update_value!(val, ch, digits, max_digits);
          state = State::DigitsBefore;
        }
        '.' if position == last => {
          exp -= 1;
          update_value!(val, b'0', digits, max_digits);
        }
        '.' => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return Value::NotANumber(false),
      },
      State::DigitsBefore => match ch {
        '0'..='9' => update_value!(val, ch, digits, max_digits),
        '.' if position == last => {
          exp -= 1;
          update_value!(val, b'0', digits, max_digits);
        }
        '.' => state = State::DigitsAfter,
        'E' | 'e' => state = State::ExponentSign,
        _ => return Value::NotANumber(false),
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          exp -= 1;
          update_value!(val, ch, digits, max_digits);
        }
        'E' | 'e' if position < last => state = State::ExponentSign,
        _ => return Value::NotANumber(false),
      },
      State::ExponentSign => match ch {
        '+' | '0' if position < last => state = State::ExponentLeadingZeros,
        '-' if position < last => {
          exp_sign = -1_i32;
          state = State::ExponentLeadingZeros;
        }
        '1'..='9' => {
          update_exponent!(exp_base, ch);
          state = State::ExponentDigits;
        }
        _ => return Value::NotANumber(false),
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          update_exponent!(exp_base, ch);
          state = State::ExponentDigits;
        }
        _ => return Value::NotANumber(false),
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          update_exponent!(exp_base, ch);
        }
        _ => return Value::NotANumber(false),
      },
      State::Inf2n => match ch {
        'n' | 'N' => state = State::Inf3f,
        _ => return Value::NotANumber(false),
      },
      State::Inf3f => match ch {
        'f' | 'F' if position == last => inf = true,
        'f' | 'F' => state = State::Inf4i,
        _ => return Value::NotANumber(false),
      },
      State::Inf4i => match ch {
        'i' | 'I' => state = State::Inf5n,
        _ => return Value::NotANumber(false),
      },
      State::Inf5n => match ch {
        'n' | 'N' => state = State::Inf6i,
        _ => return Value::NotANumber(false),
      },
      State::Inf6i => match ch {
        'i' | 'I' => state = State::Inf7t,
        _ => return Value::NotANumber(false),
      },
      State::Inf7t => match ch {
        't' | 'T' => state = State::Inf8y,
        _ => return Value::NotANumber(false),
      },
      State::Inf8y => match ch {
        'y' | 'Y' if position == last => inf = true,
        _ => return Value::NotANumber(false),
      },
      State::Nan1n => match ch {
        'n' | 'N' => state = State::Nan2a,
        _ => return Value::NotANumber(false),
      },
      State::Nan2a => match ch {
        'a' | 'A' => state = State::Nan3n,
        _ => return Value::NotANumber(false),
      },
      State::Nan3n => match ch {
        'n' | 'N' if position == last => nan = true,
        _ => return Value::NotANumber(false),
      },
    }
  }
  if inf {
    return Value::Infinite(sign);
  }
  if nan {
    return Value::NotANumber(signaling);
  }
  Value::Finite(sign, val, exp + exp_sign * exp_base)
}
