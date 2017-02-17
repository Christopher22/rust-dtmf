use std::collections::vec_deque::Iter;
use std::collections::VecDeque;
use std::str::FromStr;
use std::fmt::{Display, Formatter, Result as FormatResult};

use super::{Signal, SignalParsingError};

/// An iterator over the signals in a message.
pub type SignalIterator<'a> = Iter<'a, Signal>;

/// A message of `Signal`s in a queue.
/// # Hint
/// A `Message` guaranties correctness of its signals.
#[derive(Clone, PartialEq, Debug)]
pub struct Message {
    signals: VecDeque<Signal>,
    signal_duration: f64,
    silence_duration: f64,
}

impl Message {
    /// Creates a new message with given durations.
    pub fn new(signal_duration: f64, silence_duration: f64) -> Message {
        Message {
            signals: VecDeque::new(),
            signal_duration: signal_duration,
            silence_duration: silence_duration,
        }
    }

    /// Creates a message from a slice of signals.
    /// # Example
    /// ```
    /// use dtmf::{Message, Signal};
    ///
    /// assert!(Message::from_slice(&[Signal::A, Signal::B]).is_some());
    /// assert!(Message::from_slice(&[Signal::Digit(42)]).is_none());
    /// ```
    pub fn from_slice(signals: &[Signal]) -> Option<Message> {
        if signals.iter().all(|signal| signal.frequencies().is_some()) {
            let mut message = Message::default();
            for signal in signals {
                message.enqueue(*signal);
            }
            Some(message)
        } else {
            None
        }
    }

    /// Sets the duration of each signal in seconds.
    pub fn set_signal_duration(&mut self, duration: f64) {
        self.signal_duration = duration;
    }

    /// Sets the duration of the silence between the signals in seconds.
    pub fn set_silence_duration(&mut self, duration: f64) {
        self.silence_duration = duration;
    }

    /// The duration of each signal in seconds.
    pub fn signal_duration(&self) -> f64 {
        self.signal_duration
    }

    /// The duration of the silence between the signals in seconds.
    pub fn silence_duration(&self) -> f64 {
        self.silence_duration
    }

    /// Adds a signal to the queue, iff it is valid.
    /// # Example
    /// ```
    /// use dtmf::{Message, Signal};
    ///
    /// let mut message = Message::default();
    /// assert!(message.enqueue(Signal::A));
    /// assert!(!message.enqueue(Signal::Digit(42)));
    /// ```
    pub fn enqueue(&mut self, signal: Signal) -> bool {
        match signal.frequencies() {
            Some(_) => {
                self.signals.push_back(signal);
                true
            }
            None => false,
        }
    }

    /// Returns the first signal from the queue.
    /// # Example
    /// ```
    /// use dtmf::{Message, Signal};
    ///
    /// let mut message = Message::default();
    /// message.enqueue(Signal::A);
    /// message.enqueue(Signal::B);
    /// assert_eq!(message.dequeue(), Some(Signal::A));
    /// assert_eq!(message.dequeue(), Some(Signal::B));
    /// assert_eq!(message.dequeue(), None);
    /// ```
    pub fn dequeue(&mut self) -> Option<Signal> {
        self.signals.pop_front()
    }

    /// Returns the number of signals in the queue.
    /// # Example
    /// ```
    /// use dtmf::{Message, Signal};
    ///
    /// let mut message = Message::default();
    /// message.enqueue(Signal::A);
    /// message.enqueue(Signal::B);
    ///
    /// assert_eq!(message.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.signals.len()
    }

    /// Returns an inmutable iterator over the signals.
    pub fn iter(&self) -> SignalIterator {
        self.signals.iter()
    }
}

impl Default for Message {
    fn default() -> Message {
        Message {
            signals: VecDeque::new(),
            signal_duration: 0.7,
            silence_duration: 0.3,
        }
    }
}

impl FromStr for Message {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = Message::default();
        for char in s.chars() {
            match Signal::from_char(char) {
                Ok(signal) => result.enqueue(signal),
                Err(SignalParsingError::UnknownSignal(unknown)) => return Err(unknown),
                _ => panic!("Char separation failed"),
            };
        }
        Ok(result)
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut Formatter) -> FormatResult {
        for signal in self.iter() {
            write!(f, "{}", signal)?;
        }
        Ok(())
    }
}
