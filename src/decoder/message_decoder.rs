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
/// let data = MessageEncoder::new(&message, 48.000);
/// decode_message(&mut target_message, &data, 48.000);
///
/// assert_eq!(message, target_message);
/// ```
pub fn decode_message(message: &mut Message,
                      encoded_message: &MessageEncoder,
                      sample_rate: f64)
                      -> bool {

    // let mut signals = VecDeque::new();
    let length = encoded_message.clone().count();
    let signal_duration = (message.signal_duration() * sample_rate) as usize;
    let silence_duration = (message.silence_duration() * sample_rate) as usize;

    // split audio-message into audio-signals on pauses
    // first signal_duration
    if length >= signal_duration {
        match decode_signal((&(encoded_message.clone()
                                .take(signal_duration)
                                .map(|x| x[0])
                                .collect::<Vec<f64>>())),
                            sample_rate) {
            Some(x) => message.enqueue(x),
            None => return false,
        };
    }

    // other signals
    let mut index = signal_duration;
    while (index + signal_duration + silence_duration) <= length {
        match decode_signal((&(encoded_message.clone()
                                .take(signal_duration)
                                .map(|x| x[0])
                                .collect::<Vec<f64>>())),
                            sample_rate) {
            Some(x) => message.enqueue(x),
            None => return false,
        };
        index += signal_duration + silence_duration;
    }

    true
}
