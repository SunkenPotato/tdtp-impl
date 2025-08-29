use statrs::distribution::{ContinuousCDF, Normal};
use std::f64;

pub struct BaselineBinner {
    baseline: Vec<f64>,
    num_bits: usize,
}

impl BaselineBinner {
    /// Erstellt eine neue Instanz mit gegebener Baseline und gewünschter Bit-Anzahl.
    pub fn new(baseline: Vec<f64>, num_bits: usize) -> Self {
        Self { baseline, num_bits }
    }

    /// Gibt den Bin als Bits zurück, in dem der Wert `x` in einer Normalverteilung aus der Baseline wäre.
    pub fn bin_as_bits(&self, x: f64) -> Option<Vec<bool>> {
        if self.baseline.is_empty() || self.num_bits == 0 || self.num_bits > usize::BITS as usize {
            return None;
        }
        let mean = self.baseline.iter().copied().sum::<f64>() / self.baseline.len() as f64;
        let stddev = (self
            .baseline
            .iter()
            .map(|v| (v - mean).powi(2))
            .sum::<f64>()
            / self.baseline.len() as f64)
            .sqrt();
        let normal = Normal::new(mean, stddev).ok()?;

        // Berechne die Wahrscheinlichkeit für x
        let p = normal.cdf(x);

        // Anzahl der Bins = 2^num_bits
        let bins = 1 << self.num_bits;
        let bin = ((p * bins as f64).floor() as usize).min(bins - 1);

        // Bin als Bits (big endian, mit führenden Nullen)
        let bits = (0..self.num_bits)
            .rev()
            .map(|i| ((bin >> i) & 1) == 1)
            .collect::<Vec<_>>();

        Some(bits)
    }
}
