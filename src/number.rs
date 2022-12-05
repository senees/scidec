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

///
#[derive(Debug, Eq, PartialEq)]
pub enum Number {
  Finite(bool, u64, u64, i32),
  Inf(bool),
  NaN(bool),
}

///
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

///
macro_rules! mul_add {
  ($v:expr, $c:expr, $t:ty) => {{
    $v = $v * 10 + (($c as u8) - b'0') as $t
  }};
}

///
macro_rules! nan {
  () => {
    return Number::NaN(false)
  };
}

///
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
  Number::Finite(sign, (value >> 64) as u64, value as u64, exponent + exponent_sign * exponent_base)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_debug() {
    assert_eq!("Finite(false, 0, 0, 0)", format!("{:?}", Number::Finite(false, 0, 0, 0)));
    assert_eq!("Inf(false)", format!("{:?}", Number::Inf(false)));
    assert_eq!("NaN(false)", format!("{:?}", Number::NaN(false)));
  }

  #[test]
  fn test_eq() {
    assert_eq!(Number::Finite(false, 0, 0, 0), Number::Finite(false, 0, 0, 0));
    assert_ne!(Number::Finite(false, 0, 0, 0), Number::Inf(false));
    Number::Inf(false).assert_receiver_is_total_eq();
  }
}
