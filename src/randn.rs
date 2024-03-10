use nalgebra::{DMatrix, DVector};
use rand::{rngs::SmallRng, SeedableRng};
use rand_distr::{Distribution, StandardNormal};

pub trait Randn {
    type Output;
    fn randn(seed: u64, dim: Self::Output) -> Self;
}

impl Randn for DMatrix<f64> {
    type Output = (usize, usize);

    fn randn(seed: u64, dim: Self::Output) -> Self {
        let (nrows, ncols) = dim;
        let distr = StandardNormal;
        let mut rng = SmallRng::seed_from_u64(seed);
        DMatrix::from_fn(nrows, ncols, |_, _| distr.sample(&mut rng))
    }
}

impl Randn for DVector<f64> {
    type Output = usize;

    fn randn(seed: u64, size: Self::Output) -> Self {
        let distr = StandardNormal;
        let mut rng = SmallRng::seed_from_u64(seed);
        DVector::from_fn(size, |_, _| distr.sample(&mut rng))
    }
}

impl Randn for Vec<DMatrix<f64>> {
    type Output = (usize, usize, usize);

    fn randn(seed: u64, dim: Self::Output) -> Self {
        let (nrows, ncols, count) = dim;
        let mut rng = SmallRng::seed_from_u64(seed);
        let distr = StandardNormal;
        (0..count)
            .map(|_| DMatrix::from_fn(nrows, ncols, |_, _| distr.sample(&mut rng)))
            .collect()
    }
}
