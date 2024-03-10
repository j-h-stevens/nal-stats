use nal_stats::Randn;

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    #[test]
    fn randn_matrix() {
        let n = (50, 1000);
        let seed = 1234;
        let matrix: DMatrix<f64> = DMatrix::randn(seed, n);
        let mean = matrix.mean();
        let std = matrix.variance().sqrt();
        assert!(mean < 1e-2);
        assert!(std - 1.0 < 1e-2);
    }
    #[test]
    fn randn_vector() {
        let n = 50000;
        let seed = 1234;
        let vector: DVector<f64> = DVector::randn(seed, n);
        let mean = vector.mean();
        let std = vector.variance().sqrt();
        assert!(mean < 1e-2);
        assert!(std - 1.0 < 1e-2);
    }
    #[test]
    fn randn_vec_matrices() {
        let nrows = 10;
        let ncols = 5;
        let nsims = 1000;
        let seed = 1234;
        let matrices: Vec<DMatrix<f64>> = Vec::<DMatrix<f64>>::randn(seed, (nrows, ncols, nsims));
        let mut means: DVector<f64> = DVector::zeros(nsims);
        let mut vols: DVector<f64> = DVector::zeros(nsims);

        for i in 0..nsims {
            means[i] += matrices[i].mean();
            vols[i] += matrices[i].variance().sqrt();
        }
        assert!(
            means.mean() < 1e-2,
            "Mean is too far from 0: {}",
            means.mean()
        );
        assert!(
            (vols.mean() - 1.0) < 1e-2,
            "Std is too far from 1: {}",
            vols.mean()
        );
    }
}
