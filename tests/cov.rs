use nal_stats::Stats;

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::{DMatrix, Matrix1, Matrix2, Matrix3, Matrix4};

    #[test]
    fn test_basic_symmetrization() {
        let mut matrix = Matrix3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        matrix.symmetrize_();
        let expected = Matrix3::new(1.0, 3.0, 5.0, 3.0, 5.0, 7.0, 5.0, 7.0, 9.0);
        assert_eq!(matrix, expected, "Matrix should be symmetrically adjusted.");
    }

    #[test]
    fn test_already_symmetric() {
        let mut matrix = Matrix2::new(1.0, 2.0, 2.0, 3.0);
        let original = matrix.clone();
        matrix.symmetrize_();
        assert_eq!(
            matrix, original,
            "Already symmetric matrix should remain unchanged."
        );
    }

    #[test]
    fn test_single_element() {
        let mut matrix = Matrix1::new(1.0);
        let original = matrix.clone();
        matrix.symmetrize_();
        assert_eq!(
            matrix, original,
            "Single element matrix should remain unchanged."
        );
    }

    #[test]
    fn test_zero_matrix() {
        let mut matrix = Matrix4::from_element(0.0);
        let original = matrix.clone();
        matrix.symmetrize_();
        assert_eq!(matrix, original, "Zero matrix should remain unchanged.");
    }

    #[test]
    fn test_negative_values() {
        let mut matrix = Matrix2::new(-1.0, -2.0, -3.0, -4.0);
        matrix.symmetrize_();
        let expected = Matrix2::new(-1.0, -2.5, -2.5, -4.0);
        assert_eq!(
            matrix, expected,
            "Matrix with negative values should be correctly symmetrized."
        );
    }

    #[test]
    fn test_floating_point_types() {
        let mut matrix_f32 = Matrix2::new(1.0f32, 2.0f32, 3.0f32, 4.0f32);
        matrix_f32.symmetrize_();
        let expected_f32 = Matrix2::new(1.0f32, 2.5f32, 2.5f32, 4.0f32);
        assert_eq!(
            matrix_f32, expected_f32,
            "f32 matrix should be correctly symmetrized."
        );

        let mut matrix_f64 = Matrix2::new(1.0f64, 2.0f64, 3.0f64, 4.0f64);
        matrix_f64.symmetrize_();
        let expected_f64 = Matrix2::new(1.0f64, 2.5f64, 2.5f64, 4.0f64);
        assert_eq!(
            matrix_f64, expected_f64,
            "f64 matrix should be correctly symmetrized."
        );
    }

    #[test]
    fn test_dmatrix_symmetrization() {
        // Create a non-symmetric DMatrix
        let mut matrix =
            DMatrix::from_row_slice(3, 3, &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

        // Apply symmetrization
        matrix.symmetrize_();

        // Expected symmetric matrix
        let expected =
            DMatrix::from_row_slice(3, 3, &[1.0, 3.0, 5.0, 3.0, 5.0, 7.0, 5.0, 7.0, 9.0]);

        assert_eq!(matrix, expected, "DMatrix should be correctly symmetrized.");
    }
}
