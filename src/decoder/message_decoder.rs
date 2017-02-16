use Message;

use super::decode_signal;

/// Decodes a message.
/// # Example
/// ```
/// use dtmf::{Message, Signal};
/// use dtmf::encoder::MessageEncoder;
/// use dtmf::decoder::decode_message;
///
/// let mut message = Message::default();
/// message.enqueue(Signal::A);
/// message.enqueue(Signal::B);
///
/// let mut target_message = Message::default();
///
/// let data = MessageEncoder::new(&message, 48000.);
/// decode_message(data.map(|x| x[0]), &mut target_message, 48000.);
///
/// assert_eq!(message, target_message);
/// ```
pub fn decode_message<S>(sample_iter: S, message: &mut Message, sample_rate: f64)
    where S: IntoIterator<Item = f64>,
          S::IntoIter: ExactSizeIterator
{
    let mut samples = sample_iter.into_iter();
    let mut length = samples.len();

    let signal_duration = (message.signal_duration() * sample_rate) as usize;
    let silence_duration = (message.silence_duration() * sample_rate) as usize;

    let mut first_signal = true;
    while length > 0 {

        // Create a stream of samples
        let samples = samples.by_ref()
            .skip(match first_signal {
                true => {
                    first_signal = false;
                    0
                }
                false => {
                    length -= silence_duration;
                    silence_duration
                }
            })
            .take(signal_duration);

        // Decode each signal
        let signal = decode_signal(samples, sample_rate);

        // Add the signal
        message.enqueue(signal);
        length -= signal_duration;
    }
}
