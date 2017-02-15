use ::Message;
use ::encoder::MessageEncoder;

use super::decode_signal;

/// Decodes a message.
/// # Example
/// ```
/// use ::dtmf::{Message, Signal};
/// use ::dtmf::encoder::MessageEncoder;
/// use ::dtmf::decoder::decode_message;
///
/// let mut message = Message::default();
/// message.enqueue(Signal::A);
/// message.enqueue(Signal::B);
///
/// let mut target_message = Message::default();
///
/// let data = MessageEncoder::new(&message, 48000.);
/// decode_message(&mut target_message, &data, 48000.);
///
/// assert_eq!(message, target_message);
/// ```
pub fn decode_message(message: &mut Message, encoded_message: &MessageEncoder, sample_rate: f64) {

    // let mut signals = VecDeque::new();
    let length = encoded_message.clone().count();
    let signal_duration = (message.signal_duration() * sample_rate) as usize;
    let silence_duration = (message.silence_duration() * sample_rate) as usize;

    // split audio-message into audio-signals on pauses
    // first signal_duration
    if length >= signal_duration {
        let signal = decode_signal((&(encoded_message.clone()
                                       .take(signal_duration)
                                       .map(|x| x[0])
                                       .collect::<Vec<f64>>())),
                                   sample_rate);

        message.enqueue(signal);
    }

    // other signals
    let mut index = signal_duration;
    while (index + signal_duration + silence_duration) <= length {
        let signal = decode_signal((&(encoded_message.clone()
                                       .take(signal_duration)
                                       .map(|x| x[0])
                                       .collect::<Vec<f64>>())),
                                   sample_rate);

        message.enqueue(signal);
        index += signal_duration + silence_duration;
    }
}
