use super::goertzel;
use ::dtmf::signal::Signal;



const RATE: f64 = 44100;

pub struct MessageMaker {
    message: Vec<Signal>,
}

impl MessageMaker {
    pub fn new(audio: XXX) -> MessageMaker {
        let mut message = String::new();
        //split audio-message into audio-signals on pauses

        //DO this for every audiosignal:
        //convert every audio-signal into samples
        //let signal = Goertzel_DTMF::new(samples, RATE).find_signal();
        //if let Some(x) = signal {
            //message.push(x);
        //}

        MessageMaker{
            message: message,
        }
    }

    fn audio_to_samples(audio: XXX) -> &Vec<f64> {
        //TODO: sample audio with RATE
    }
}
