use std::f64::consts::PI;

struct SVF {
    ic1eq: f64,
    ic2eq: f64,
    g: f64,
    k: f64,
    a1: f64,
    a2: f64,
    a3: f64,
}

impl SVF {
    pub fn new(sample_rate: u64, cutoff: f64, resonance: f64) -> Self {
        let g = f64::tan(PI * sample_rate as f64 / cutoff);
        let k = 2. - 2. * resonance;
        let a1 = 1. / (1. + g * (g + k));
        let a2 = g * a1;
        let a3 = g * a2;
        SVF {
            ic1eq: 0.,
            ic2eq: 0.,
            g,
            k,
            a1,
            a2,
            a3,
        }
    }

    pub fn clear(&mut self) {
        self.ic1eq = 0.;
        self.ic2eq = 0.;
    }

    fn process_sample(&mut self, v0: f64) -> f64 {
        let v3 = v0 - self.ic2eq;
        let v1 = self.a1 * self.ic1eq + self.a3 * v3;
        let v2 = self.ic2eq + self.a2 * self.ic1eq + self.a3 * v3;
        self.ic1eq = 2. * v1 - self.ic1eq;
        self.ic2eq = 2. * v2 - self.ic2eq;
        v2
    }

    pub fn process_block(&mut self, samples: &mut Vec<f64>) {
        for sample in samples.iter_mut() {
            self.process_sample(*sample);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_bare() {}

    #[test]
    fn test_filter_res() {}

    #[test]
    fn test_clear() {}

    #[test]
    fn test_process_sample() {}

    #[test]
    fn test_process_block() {}
}
