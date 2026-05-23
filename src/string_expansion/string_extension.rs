use crate::errors::{ ParseErrors };

// === Traits Declared Here === //
pub trait ExtendedStrings {
    fn is_int(&self) -> bool;
    fn is_float(&self) -> bool;
    fn is_bool(&self) -> bool;
    fn to_int(&self) -> Result<i64, ParseErrors>;
    fn to_float(&self) -> Result<f64, ParseErrors>;
    fn to_bool(&self) -> Result<bool, ParseErrors>;
}

// === Traits Implemented Here === //
impl ExtendedStrings for str {
    fn is_int(&self) -> bool {
        return self.parse::<i64>().is_ok();
    }

    fn is_float(&self) -> bool {
        return self.parse::<f64>().is_ok();
    }

    fn is_bool(&self) -> bool {
        if self == "true" || self == "false" {
            return true;
        }
        return false;
    }

    fn to_int(&self) -> Result<i64, ParseErrors> {
        match self.parse::<i64>() {
            Ok(int) => return Ok(int),
            Err(_) => return Err(ParseErrors::NotAValidInt),
        }
    }

    fn to_float(&self) -> Result<f64, ParseErrors> {
        match self.parse::<f64>() {
            Ok(float) => return Ok(float),
            Err(_) => return Err(ParseErrors::NotAValidFloat),
        }
    }

    fn to_bool(&self) -> Result<bool, ParseErrors> {
        match self {
            "true" => return Ok(true),
            "false" => return Ok(false),
            _ => return Err(ParseErrors::NotAValidBool),
        }
    }
}

// === UNIT TESTS FOR THIS PART OF THE CRATE HERE === //
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_int_true() {
        let x: String = String::from("69");
        let flag: bool = x.is_int();
        assert_eq!(flag, true, "[TEST]: String {} should have evalueted to true because it is indeed an integer", x);
    }

    #[test]
    fn test_is_int_false() {
        let x: String = String::from("42.6");
        let flag: bool = x.is_int();
        assert_eq!(flag, false, "[TEST]: String {} should have evaluated to false as it is a floating point integer", x);
    }

    #[test]
    fn test_is_int_alphanumerical() {
        let x: String = String::from("abc");
        let flag: bool = x.is_int();
        assert_eq!(flag, false, "[TEST]: String {} should have evaluated to false as it is not a number", x);
    }

    #[test]
    fn test_is_float_true() {
        let x: String = String::from("69.42");
        let flag: bool = x.is_float();
        assert_eq!(flag, true, "[TEST]: String {} should have evalueted to true because it is indeed an integer", x);
    }

    #[test]
    fn test_is_float_alphanumerical() {
        let x: String = String::from("abc");
        let flag: bool = x.is_float();
        assert_eq!(flag, false, "[TEST]: String {} should have evaluated to false as it is not a number", x);
    }
}
