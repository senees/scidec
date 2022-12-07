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

use crate::fsm::{recognize, Value};

/// 128-bit decimal in binary format.
#[derive(Eq, PartialEq)]
pub struct Bid128 {
  pub w: [u64; 2],
}

const BID128_BIAS: i32 = 6176;

const BID128_EMAX: i32 = 6144;

const BID128_NAX_DIGITS: i32 = 34;

const BID128_SIGN: u64 = 0x8000000000000000;

const BID128_NAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7c00000000000000],
};

const BID128_NEG_NAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xfc00000000000000],
};

const BID128_SNAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7e00000000000000],
};

const BID128_NEG_SNAN: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xfe00000000000000],
};

const BID128_INF: Bid128 = Bid128 {
  w: [0x0000000000000000, 0x7800000000000000],
};

const BID128_NEG_INF: Bid128 = Bid128 {
  w: [0x0000000000000000, 0xf800000000000000],
};

/// Parses a 128-bit floating-point decimal from text in scientific notation.
pub fn bid128_from_string(input: &str) -> (Bid128, u32) {
  match recognize(input, 34) {
    Value::Finite(sign, value, exponent) => {
      let mut flags = 0;
      let mut e = 0_u64;
      if value == 0 {
        if exponent < -(BID128_BIAS + BID128_NAX_DIGITS) {
          flags = 0x30;
          e = 0;
        } else if exponent < -BID128_BIAS {
          e = 0;
        } else if exponent < BID128_EMAX - BID128_NAX_DIGITS + 1 {
          e = (BID128_BIAS + exponent) as u64;
        } else {
          e = (BID128_BIAS + BID128_EMAX - BID128_NAX_DIGITS + 1) as u64;
        }
      } else {
        match (value == 0, exponent > 6176, exponent < -6176) {
          (_, false, false) => e = (6176_i32 + exponent) as u64,
          (false, false, true) => return (BID128_NEG_INF, flags),
          (false, true, false) => return (BID128_INF, flags),
          (true, true, false) => e = 12352_u64,
          _ => {}
        };
      }
      let s = if sign { BID128_SIGN } else { 0 };
      (
        Bid128 {
          w: [value as u64, ((value >> 64) as u64) | e << 49 | s],
        },
        flags,
      )
    }
    Value::Infinite(sign) => {
      if sign {
        (BID128_NEG_INF, 0)
      } else {
        (BID128_INF, 0)
      }
    }
    Value::NotANumber(sign, signaling) => match (sign, signaling) {
      (false, false) => (BID128_NAN, 0),
      (false, true) => (BID128_SNAN, 0),
      (true, false) => (BID128_NEG_NAN, 0),
      (true, true) => (BID128_NEG_SNAN, 0),
    },
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_eq() {
    BID128_NAN.assert_receiver_is_total_eq();
  }
}
