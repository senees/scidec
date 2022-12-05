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
pub fn number_from_string(input: &str) -> Number {
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
          value = value * 10 + ((ch as u8) - b'0') as u128;
          state = State::DigitsBefore;
        }
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => panic!("ERROR1"),
      },
      State::LeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          value = value * 10 + ((ch as u8) - b'0') as u128;
          state = State::DigitsBefore;
        }
        '.' => state = State::DigitsAfter,
        'i' | 'I' => state = State::Inf2n,
        'n' | 'N' => state = State::Nan2a,
        's' | 'S' => {
          signaling = true;
          state = State::Nan1n
        }
        _ => panic!("ERROR2"),
      },
      State::DigitsBefore => match ch {
        '0'..='9' => {
          value = value * 10 + ((ch as u8) - b'0') as u128;
        }
        _ => panic!("ERROR3"),
      },
      State::DigitsAfter => match ch {
        '0'..='9' => {
          exponent -= 1;
          value = value * 10 + ((ch as u8) - b'0') as u128;
        }
        'E' | 'e' => state = State::ExponentSign,
        _ => panic!("ERROR4"),
      },
      State::ExponentSign => match ch {
        '+' | '0' => state = State::ExponentLeadingZeros,
        '-' => {
          exponent_sign = -1_i32;
          state = State::ExponentLeadingZeros;
        }
        '1'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - b'0') as i32;
          state = State::ExponentDigits;
        }
        _ => panic!("ERROR5"),
      },
      State::ExponentLeadingZeros => match ch {
        '0' => {}
        '1'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - b'0') as i32;
          state = State::ExponentDigits;
        }
        _ => panic!("ERROR6"),
      },
      State::ExponentDigits => match ch {
        '0'..='9' => {
          exponent_base = exponent_base * 10 + ((ch as u8) - b'0') as i32;
        }
        _ => panic!("ERROR7"),
      },
      State::Inf2n => match ch {
        'n' | 'N' => state = State::Inf3f,
        _ => panic!("ERROR8"),
      },
      State::Inf3f => match ch {
        'f' | 'F' if position == last => infinity = true,
        'f' | 'F' => state = State::Inf4i,
        _ => panic!("ERROR9"),
      },
      State::Inf4i => match ch {
        'i' | 'I' => state = State::Inf5n,
        _ => panic!("ERROR10"),
      },
      State::Inf5n => match ch {
        'n' | 'N' => state = State::Inf6i,
        _ => panic!("ERROR11"),
      },
      State::Inf6i => match ch {
        'i' | 'I' => state = State::Inf7t,
        _ => panic!("ERROR12"),
      },
      State::Inf7t => match ch {
        't' | 'T' => state = State::Inf8y,
        _ => panic!("ERROR13"),
      },
      State::Inf8y => match ch {
        'y' | 'Y' if position == last => infinity = true,
        _ => panic!("ERROR14"),
      },
      State::Nan1n => match ch {
        'n' | 'N' => state = State::Nan2a,
        _ => panic!("ERROR15"),
      },
      State::Nan2a => match ch {
        'a' | 'A' => state = State::Nan3n,
        _ => panic!("ERROR16"),
      },
      State::Nan3n => match ch {
        'n' | 'N' if position == last => nan = true,
        _ => panic!("ERROR17"),
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
