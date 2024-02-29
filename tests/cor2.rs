use nal_stats::Cor2cov;

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, DVector};

    #[test]
    #[should_panic(expected = "inconsistent dimensions")]
    fn test_cor2cov_inconsistent_dimensions() {
        let mut matrix = DMatrix::from_element(2, 3, 0.0);
        let scale = DVector::from_element(2, 1.0);
        matrix.cor2cov_(&scale); // This should panic
    }

    #[test]
    fn test_cor2cov_in_place() {
        let mut matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 0.5, 0.5]));
        let scale = DVector::from_vec(vec![2.0, 3.0, 4.0]);
        let expected = DMatrix::from_diagonal(&DVector::from_vec(vec![4.0, 4.5, 8.0]));
        matrix.cor2cov_(&scale);

        assert_eq!(matrix, expected);
    }

    #[test]
    fn test_cor2cov_out_of_place() {
        let matrix = DMatrix::from_diagonal(&DVector::from_vec(vec![1.0, 0.5, 0.5]));
        let scale = DVector::from_vec(vec![2.0, 3.0, 4.0]);
        let expected = DMatrix::from_diagonal(&DVector::from_vec(vec![4.0, 4.5, 8.0]));
        let result = matrix.cor2cov(&scale);

        assert_eq!(result, expected);
    }
}
