extern crate sample;

mod dtmf;
pub mod decoder;
pub mod encoder;

// Export the important structs directly into the lib root.
pub use self::dtmf::{Signal, SignalIterator, SignalParsingError, Message};