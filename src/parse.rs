
use core::fmt;
use std::error;


pub fn get_time(time: String) -> Result<(u8, u8, u8), Box<dyn error::Error>> {
  let times = time.split(":").collect::<Vec<&str>>();
  match times.len() {
    2 => {
      let min = times[0].trim().parse::<u8>()?;
      let sec = times[1].trim().parse::<u8>()?;
      if min > 60 || sec > 60 {
        return Err(Box::new(BadInputError));
      }
      Ok((0, min, sec))
    }
    3 => {
      let hr = times[0].trim().parse::<u8>()?;
      let min = times[1].trim().parse::<u8>()?;
      let sec = times[2].trim().parse::<u8>()?;
      if hr > 60 || min > 60 || sec > 60 {
        return Err(Box::new(BadInputError));
      }
      Ok((hr, min, sec))

    }
    _ => Err(Box::new(BadInputError)),
  }
}

#[derive(Debug)]
struct BadInputError;
impl error::Error for BadInputError {}
impl fmt::Display for BadInputError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Invalid input")
  }
}


#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_get_time_with_hour() {
    if let Ok((hr, min, sec)) = get_time("1:23:45".to_string()) {
      assert_eq!(hr, 1);
      assert_eq!(min, 23);
      assert_eq!(sec, 45);
    }
  }

  #[test]
  fn test_get_time_with_no_hour() {
    if let Ok((hr, min, sec)) = get_time("23:45".to_string()) {
      assert_eq!(hr, 0);
      assert_eq!(min, 23);
      assert_eq!(sec, 45);
    }
  }

  #[test]
  fn test_get_time_with_zeros() {
    if let Ok((hr, min, sec)) = get_time("01:05:09".to_string()) {
      assert_eq!(hr, 1);
      assert_eq!(min, 5);
      assert_eq!(sec, 9);
    }
  }

  #[test]
  fn test_get_time_bad_inputs() {
    let tests = ["total nonsense", "80:01:01", "300:01", "234234"];

    for test in tests {
      if let Ok(_) = get_time(test.to_string()) {
        panic!("{} should have thrown an error", test)
      }
    }

  }
}