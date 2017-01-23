mod dtmf;
mod decoder;
mod encoder;

// Export the important structs directly into the lib root.
pub use self::dtmf::{Signal, SignalIterator, SignalParsingError, Message};