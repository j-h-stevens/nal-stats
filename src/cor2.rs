use nalgebra::{DMatrix, DVector};

pub trait Cor2cov {
    // In-place cor2cov
    fn cor2cov_(&mut self, s: &DVector<f64>);

    // Out-of-place rounding, returns a new instance
    fn cor2cov(&self, s: &DVector<f64>) -> Self;
}

impl Cor2cov for DMatrix<f64> {
    ///Mutable implementation
    fn cor2cov_(&mut self, s: &DVector<f64>) {
        let n = self.shape();
        if n.0 != s.len() || n.1 != s.len() {
            panic!("inconsistent dimensions");
        }

        for i in 0..n.0 {
            for j in 0..n.1 {
                self[(i, j)] *= s[i] * s[j];
            }
        }
    }
    ///Copy implementation
    fn cor2cov(&self, s: &DVector<f64>) -> Self {
        let mut result = self.clone();
        result.cor2cov_(s);
        result
    }
}
