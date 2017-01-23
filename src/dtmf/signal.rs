use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FormatResult};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
/// A valid signal for DTMF.
pub enum Signal {
    /// A digit from 0 - 9.
    Digit(u8),
    /// Encodes the signal "A".
    A,
    /// Encodes the signal "B".
    B,
    /// Encodes the signal "C".
    C,
    /// Encodes the signal "D".
    D,
    /// Encodes the signal "*".
    Asterisk,
    /// Encodes the signal "#".
    Hash,
}

impl Signal {
    /// Generates a signal from a char.
    /// # Example
    /// ```
    /// use ::dtmf::{Signal, SignalParsingError};
    ///
    /// assert_eq!(Signal::from_char('A'), Ok(Signal::A));
    /// assert_eq!(Signal::from_char('7'), Ok(Signal::Digit(7)));
    /// assert_eq!(Signal::from_char('💣'), Err(SignalParsingError::UnknownSignal('💣')));
    /// ```
    pub fn from_char(input: char) -> Result<Self, SignalParsingError> {
        match input {
            '*' => Ok(Signal::Asterisk),
            '#' => Ok(Signal::Hash),
            'A' | 'a' => Ok(Signal::A),
            'B' | 'b' => Ok(Signal::B),
            'C' | 'c' => Ok(Signal::C),
            'D' | 'd' => Ok(Signal::D),
            number @ '0'...'9' => {
                Ok(Signal::Digit(number.to_digit(10).expect("Invalid number") as u8))
            }
            unknown @ _ => Err(SignalParsingError::UnknownSignal(unknown)),
        }
    }

    /// Returns the lower and the upper frequency of the signal according to the standardization.
    /// # Example
    /// ```
    /// use ::dtmf::Signal;
    ///
    /// let signal = Signal::A;
    /// let (low, high) = signal.frequencies().expect("Valid signal");
    ///
    /// assert_eq!(low, 697);
    /// assert_eq!(high, 1633);
    /// ```
    pub fn frequencies(&self) -> Option<(u16, u16)> {

        // Just a bunch of constants...
        match *self {
            // Valid digits
            Signal::Digit(0) => Some((941, 1336)),
            Signal::Digit(1) => Some((697, 1209)),
            Signal::Digit(2) => Some((697, 1336)),
            Signal::Digit(3) => Some((697, 1477)),
            Signal::Digit(4) => Some((770, 1209)),
            Signal::Digit(5) => Some((770, 1336)),
            Signal::Digit(6) => Some((770, 1477)),
            Signal::Digit(7) => Some((852, 1209)),
            Signal::Digit(8) => Some((852, 1336)),
            Signal::Digit(9) => Some((852, 1477)),
            // Valid letters
            Signal::A => Some((697, 1633)),
            Signal::B => Some((770, 1633)),
            Signal::C => Some((852, 1633)),
            Signal::D => Some((941, 1633)),
            // Other symbols
            Signal::Asterisk => Some((941, 1209)),
            Signal::Hash => Some((941, 1477)),
            // Invalid digit
            _ => None,
        }
    }
}

/// An error occurring while parsing a signal.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SignalParsingError {
    /// The given input was empty.
    TooShort,
    /// The given input contains more than just one signal.
    TooLong,
    /// An unknown signal appears.
    UnknownSignal(char),
}

impl FromStr for Signal {
    type Err = SignalParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {

        // We need only ASCII chars: A length check on byte number is sufficient.
        match s.len() {
            0 => Err(SignalParsingError::TooShort),
            1 => Signal::from_char(s.chars().next().expect("Char missing")),
            _ => Err(SignalParsingError::TooLong),
        }
    }
}

impl Display for Signal {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        write!(f,
               "{}",
               match *self {
                   Signal::Asterisk => '*',
                   Signal::Hash => '#',
                   Signal::A => 'A',
                   Signal::B => 'B',
                   Signal::C => 'C',
                   Signal::D => 'D',
                   Signal::Digit(number) => char::from(number),
               })
    }
}
