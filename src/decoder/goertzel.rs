
extern crate goertzel_filter;
use self::goertzel_filter::dft_power;
use ::dtmf::signal::Signal;

pub struct Goertzel_DTMF {
    higher_f: f64,
    lower_f: f64,
    pub signal: Signal,
}

impl Goertzel_DTMF {
    pub fn new(samples: &Vec<f64>) -> Result<Goertzel_DTMF, &'static str> {
        let &lower_f = [697.,770., 852., 941.].iter()
                                        .max_by_key(|x| dft_power(samples, **x).round() as i64)
                                        .unwrap();
        println!("Goertzel 697: {}", dft_power(samples, 697.));
        println!("Goertzel 941: {}", dft_power(samples, 941.));

        println!("Goertzel 1209: {}", dft_power(samples, 1209.));
        println!("Goertzel 1633: {}", dft_power(samples, 1633.));
        
        let &higher_f = [1209., 1336., 1477., 1633.].into_iter()
                                        .max_by_key(|x| dft_power(samples, **x).round() as i64)
                                        .unwrap();

        if !find_signal(lower_f, higher_f).is_some() {
            Err("Problem with Goertzel")
        } else {
            Ok(Goertzel_DTMF{
                higher_f: higher_f.sqrt(),
                lower_f: lower_f.sqrt(),
                signal: find_signal(lower_f, higher_f).unwrap(),
            }) 
        } 
    }
}

 fn find_signal(lower_f: f64, higher_f: f64) -> Option<Signal> {
        println!("Frequenzen: {}, {}", lower_f, higher_f);
        match (lower_f, higher_f) {
            (941., 1336.) => Some(Signal::Digit(0)),
            (697., 1209.) => Some(Signal::Digit(1)),
            (697., 1336.) => Some(Signal::Digit(2)),
            (697., 1477.) => Some(Signal::Digit(3)),
            (770., 1209.) => Some(Signal::Digit(4)),
            (770., 1336.) => Some(Signal::Digit(5)),
            (770., 1477.) => Some(Signal::Digit(6)),
            (852., 1209.) => Some(Signal::Digit(7)),
            (852., 1336.) => Some(Signal::Digit(8)),
            (852., 1477.) => Some(Signal::Digit(9)),
            // Valid letters
            (697., 1633.) => Some(Signal::A),
            (770., 1633.) => Some(Signal::B),
            (852., 1633.) => Some(Signal::C),
            (941., 1633.) => Some(Signal::D),
            // Other symbols
            (941., 1209.) => Some(Signal::Asterisk),
            (941., 1477.) => Some(Signal::Hash),
            // Invalid frequencies
            _ => None,
        }
    }
