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

//! Implementation of the recognizer for scientific `E` notation.

type Flags = u32;

pub const FLAG_OVERFLOW: Flags = 0x08;
pub const FLAG_UNDERFLOW: Flags = 0x10;
pub const FLAG_INEXACT: Flags = 0x20;

/// Rounding modes.
#[repr(i32)]
pub enum Rounding {
  ToNearest = 0x00000,
  Down = 0x00001,
  Up = 0x00002,
  ToZero = 0x00003,
  TiesAway = 0x00004,
}

impl From<i32> for Rounding {
  /// Converts [Rounding] from [i32].
  fn from(value: i32) -> Self {
    match value {
      0x00000 => Rounding::ToNearest,
      0x00001 => Rounding::Down,
      0x00002 => Rounding::Up,
      0x00003 => Rounding::ToZero,
      0x00004 => Rounding::TiesAway,
      _ => Rounding::ToNearest,
    }
  }
}

/// States of the finite state machine used to parse the input text.
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
    /// Exception status flags.
    Flags,
  ),
  /// Variant representing an infinity.
  Infinity(
    /// Flag indicating if the infinity is signed, if `true` then signed.
    bool,
  ),
  /// Variant representing an invalid number.
  NaN(
    /// Flag indicating if the value is signed, if `true` then signed.
    bool,
    /// Flag indicating if this is a signalling NaN, if `true` then signaling.
    bool,
  ),
}

/// Maximum number of recognized digits.
const MAX_STRING_DIGITS: usize = 100;

macro_rules! update_value {
  ($value:expr, $ch:expr, $digits:expr, $max_digits: expr, $digits_total:expr, $buffer:expr, $inexact:expr) => {{
    let b = ($ch as u8) - b'0';
    if $digits < $max_digits {
      $value = $value * 10 + b as u128;
      if $value > 0 {
        $digits += 1;
      }
    } else {
      $inexact = b > 0;
    }
    if $value > 0 && $digits_total < MAX_STRING_DIGITS {
      $buffer[$digits_total] = b;
      $digits_total += 1;
    }
  }};
}

macro_rules! update_exponent {
  ($v:expr, $c:expr) => {{
    $v = $v.saturating_mul(10).saturating_add((($c as u8) - b'0') as i32);
  }};
}

/// Recognizes a number from scientific notation.
pub fn recognize(input: &str, max_digits: usize, rnd: Rounding) -> Value {
  let mut sign = false;
  let mut signaling = false;
  if input.is_empty() {
    return Value::NaN(sign, signaling);
  }
  let mut state = State::BeginNumber;
  let mut exp = 0_i32;
  let mut exp_base = 0_i32;
  let mut exp_sign = 1_i32;
  let mut val = 0_u128;
  let mut inf = false;
  let mut nan = false;
  let mut digits = 0_usize;
  let mut digits_total = 0_usize;
  let mut inexact = false;
  let mut buffer = [0_u8; MAX_STRING_DIGITS];
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
          update_value!(val, ch, digits, max_digits, digits_total, buffer, inexact);
          state = State::DigitsBefore;
        }
        '.' if position < last => state = State::DigitsAfter,
        '.' if position == last => break,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return Value::NaN(sign, signaling),
      },
      State::LeadingZerosBefore => match ch {
        '0' => {}
        '1'..='9' => {
          update_value!(val, ch, digits, max_digits, digits_total, buffer, inexact);
          state = State::DigitsBefore;
        }
        '.' => state = State::DigitsAfter,
        'E' | 'e' => state = State::ExponentSign,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => return Value::NaN(sign, signaling),
      },
      State::DigitsBefore => match ch {
        '0'..='9' => {
          if digits == max_digits {
            exp += 1;
          }
          update_value!(val, ch, digits, max_digits, digits_total, buffer, inexact)
        }
        '.' => state = State::DigitsAfter,
        'E' | 'e' => state = State::ExponentSign,
        _ => return Value::NaN(sign, signaling),
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          if digits < max_digits {
            exp -= 1;
          }
          update_value!(val, ch, digits, max_digits, digits_total, buffer, inexact);
        }
        'E' | 'e' if position < last => state = State::ExponentSign,
        _ => return Value::NaN(sign, signaling),
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
        _ => return Value::NaN(sign, signaling),
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          update_exponent!(exp_base, ch);
          state = State::ExponentDigits;
        }
        _ => break,
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          update_exponent!(exp_base, ch);
        }
        _ => break,
      },
      State::Inf2n => match ch {
        'n' | 'N' => state = State::Inf3f,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf3f => match ch {
        'f' | 'F' if position == last => inf = true,
        'f' | 'F' => state = State::Inf4i,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf4i => match ch {
        'i' | 'I' => state = State::Inf5n,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf5n => match ch {
        'n' | 'N' => state = State::Inf6i,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf6i => match ch {
        'i' | 'I' => state = State::Inf7t,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf7t => match ch {
        't' | 'T' => state = State::Inf8y,
        _ => return Value::NaN(sign, signaling),
      },
      State::Inf8y => match ch {
        'y' | 'Y' if position == last => {
          inf = true;
          break;
        }
        _ => return Value::NaN(sign, signaling),
      },
      State::Nan1n => match ch {
        'n' | 'N' => state = State::Nan2a,
        _ => return Value::NaN(sign, false),
      },
      State::Nan2a => match ch {
        'a' | 'A' => state = State::Nan3n,
        _ => return Value::NaN(sign, false),
      },
      State::Nan3n => match ch {
        'n' | 'N' => {
          nan = true;
          break;
        }
        _ => return Value::NaN(sign, false),
      },
    }
  }

  // check for infinity
  if inf {
    // return +/-infinity
    return Value::Infinity(sign);
  }

  // check for invalid number
  if nan {
    // return +/-[s]nan
    return Value::NaN(sign, signaling);
  }

  // calculate final exponent
  exp = exp.saturating_add(exp_sign.saturating_mul(exp_base));

  // apply rounding if needed
  let mut flags: Flags = 0_u32;
  if digits_total > max_digits {
    let mut carry = 0_u32;
    let mut i = max_digits;
    match rnd {
      Rounding::ToNearest => {
        carry = ((4 - (buffer[i] as i32)) as u32) >> 31;
        if (buffer[i] == 5 && (buffer[i - 1] & 1) == 0) || exp < 0 {
          if exp >= 0 {
            carry = 0;
            i += 1;
          }
          for b in &buffer[i..digits_total] {
            if *b > 0 {
              carry = 1;
              break;
            }
          }
        }
      }
      Rounding::Down => {
        if sign {
          for b in &buffer[i..digits_total] {
            if *b > 0 {
              carry = 1;
              break;
            }
          }
        }
      }
      Rounding::Up => {
        if !sign {
          for b in &buffer[i..digits_total] {
            if *b > 0 {
              carry = 1;
              break;
            }
          }
        }
      }
      Rounding::ToZero => {
        carry = 0;
      }
      Rounding::TiesAway => {
        carry = ((4 - (buffer[i] as i32)) as u32) >> 31;
        if exp < 0 {
          for b in &buffer[i..digits_total] {
            if *b > 0 {
              carry = 1;
              break;
            }
          }
        }
      }
    }
    val += carry as u128;
    if inexact {
      flags |= FLAG_INEXACT;
    }
  }

  // return finite number
  Value::Finite(sign, val, exp, flags)
}
