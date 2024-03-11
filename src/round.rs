use nalgebra::{DMatrix, DVector};

pub trait Round {
    // In-place element-wise rounding
    fn round_(&mut self, decimal_places: usize);

    // Out-of-place element-wise rounding, returns a new instance
    fn round(&self, decimal_places: usize) -> Self;
}

impl Round for DVector<f64> {
    /// # Example
    /// ```
    ///let mut v = DVector::from_vec(vec![123.456, 78.9, 0.12345]);
    ///v.round_(2);
    ///assert_eq!(v, DVector::from_vec(vec![123.46, 78.90, 0.12]));
    /// ```
    #[inline]
    fn round_(&mut self, decimal_places: usize) {
        let factor = 10f64.powi(decimal_places as i32);
        for i in 0..self.len() {
            self[i] = (self[i] * factor).round() / factor;
        }
    }
    /// # Example
    /// ```
    ///let mut v = DVector::from_vec(vec![123.456, 78.9, 0.12345]);
    ///let rounded = v.round(1);
    /// assert_eq!(rounded, DVector::from_vec(vec![123.5, 78.9, 0.1]));
    /// ```
    #[inline]
    fn round(&self, decimal_places: usize) -> Self {
        let mut result = self.clone();
        result.round_(decimal_places);
        result
    }
}

impl Round for DMatrix<f64> {
    /// # Example
    /// ```
    /// let mut m = DMatrix::from_row_slice(2, 2, &[123.456, 78.9, 0.12345, 456.789]);
    /// m.round_(3);
    /// assert_eq!(m, DMatrix::from_row_slice(2, 2, &[123.456, 78.900, 0.123, 456.789]));
    /// ```
    #[inline]
    fn round_(&mut self, decimal_places: usize) {
        let factor = 10f64.powi(decimal_places as i32);
        for i in 0..self.nrows() {
            for j in 0..self.ncols() {
                self[(i, j)] = (self[(i, j)] * factor).round() / factor;
            }
        }
    }
    /// # Example
    /// ```
    /// let mut m = DMatrix::from_row_slice(2, 2, &[123.456, 78.9, 0.12345, 456.789]);
    /// let rounded = m.round(1);
    /// assert_eq!(rounded, DMatrix::from_row_slice(2, 2, &[123.5, 78.9, 0.1, 456.8]));
    /// ```
    #[inline]
    fn round(&self, decimal_places: usize) -> Self {
        let mut result = self.clone();
        result.round_(decimal_places);
        result
    }
}
