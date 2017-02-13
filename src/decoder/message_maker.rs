use super::goertzel::Goertzel_DTMF;
use ::dtmf::signal::Signal;
use ::dtmf::message::Message;
use ::encoder::message_encoder::MessageEncoder;
use std::collections::VecDeque;




pub struct MessageMaker {
    pub message: Message,
}

impl MessageMaker {
    pub fn new(encoded_message: MessageEncoder) -> Result<MessageMaker, &'static str> {
        let mut signals = VecDeque::new();
        let length = encoded_message.clone().count();
        let signal_duration = encoded_message.clone().signal_duration;
        let silence_duration = encoded_message.clone().silence_length;
        //split audio-message into audio-signals on pauses
        //first signal_duration
        if length >= signal_duration {
            match Goertzel_DTMF::new((&(encoded_message.clone().take(signal_duration).map(|x| x[0]).collect::<Vec<f64>>()))) {
                Ok(x) => {
                    signals.push_back(x.signal);
                }
                Err(e) => return Err(e),
            };
        }

        //other signals
        let mut index = signal_duration;
        while (index + signal_duration + silence_duration) < length {
            println!("zweite runde");
            match Goertzel_DTMF::new((&(encoded_message.clone().skip(index+silence_duration)
                                                            .take(signal_duration).map(|x| x[0]).collect::<Vec<f64>>()))) {
                Ok(x) => {
                    signals.push_back(x.signal);
                }
                Err(e) => return Err(e),
            };
            index = index + signal_duration + silence_duration;
        }

        Ok(MessageMaker {
            message: Message {
                signals: signals,
                signal_duration: 0.7,
                silence_duration: 0.3,
            }
        })
    }
}
