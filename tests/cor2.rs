use nal_stats::Cor2Chol;
use nal_stats::Cor2Cov;
#[cfg(test)]
mod cor2cov {
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

#[cfg(test)]
mod cor2chol {
    use super::*;
    use nalgebra::{dmatrix, dvector, DMatrix, DVector};

    #[test]
    fn test_valid_correlation_matrix() {
        let cor = dmatrix![
            1.0, 0.5;
            0.5, 1.0
        ];
        let std_dev = dvector![1.0, 2.0];
        let chol_result = cor.cor2chol_u(&std_dev);
        assert!(chol_result.is_ok());
        // Additional checks can be added to verify the contents of the result
    }

    #[test]
    #[should_panic(expected = "Matrix is not positive-definite")]
    fn test_invalid_correlation_matrix() {
        let cor = dmatrix![
            1.0, 2.0;
            2.0, 1.0
        ];
        let std_dev = dvector![1.0, 1.0];
        cor.cor2chol_u(&std_dev).unwrap();
    }

    #[test]
    fn test_empty_matrix_and_vector() {
        let cor = DMatrix::<f64>::zeros(0, 0);
        let std_dev = DVector::<f64>::zeros(0);
        let result = cor.cor2chol_u(&std_dev);
        assert!(result.is_ok());

        // Use `as_ref` to borrow the result for inspection without consuming it
        let unwrapped_result = result.as_ref().unwrap();
        assert_eq!(unwrapped_result.ncols(), 0);
        assert_eq!(unwrapped_result.nrows(), 0);
    }

    #[test]
    fn test_single_element_matrix() {
        let cor = dmatrix![1.0];
        let std_dev = dvector![1.0];
        let result = cor.cor2chol_u(&std_dev).unwrap();
        assert_eq!(result, dmatrix![1.0]);
    }

    #[test]
    #[should_panic(expected = "inconsistent dimensions")]
    fn test_dimension_mismatch() {
        let cor = dmatrix![
            1.0, 0.5;
            0.5, 1.0
        ];
        let std_dev = dvector![1.0];
        cor.cor2chol_u(&std_dev).unwrap();
    }
}
