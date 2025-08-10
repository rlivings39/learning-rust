use std::{error, fmt};

#[derive(Debug)]
pub struct ValError {
  msg: String,
}

impl error::Error for ValError {}
impl fmt::Display for ValError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.msg)
  }
}
pub struct Guess {
  value: i32,
}

impl Guess {
  pub fn new(val: i32) -> Result<Guess, ValError> {
    if val < 1 || val > 100 {
      return Err(ValError {
        msg: "Value should be between 1 and 100".to_string(),
      });
    }
    Ok(Guess { value: val })
  }
  pub fn value(&self) -> i32 {
    self.value
  }
}
