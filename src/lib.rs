pub mod round;
pub use round::Round;

#[cfg(test)]
mod tests {
    use super::*; // This imports Roundable into the scope
    use nalgebra::{DMatrix, DVector};

    #[test]
    fn test_round_vector() {
        // Explicitly using DVector<f64>
        let mut v = DVector::from_vec(vec![123.456, 78.9, 0.12345]);
        v.round_(2); // Now explicitly calling round_ on a DVector<f64>
        assert_eq!(v, DVector::from_vec(vec![123.46, 78.90, 0.12]));

        let rounded = v.round(1); // Now explicitly calling round on a DVector<f64>
        assert_eq!(rounded, DVector::from_vec(vec![123.5, 78.9, 0.1]));
    }

    #[test]
    fn test_round_matrix() {
        // Explicitly using DMatrix<f64>
        let mut m = DMatrix::from_row_slice(2, 2, &[123.456, 78.9, 0.12345, 456.789]);
        m.round_(3); // Now explicitly calling round_ on a DMatrix<f64>
        assert_eq!(
            m,
            DMatrix::from_row_slice(2, 2, &[123.456, 78.900, 0.123, 456.789])
        );

        let rounded = m.round(1); // Now explicitly calling round on a DMatrix<f64>
        assert_eq!(
            rounded,
            DMatrix::from_row_slice(2, 2, &[123.5, 78.9, 0.1, 456.8])
        );
    }
}
