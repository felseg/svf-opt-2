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

    pub fn clear_state(&mut self) {
        self.ic1eq = 0.;
        self.ic2eq = 0.;
    }

    pub fn process_sample(&mut self) -> f64 {
        !unimplemented!()
    }

    pub fn process_block(&mut self, samples: Vec<f64>) -> Vec<f64> {
        !unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
