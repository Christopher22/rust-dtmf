use std::cmp::Ordering;
use std::collections::HashSet;

use Signal;

/// Decodes a signal from a stream of samples.
/// # Example
/// ```
/// use ::dtmf::encoder::SignalEncoder;
/// use ::dtmf::decoder::decode_signal;
/// use ::dtmf::Signal;
///
/// for &signal in Signal::iter() {
///     let data = SignalEncoder::new(signal, 48000.).unwrap().take(12000).map(|x| x[0]).collect::<Vec<f64>>();
///     assert_eq!(decode_signal(data, 48000.), signal);
/// }
/// ```
pub fn decode_signal<T>(samples: T, sample_rate: f64) -> Signal
    where T: IntoIterator<Item = f64>,
          T::IntoIter: ExactSizeIterator
{
    // Separate higher and lower frequencies
    let low_freq: HashSet<u16> = [697, 770, 852, 941].iter().cloned().collect();
    let high_freq: HashSet<u16> = [1209, 1336, 1477, 1633].iter().cloned().collect();

    // Apply the goerzel algorithm to all frequencies
    let mut bins = GoertzelBin::apply_goerzel(samples.into_iter(),
                                              sample_rate,
                                              low_freq.union(&high_freq).cloned());

    // Sorts the bins by their power
    bins.sort();

    // Find the high frequence with the most power
    let low_freq = bins.iter()
        .rev()
        .map(|bin| bin.frequency())
        .find(|freq| low_freq.contains(freq))
        .expect("Missing lower frequency");

    // Find the high frequence with the most power
    let high_freq = bins.iter()
        .rev()
        .map(|bin| bin.frequency())
        .find(|freq| high_freq.contains(freq))
        .expect("Missing higher frequency");

    Signal::from_frequencies((low_freq, high_freq)).expect("Valid frequencies")
}

/// An bin for the goertzel algorithm which could be sorted by its power.
struct GoertzelBin {
    real: f64,
    coeff: (f64, f64),
    freq: u16,
}

impl GoertzelBin {
    /// Creates a new bin for a specific frequency.
    pub fn new(freq: u16, step: f64, len: usize) -> GoertzelBin {
        let step_normalized = 1.0 / len as f64;
        let f = (freq as f64 / step) * step_normalized;

        GoertzelBin {
            real: 2.0 * (2.0 * ::std::f64::consts::PI * f).cos(),
            coeff: (0., 0.),
            freq: freq,
        }
    }

    /// Applies the Goertzel algorithm on an slice of frequencies and returns the bins.
    pub fn apply_goerzel<S, F>(samples: S, sample_rate: f64, freqs: F) -> Vec<GoertzelBin>
        where S: Iterator<Item = f64> + ExactSizeIterator,
              F: Iterator<Item = u16>
    {

        let len = samples.len();
        let step = sample_rate / len as f64;

        // Create the GoertzelBins from the frequencies
        // TODO: freq > (len as f64) - 1f64?!
        let mut bins: Vec<_> = freqs.map(|freq| GoertzelBin::new(freq, step, len)).collect();

        // Fill them with the samples
        for sample in samples {
            for bin in bins.iter_mut() {
                bin.add_sample(sample)
            }
        }

        bins
    }

    /// Adds an sample to the bin.
    pub fn add_sample(&mut self, sample: f64) {
        self.coeff = (sample + self.real * self.coeff.0 - self.coeff.1, self.coeff.0);
    }

    /// Calculates the current power of the bin
    pub fn calculate(&self) -> f64 {
        self.coeff.1.powi(2) + self.coeff.0.powi(2) - self.real * self.coeff.0 * self.coeff.1
    }

    /// Returns the frequency of the bin.
    pub fn frequency(&self) -> u16 {
        self.freq
    }
}

impl Ord for GoertzelBin {
    fn cmp(&self, other: &GoertzelBin) -> Ordering {
        self.calculate().partial_cmp(&other.calculate()).expect("Non NaN")
    }
}

impl Eq for GoertzelBin {}

impl PartialOrd for GoertzelBin {
    fn partial_cmp(&self, other: &GoertzelBin) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for GoertzelBin {
    fn eq(&self, other: &GoertzelBin) -> bool {
        self.calculate() == other.calculate()
    }
}
