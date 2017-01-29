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

fn main() {
    // Get arguments
    let args: Vec<_> = ::std::env::args().skip(1).collect();

    match args.len() {
        0 => unimplemented!(),
        1 => {
            let success = match args.first().unwrap().parse() {
                Ok(message) => encode(message),
                Err(_) => {
                    println!("[ERROR] Invalid message");
                    return;
                }
            };

            if !success {
                println!("[ERROR] Error during writing the file.")
            }
        }
        _ => println!("[ERROR] Please specify an argument."),
    }
}
