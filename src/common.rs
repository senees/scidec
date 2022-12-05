//! Common definitions.

/// States of the FSM used for parsing the input text.
pub enum State {
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

pub(crate) use mul_add;
