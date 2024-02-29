use nalgebra::{DMatrix, DVector};

pub trait Round {
    // In-place rounding
    fn round_(&mut self, decimal_places: usize);

    // Out-of-place rounding, returns a new instance
    fn round(&self, decimal_places: usize) -> Self;
}

impl Round for DMatrix<f64> {
    fn round_(&mut self, decimal_places: usize) {
        let factor = 10f64.powi(decimal_places as i32);
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                self[(i, j)] = (self[(i, j)] * factor).round() / factor;
            }
        }
    }

    fn round(&self, decimal_places: usize) -> Self {
        let mut result = self.clone();
        result.round_(decimal_places);
        result
    }
}

impl Round for DVector<f64> {
    fn round_(&mut self, decimal_places: usize) {
        let factor = 10f64.powi(decimal_places as i32);
        for i in 0..self.len() {
            self[i] = (self[i] * factor).round() / factor;
        }
    }

    fn round(&self, decimal_places: usize) -> Self {
        let mut result = self.clone();
        result.round_(decimal_places);
        result
    }
}
