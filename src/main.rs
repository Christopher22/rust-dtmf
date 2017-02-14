extern crate dtmf;
extern crate hound;
extern crate sample;

use dtmf::Message;

fn encode(message: Message) -> bool {
    use dtmf::encoder::MessageEncoder;
    use hound::{WavWriter, WavSpec};

    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };

    if let Ok(mut writer) = WavWriter::create("dtmf.wav", spec) {
        for sample in MessageEncoder::new(&message, 44_100.0)
            .map(|s| sample::conv::f64::to_i32(s[0])) {

            if writer.write_sample(sample).is_err() {
                return false;
            }
        }
        true
    } else {
        false
    }
}

// // test without audio-file, just with messageEncoder object
// fn test_method(message: Message) -> Result<Message, &'static str> {
// use dtmf::decoder::message_decoder::MessageMaker;
// use dtmf::encoder::MessageEncoder;
//
// match MessageMaker::new(MessageEncoder::new(&message, 44_100.0), 44100.0) {
// Ok(x) => Ok(x.message),
// Err(e) => Err(e),
// }
// }

fn main() {}
