use nalgebra::{Cholesky, DMatrix, DVector};

pub trait Cor2 {
    // In-place cor2cov
    fn cor2cov_(&mut self, s: &DVector<f64>);

    // Out-of-place rounding, returns a new instance
    fn cor2cov(&self, s: &DVector<f64>) -> Self;

    // Calculate upper triangular Cholesky decomposition
    fn cor2chol_u(&self, s: &DVector<f64>) -> DMatrix<f64>;
}

impl Cor2 for DMatrix<f64> {
    /// Converts the correlation matrix in-place to a covariance matrix using standard deviations.
    /// # Example
    /// ```
    /// use nalgebra::{DMatrix, DVector};
    /// use nal_stats::Cor2;
    /// let mut matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 0.5, 0.5]));
    /// let scale = DVector::from_vec(vec![2.0, 3.0, 4.0]);
    /// matrix.cor2cov_(&scale);
    /// ```
    #[inline]
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
    /// Converts the correlation matrix to a covariance matrix using standard deviations, returning a new matrix.
    /// # Example
    /// ```
    /// use nalgebra::{DMatrix, DVector};
    /// use nal_stats::Cor2;
    /// let matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 0.5, 0.5]));
    /// let scale = DVector::from_vec(vec![2.0, 3.0, 4.0]);
    /// let result = matrix.cor2cov(&scale); // Returns a new matrix
    /// ```
    #[inline]
    fn cor2cov(&self, s: &DVector<f64>) -> Self {
        let mut result = self.clone();
        result.cor2cov_(s);
        result
    }
    /// Computes the upper triangular Cholesky decomposition of the correlation matrix converted to a covariance matrix.
    /// # Example
    /// ```
    /// use nalgebra::{DMatrix, DVector};
    /// use nal_stats::Cor2;
    /// let cor = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 0.5, 0.5]));
    /// let std_dev = DVector::from_vec(vec![2.0, 3.0, 4.0]);
    /// let chol_u = cor.cor2chol_u(&std_dev); // Returns upper triangular Cholesky decomposition
    /// ```
    #[inline]
    fn cor2chol_u(&self, s: &DVector<f64>) -> DMatrix<f64> {
        let cov = self.cor2cov(s);
        let chol = Cholesky::new(cov).expect("Matrix is not positive-definite");
        chol.l().transpose()
    }
}
