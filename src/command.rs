use regex::Regex;
use std::{fmt::Display, fs::read_to_string, ops::Range, path::Path, str::FromStr};
#[derive(Debug, PartialEq)]
pub enum Command {
    Input {
        pin_name: String,
        bits: Range<u8>,
        array: Range<u8>,
    },
    Output {
        pin_name: String,
        bits: Range<u8>,
        array: Range<u8>,
    },
    Inout {
        pin_name: String,
        bits: Range<u8>,
        array: Range<u8>,
    },
    Register {
        reg_name: String,
        bits: Range<u8>,
        array: Range<u8>,
    },
    RegisterTransfer {
        reg_name: String,
        reg_value: String,
    },
    Then {
        next_node: String,
    },
    Check {
        check: String,
    },
    Yes {
        next_node: String,
    },
    No {
        next_node: String,
    },
    Empty,
}
#[derive(Debug, PartialEq)]
pub enum UnableToParseError {
    InvalidFormat,
    InvalidRange,
}
impl FromStr for Command {
    type Err = UnableToParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.trim().is_empty() {
            return Ok(Command::Empty);
        }
        let mut parts = s.split("=>");

        if let (Some(lhs), Some(rhs)) = (parts.next(), parts.next()) {
            match lhs.trim() {
                "then" => Ok(Self::Then {
                    next_node: rhs.trim().to_string(),
                }),
                "yes" => Ok(Self::Yes {
                    next_node: rhs.trim().to_string(),
                }),
                "no" => Ok(Self::No {
                    next_node: rhs.trim().to_string(),
                }),
                "check" => Ok(Self::Check {
                    check: rhs.trim().to_string(),
                }),
                _ => unsafe {
                    if SINGLE_BIT_INPUT.is_match(rhs.trim()) {
                        Ok(Self::Input {
                            pin_name: lhs.trim().to_string(),
                            bits: 0..0,
                            array: 0..0,
                        })
                    } else if SINGLE_BIT_OUTPUT.is_match(rhs.trim()) {
                        Ok(Self::Output {
                            pin_name: lhs.trim().to_string(),
                            bits: 0..0,
                            array: 0..0,
                        })
                    } else if SINGLE_BIT_INOUT.is_match(rhs.trim()) {
                        Ok(Self::Inout {
                            pin_name: lhs.trim().to_string(),
                            bits: 0..0,
                            array: 0..0,
                        })
                    } else if SINGLE_BIT_REG.is_match(rhs.trim()) {
                        Ok(Self::Register {
                            reg_name: lhs.trim().to_string(),
                            bits: 0..0,
                            array: 0..0,
                        })
                    } else if let Some(capt) = MULTI_BIT_INPUT.captures(rhs.trim()) {
                        if let (Ok(l), Ok(r)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Input {
                                pin_name: lhs.trim().to_string(),
                                bits: l..r,
                                array: 0..0,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = MULTI_BIT_OUTPUT.captures(rhs.trim()) {
                        if let (Ok(l), Ok(r)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Output {
                                pin_name: lhs.trim().to_string(),
                                bits: l..r,
                                array: 0..0,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = MULTI_BIT_INOUT.captures(rhs.trim()) {
                        if let (Ok(l), Ok(r)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Inout {
                                pin_name: lhs.trim().to_string(),
                                bits: l..r,
                                array: 0..0,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = MULTI_BIT_REG.captures(rhs.trim()) {
                        if let (Ok(l), Ok(r)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Register {
                                reg_name: lhs.trim().to_string(),
                                bits: l..r,
                                array: 0..0,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = ARRAY_INPUT.captures(rhs.trim()) {
                        if let (Ok(l1), Ok(r1), Ok(l2), Ok(r2)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                            capt.get(3).unwrap().as_str().parse(),
                            capt.get(4).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Input {
                                pin_name: lhs.trim().to_string(),
                                bits: l2..r2,
                                array: l1..r1,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = ARRAY_OUTPUT.captures(rhs.trim()) {
                        if let (Ok(l1), Ok(r1), Ok(l2), Ok(r2)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                            capt.get(3).unwrap().as_str().parse(),
                            capt.get(4).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Output {
                                pin_name: lhs.trim().to_string(),
                                bits: l2..r2,
                                array: l1..r1,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = ARRAY_INOUT.captures(rhs.trim()) {
                        if let (Ok(l1), Ok(r1), Ok(l2), Ok(r2)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                            capt.get(3).unwrap().as_str().parse(),
                            capt.get(4).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Inout {
                                pin_name: lhs.trim().to_string(),
                                bits: l2..r2,
                                array: l1..r1,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else if let Some(capt) = ARRAY_REG.captures(rhs.trim()) {
                        if let (Ok(l1), Ok(r1), Ok(l2), Ok(r2)) = (
                            capt.get(1).unwrap().as_str().parse(),
                            capt.get(2).unwrap().as_str().parse(),
                            capt.get(3).unwrap().as_str().parse(),
                            capt.get(4).unwrap().as_str().parse(),
                        ) {
                            Ok(Self::Register {
                                reg_name: lhs.trim().to_string(),
                                bits: l2..r2,
                                array: l1..r1,
                            })
                        } else {
                            Err(UnableToParseError::InvalidRange)
                        }
                    } else {
                        Ok(Self::RegisterTransfer {
                            reg_name: lhs.trim().to_string(),
                            reg_value: rhs.trim().to_string(),
                        })
                    }
                },
            }
        } else {
            Err(UnableToParseError::InvalidFormat)
        }
    }
}
lazy_static::lazy_static! {
        static ref SINGLE_BIT_INPUT : Regex = Regex::new(r"^input$").unwrap();
        static ref SINGLE_BIT_OUTPUT : Regex = Regex::new(r"^output$").unwrap();
        static ref SINGLE_BIT_INOUT: Regex  = Regex::new(r"^inout$").unwrap();
        static ref SINGLE_BIT_REG : Regex = Regex::new(r"^reg$").unwrap();
        static ref MULTI_BIT_INPUT : Regex = Regex::new(r"^input *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref MULTI_BIT_OUTPUT : Regex = Regex::new(r"^output *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref MULTI_BIT_INOUT : Regex = Regex::new(r"^inout *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref MULTI_BIT_REG : Regex = Regex::new(r"^reg *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref ARRAY_INPUT : Regex =
            Regex::new(r"^input *\[ *(\d+) *: *(\d+) *\] *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref ARRAY_OUTPUT : Regex =
            Regex::new(r"^output *\[ *(\d+) *: *(\d+) *\] *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref ARRAY_INOUT : Regex =
            Regex::new(r"^inout *\[ *(\d+) *: *(\d+) *\] *\[ *(\d+) *: *(\d+) *\]$").unwrap();
        static ref ARRAY_REG : Regex =
            Regex::new(r"^reg *\[ *(\d+) *: *(\d+) *\] *\[ *(\d+) *: *(\d+) *\]$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yes_test() {
        let cmd = "   yes  =>   .help ".parse::<Command>();
        match cmd {
            Ok(Command::Yes { next_node }) => assert_eq!(next_node, ".help".to_string()),
            _ => assert!(false),
        }

        let cmd = " yes      =>   .test2     ".parse::<Command>();
        match cmd {
            Ok(Command::Yes { next_node }) => assert_eq!(next_node, ".test2".to_string()),
            _ => assert!(false),
        }
    }
    #[test]
    fn no_test() {
        let cmd = "   no  =>   .help ".parse::<Command>();
        match cmd {
            Ok(Command::No { next_node }) => assert_eq!(next_node, ".help".to_string()),
            _ => assert!(false),
        }

        let cmd = " no      =>   .test2     ".parse::<Command>();
        match cmd {
            Ok(Command::No { next_node }) => assert_eq!(next_node, ".test2".to_string()),
            _ => assert!(false),
        }
    }
    #[test]
    fn then_test() {
        let cmd = "  then  =>   .help ".parse::<Command>();
        match cmd {
            Ok(Command::Then { next_node }) => assert_eq!(next_node, ".help".to_string()),
            _ => assert!(false),
        }

        let cmd = " then      =>   .test2     ".parse::<Command>();
        match cmd {
            Ok(Command::Then { next_node }) => assert_eq!(next_node, ".test2".to_string()),
            _ => assert!(false),
        }
    }
    #[test]
    fn check_test() {
        let cmd = "  check  =>   r[0] | r[1]  ".parse::<Command>();
        match cmd {
            Ok(Command::Check { check }) => assert_eq!(check, "r[0] | r[1]".to_string()),
            _ => assert!(false),
        }

        let cmd = " check      =>   r[1] & r[2]    ".parse::<Command>();
        match cmd {
            Ok(Command::Check { check }) => assert_eq!(check, "r[1] & r[2]".to_string()),
            _ => assert!(false),
        }
    }
    #[test]
    fn regtrans_test() {
        let cmd = "  r0  =>   r0 + r1  ".parse::<Command>();
        match cmd {
            Ok(Command::RegisterTransfer {
                reg_name,
                reg_value,
            }) => {
                assert_eq!(reg_name, "r0".to_string());
                assert_eq!(reg_value, "r0 + r1".to_string());
            }
            _ => assert!(false),
        }

        let cmd = "     r0   =>   r2 * r3  ".parse::<Command>();
        match cmd {
            Ok(Command::RegisterTransfer {
                reg_name,
                reg_value,
            }) => {
                assert_eq!(reg_name, "r0".to_string());
                assert_eq!(reg_value, "r2 * r3".to_string());
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn reg_test() {
        let cmd = "  r0  =>   reg  ".parse::<Command>();
        match cmd {
            Ok(Command::Register {
                reg_name,
                bits,
                array,
            }) => {
                assert_eq!(reg_name, "r0".to_string());
                assert_eq!(bits, 0..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   reg[   3 :0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Register {
                reg_name,
                bits,
                array,
            }) => {
                assert_eq!(reg_name, "r0".to_string());
                assert_eq!(bits, 3..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   reg[ 4 :0 ][1 : 0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Register {
                reg_name,
                bits,
                array,
            }) => {
                assert_eq!(reg_name, "r0".to_string());
                assert_eq!(bits, 1..0);
                assert_eq!(array, 4..0);
            }
            _ => assert!(false),
        }
    }

    #[test]
    fn input_test() {
        let cmd = "  r0  =>   input  ".parse::<Command>();
        match cmd {
            Ok(Command::Input {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 0..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   input[   3 :0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Input {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 3..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   input[ 4 :0 ][1 : 0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Input {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 1..0);
                assert_eq!(array, 4..0);
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn output_test() {
        let cmd = "  r0  =>   output  ".parse::<Command>();
        match cmd {
            Ok(Command::Output {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 0..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   output[   3 :0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Output {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 3..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   output[ 4 :0 ][1 : 0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Output {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 1..0);
                assert_eq!(array, 4..0);
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn inout_test() {
        let cmd = "  r0  =>   inout  ".parse::<Command>();
        match cmd {
            Ok(Command::Inout {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 0..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   inout[   3 :0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Inout {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 3..0);
                assert_eq!(array, 0..0);
            }
            _ => assert!(false),
        }

        let cmd = "  r0  =>   inout[ 4 :0 ][1 : 0]  ".parse::<Command>();
        match cmd {
            Ok(Command::Inout {
                pin_name,
                bits,
                array,
            }) => {
                assert_eq!(pin_name, "r0".to_string());
                assert_eq!(bits, 1..0);
                assert_eq!(array, 4..0);
            }
            _ => assert!(false),
        }
    }
    #[test]
    fn empty_test() {
        let cmd = "       ".parse::<Command>();
        match cmd {
            Ok(Command::Empty) => {}
            _ => assert!(false),
        }
    }
}
