extern crate dtmf;
extern crate hound;
extern crate sample;

#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};

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
    let parser = app_from_crate!()
        .arg(Arg::with_name("file")
            .help("The wav file which is used to be en- or decoded.")
            .value_name("FILE")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("signal")
            .help("The duration of a single signal in seconds.")
            .takes_value(true)
            .default_value("0.7")
            .validator(|input| {
                input.parse::<f32>()
                    .or_else(|_| Err(String::from("Invalid floating point.")))
                    .and_then(|rate| Ok(()))
            }))
        .arg(Arg::with_name("silence")
            .help("The duration of the silence between the signals in seconds.")
            .takes_value(true)
            .default_value("0.3")
            .validator(|input| {
                input.parse::<f32>()
                    .or_else(|_| Err(String::from("Invalid floating point.")))
                    .and_then(|rate| Ok(()))
            }))
        .subcommand(SubCommand::with_name("encode")
            .about("Encodes an message which was read from STDIN into a file")
            .arg(Arg::with_name("sample_rate")
                .help("The sample rate of the message in the range of 8 kHz - 92 kHz.")
                .default_value("44100")
                .validator(|input| {
                    input.parse::<u32>()
                        .or_else(|_| Err(String::from("Invalid number for sample rate")))
                        .and_then(|rate| {
                            match rate >= 8000 && rate <= 92000 {
                                true => Ok(()),
                                false => Err(String::from("Invalid range for the rate")),
                            }
                        })
                })
                .takes_value(true)))
        .subcommand(SubCommand::with_name("decode")
            .about("Decodes an message from a file and print it to STDOUT"));

    match parser.get_matches().subcommand() {
        ("encode", Some(encode_parser)) => {}
        ("decode", Some(decode_parser)) => {}
        _ => {
            println!("[ERROR] Please specify a subcommand or use 'help' for further assistance!");
        }
    }
}
