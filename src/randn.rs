use nalgebra::{DMatrix, DVector};
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, StandardNormal};

pub trait Randn {
    type Output;
    fn randn(seed: u64, dim: Self::Output) -> Self;
}

impl Randn for DVector<f64> {
    type Output = usize;
    /// Generates a vector with random numbers.
    /// # Example
    /// ```
    /// use nalgebra::DVector;
    /// use nal_stats::Randn;
    /// let seed = 1234;
    /// let size = 1000; // Size of the vector
    /// let vector: DVector<f64> = DVector::randn(seed, size);
    /// ```
    #[inline]
    fn randn(seed: u64, size: Self::Output) -> Self {
        let distr = StandardNormal;
        let mut rng = SmallRng::seed_from_u64(seed);
        DVector::from_fn(size, |_, _| distr.sample(&mut rng))
    }
}

impl Randn for DMatrix<f64> {
    type Output = (usize, usize);
    /// Generates a matrix with random numbers.
    /// # Example
    /// ```
    /// use nalgebra::DMatrix;
    /// use nal_stats::Randn;
    /// let seed = 1234;
    /// let dimensions = (50, 100); // 50 rows, 100 columns
    /// let matrix: DMatrix<f64> = DMatrix::randn(seed, dimensions);
    /// ```
    #[inline]
    fn randn(seed: u64, dim: Self::Output) -> Self {
        let (nrows, ncols) = dim;
        let distr = StandardNormal;
        let mut rng = SmallRng::seed_from_u64(seed);
        DMatrix::from_fn(nrows, ncols, |_, _| distr.sample(&mut rng))
    }
}

impl Randn for Vec<DMatrix<f64>> {
    type Output = (usize, usize, usize);
    /// Generates a vector of matrices with random numbers.
    /// # Example
    /// ```
    /// use nalgebra::DMatrix;
    /// use nal_stats::Randn;
    /// let seed = 1234;
    /// let dimensions = (10, 5, 100); // 10 rows, 5 columns, 100 matrices
    /// let matrices: Vec<DMatrix<f64>> = Vec::<DMatrix<f64>>::randn(seed, dimensions);
    /// ```
    #[inline]
    fn randn(seed: u64, dim: Self::Output) -> Self {
        let (nrows, ncols, count) = dim;
        let mut rng = SmallRng::seed_from_u64(seed);
        let distr = StandardNormal;
        (0..count)
            .map(|_| DMatrix::from_fn(nrows, ncols, |_, _| distr.sample(&mut rng)))
            .collect()
    }
}
