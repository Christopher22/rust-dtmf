
extern crate goertzel_filter;
use self::goertzel_filter::dft_power;
use ::dtmf::signal::Signal;
use std;

pub struct Goertzel_DTMF {
    higher_f: f64,
    lower_f: f64,
    pub signal: Signal,
}

impl Goertzel_DTMF {
    pub fn new(samples: &Vec<f64>) -> Result<Goertzel_DTMF, &'static str> {
        let &lower_f = [697.,770., 852., 941.].iter()
                                        .max_by_key(|x| (dft_power(samples, **x)*10000000000000.0).round() as i64)
                                        .unwrap();
        println!("Goertzel 697: {}", dft_power(samples, 697.));
        println!("Goertzel 770: {}", dft_power(samples, 770.));

        println!("Goertzel 1209: {}", dft_power(samples, 1209.));
        println!("Goertzel 1477: {}", dft_power(samples, 1477.));
        println!("Goertzel 1209 - rounded: {}", dft_power(samples, 1209.).round() as i64);
        println!("Goertzel 1633 - rounded: {}", dft_power(samples, 1633.).round() as i64);
        println!("Goertzel 1336 - rounded: {}", dft_power(samples, 1336.).round() as i64);
        
        let &higher_f = [1209., 1336., 1477., 1633.].into_iter()
                                        .max_by_key(|x| (dft_power(samples, **x)*10000000000000.0).round() as i64)
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

pub fn goertzel_filter(samples: &Vec<f64>, sample_rate: f64){
    let len: i64 = samples.len() as i64;
    let step: f64 = sample_rate / (len as f64);
    let step_normalized = 1.0 / (len as f64);

    //lower frequencies
    let mut bins = Vec::new();
    for i in [697,770, 852, 941].iter() {
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
}
