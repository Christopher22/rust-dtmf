extern crate dtmf;
extern crate hound;
extern crate sample;

use sample::Sample;
use dtmf::Message;

fn encode(message: Message) {
    use dtmf::encoder::MessageEncoder;

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };

    let encoder = MessageEncoder::new(&message, 44_100.0).map(|s| sample::conv::f64::to_i32(s[0]));
    //use ::sample::signal::{rate, Sine, AddAmp, ConstHz, Signal};
    //let encoder = rate(44_100.0).const_hz(852.0).sine().scale_amp(0.5).take(44_100).map(|s| sample::conv::f64::to_i32(s[0]));

    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for sample in encoder {
        writer.write_sample(sample);
    }
}

fn main() {

    let args: Vec<_> = ::std::env::args().skip(1).collect();
    match args.len() {
        0 => unimplemented!(),
        1 => {
            match args.first().unwrap().parse() {
                Ok(message) => encode(message),
                Err(_) => println!("[ERROR] Invalid message"),
            };
        }
        _ => println!("[ERROR] Please insert an argument."),
    }
}
