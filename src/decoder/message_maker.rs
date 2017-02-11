use super::goertzel;
use ::dtmf::signal::Signal;
use ::dtmf::message::Message;
use ::encoder::message_encoder::MessageEncoder;



const RATE: f64 = 44100;

pub struct MessageMaker {
    message: Message,
}

impl MessageMaker {
    pub fn new(encoded_message: MessageEncoder) -> MessageMaker {
        let mut signals = VecDeque::new();
        let length = encoded_message.iter().count();
        let signal_duration = encoded_message.signal_duration;
        let silence_duration = encoded_message.silence_length;
        let sample_rate = encoded_message.sample_rate;
        //split audio-message into audio-signals on pauses
        //first signal_duration
        if (length >= signal_duration) {
            let signal = Goertzel_DTMF::new(encoded_message.iter().take(signal_duration), sample_rate).find_signal();
            if let Some(x) = signal {
                signals.push(x);
            }
        }
        //other signals
        for (i=signal_duration; (i + signal_duration + silence_duration) < length; i + signal_duration + silence_duration){
            
            let signal = Goertzel_DTMF::new(encoded_message.iter()
                                                            .skip(i+silence_duration)
                                                            .take(signal_duration), sample_rate).find_signal();
            if let Some(x) = signal {
                signals.push(x);
            }
        }

        MessageMaker {
            message: Message {
                signals: signals,
                signal_duration: signal_duration/ sample_rate,
                silence_duration: silence_duration / sample_rate,
            }
        }
    }
}
