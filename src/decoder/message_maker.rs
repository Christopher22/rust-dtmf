use super::goertzel::Goertzel_DTMF;
use ::dtmf::signal::Signal;
use ::dtmf::message::Message;
use ::encoder::message_encoder::MessageEncoder;
use std::collections::VecDeque;




pub struct MessageMaker {
    pub message: Message,
}

impl MessageMaker {
    pub fn new(encoded_message: MessageEncoder) -> MessageMaker {
        let mut signals = VecDeque::new();
        let length = encoded_message.iter().count();
        let signal_duration = encoded_message.signal_duration;
        let silence_duration = encoded_message.silence_length;
        //split audio-message into audio-signals on pauses
        //first signal_duration
        if length >= signal_duration {
            let signal = Goertzel_DTMF::new(encoded_message.iter().take(signal_duration)).find_signal();
            if let Some(x) = signal {
                signals.push(x);
            }
        }

        //other signals
        let mut index = signal_duration;
        while (index + signal_duration + silence_duration) < length {
            
            let signal = Goertzel_DTMF::new(encoded_message.iter()
                                                            .skip(index+silence_duration)
                                                            .take(signal_duration)).find_signal();
            if let Some(x) = signal {
                signals.push(x);
            }
            index = index + signal_duration + silence_duration;
        }

        MessageMaker {
            message: Message {
                signals: signals,
                signal_duration: 0.7,
                silence_duration: 0.3,
            }
        }
    }
}
