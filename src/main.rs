extern crate dtmf;
extern crate hound;
extern crate sample;

#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};

use std::path::Path;
use dtmf::Message;

/// Encodes a message into a file.
fn encode<P: AsRef<Path>>(file: P, message: Message, sample_rate: u32) -> bool {
    use dtmf::encoder::MessageEncoder;
    use hound::{WavWriter, WavSpec};

    // Create metadata for the wav file
    let spec = WavSpec {
        channels: 1,
        sample_rate: sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Int,
    };

    // Try to create the file
    match WavWriter::create(&file, spec) {
        Ok(mut writer) => {
            // Write all the samples
            for sample in MessageEncoder::new(&message, sample_rate as f64)
                .map(|s| sample::conv::f64::to_i32(s[0])) {
                if writer.write_sample(sample).is_err() {
                    return false;
                }
            }
            true
        }
        _ => false,
    }
}

/// Decodes a message for a file.
fn decode<P: AsRef<Path>>(file: P, message: &mut Message) -> bool {
    use hound::WavReader;

    // Try to open the file
    match WavReader::open(file) {
        Ok(reader) => {
            // TODO, when decoder is ready
            reader.into_samples::<i32>().map(|s| match s {
                Ok(sample) => sample::conv::i32::to_f64(sample),
                Err(_) => 0.,
            });
            true
        }
        Err(_) => false,
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
                input.parse::<f64>()
                    .or_else(|_| Err(String::from("Invalid floating point.")))
                    .and_then(|rate| Ok(()))
            }))
        .arg(Arg::with_name("silence")
            .help("The duration of the silence between the signals in seconds.")
            .takes_value(true)
            .default_value("0.3")
            .validator(|input| {
                input.parse::<f64>()
                    .or_else(|_| Err(String::from("Invalid floating point.")))
                    .and_then(|_| Ok(()))
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
            .about("Decodes an message from a file and print it to STDOUT"))
        .get_matches();

    match parser.subcommand() {

        // The encode subcommand
        ("encode", Some(encode_parser)) => {
            // Read the input from STDIN
            let mut input = String::new();
            if let Err(_) = ::std::io::stdin().read_line(&mut input) {
                println!("[ERROR] Accessing STDIN failed!");
                return;
            }

            // Parse the input into a message
            let mut message = match input.trim().parse::<Message>() {
                Ok(message) => message,
                Err(_) => {
                    println!("[ERROR] Invalid message!");
                    return;
                }
            };

            // Set the parameter
            message.set_signal_duration(value_t!(parser, "signal", f64).expect("Invalid value"));
            message.set_silence_duration(value_t!(parser, "silence", f64).expect("Invalid value"));

            // Try to encode the message
            if !encode(Path::new(parser.value_of("file").expect("Valid file")),
                       message,
                       value_t!(encode_parser, "sample_rate", u32).expect("Invalid value")) {
                println!("[ERROR] Writing the file failed. Do you have sufficient rights?")
            }
        }
        ("decode", Some(decode_parser)) => {}
        _ => {
            println!("[ERROR] Please specify a subcommand or use 'help' for further assistance!");
        }
    }
}
