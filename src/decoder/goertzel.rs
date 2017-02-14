
extern crate goertzel_filter;
use self::goertzel_filter::dft_power;
use ::dtmf::signal::Signal;
use std;

pub struct Goertzel_DTMF {
    pub signal: Signal,
}

impl Goertzel_DTMF {
    ///creates Goertzel_DTMF which holds the signal which fits the samples
    ///needs samples and sample_rate
    pub fn new(samples: &Vec<f64>, sample_rate: f64) -> Result<Goertzel_DTMF, &'static str> {
        let signal = match find_signal(goertzel_filter(samples, sample_rate, &[697, 770, 852, 941]),
                                      goertzel_filter(samples, sample_rate, &[1209, 1336, 1477, 1633])) {
            Some(x) => x,
            None => return Err("Problem with Signal matching"),
        };

        Ok(Goertzel_DTMF {
            signal: signal,
        })
    }
}
///matcht 2 frequencies on the fitting signal
 fn find_signal(lower_f: i32, higher_f: i32) -> Option<Signal> {
        println!("Frequenzen: {}, {}", lower_f, higher_f);
        match (lower_f, higher_f) {
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
///examines frequency which has most power in samples
///needs samples, sample_rate and the frequencies which are to be tested
pub fn goertzel_filter(samples: &Vec<f64>, sample_rate: f64, dtmf_freq: &[i32]) -> i32{
    let len: i64 = samples.len() as i64;
    let step: f64 = sample_rate / (len as f64);
    let step_normalized = 1.0 / (len as f64);

    //make bins
    let mut bins = Vec::new();
    for i in dtmf_freq.iter() {
        let mut freq = (*i as f64) / step;
        if freq > (len as f64) -1f64 {println!("Frequency out of range {}", i);}
        bins.push(freq.clone());
    }
    let n_range: Vec<i64> = (0..len).collect();
    let mut freqs = Vec::new();
    let mut results = Vec::new();
    for k in bins {
        //bin frequency and coefficients for computation
        let f = k * step_normalized;
        let real = 2.0 * (2.0 * std::f64::consts::PI * f).cos();
        let imag = (2.0 * std::f64::consts::PI * f).sin();

        let mut coeff1 = 0.0;
        let mut coeff2 = 0.0;
        //doing calculation on all samples
        for n in &n_range {
            let y = samples[*n as usize] + real * coeff1 - coeff2;
            coeff2 = coeff1;
            coeff1 = y;
        }
        //storing results
        results.push(coeff2.powi(2) + coeff1.powi(2) - real * coeff1 * coeff2);
        freqs.push(f*sample_rate);
    }
    println!("Freq: {:?}", freqs);
    println!("Results: {:?}", results);
    //comparing results, find frequency
    //freqs[results.iter().enumerate().max().0] 
    let mut index = 0;
    for (j, &value) in results.iter().enumerate() {
        if value > results[index] {
            index = j;
        }
    }
    (freqs[index].round() as i32)

}
