
use ::goertzel_filter::dft_power;
use ::dtmf::signal::Signal;

pub struct Goertzel_DTMF {
    higher_f: f64,
    lower_f: f64,
    signal: Signal,
}

impl Goertzel_DTMF {
    pub fn new(samples: &Vec<f64>, rate) -> Goertzel_DTMF {
        let mut result_higher: HashMap<f64, f64> = HashMap::new();
        let mut result_lower: HashMap<f64, f64> = HashMap::new();
        let lower_f = [697,770, 852, 941].iter()
                                        .max_by_key(|x| dft_power(x))
                                        .unwrap();
        
        let higher_f = [1209, 1336, 1477, 1633].iter()
                                        .max_by_key(|x| dft_power(x))
                                        .unwrap();

        if !find_signal().is_some {println("Problem with Goertzel");}

        Goertzel_DTMF{
            higher_f: higher_f,
            lower_f: lower_f,
            signal: find_signal();
        }  
    }

    fn find_signal() -> Option<Signal> {
        match (lower_f, higher_f):
            (941, 1336) => Some(Signal::Digit(0)),
            (697, 1209) => Some(Signal::Digit(1)),
            (697, 1336) => Some(Signal::Digit(2)),
            (697, 1477) => Some(Signal::Digit(3)),
            (770, 1209) => Some(Signal::Digit(4)),
            (770, 1336) => Some(Signal::Digit(5)),
            (770, 1477) => Some(Signal::Digit(6)),
            (852, 1209) => Some(Signal::Digit(7)),
            (852, 1336) => Some(Signal::Digit(8)),
            (852, 1477) => Some(Signal::Digit(9)),
            // Valid letters
            (697, 1633) => Some(Signal::A),
            (770, 1633) => Some(Signal::B),
            (852, 1633) => Some(Signal::C),
            (941, 1633) => Some(Signal::D),
            // Other symbols
            (941, 1209) => Some(Signal::Asterisk),
            (941, 1477) => Some(Signal::Hash),
            // Invalid frequencies
            _ => None,
    }
}
