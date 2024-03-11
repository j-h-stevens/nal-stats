use nal_stats::pola2::Df2Nal;
use polars::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra::DMatrix;

    #[test]
    fn dataframe_to_dmatrix_f64() {
        let df = df!["a" => [1.0, 2.0, 3.0], "b" => [4.0, 5.0, 6.0]].unwrap();
        let matrix_result = df.to_nal_mat::<f64>().unwrap();
        let expected = DMatrix::<f64>::from_row_slice(3, 2, &[1.0, 4.0, 2.0, 5.0, 3.0, 6.0]);
        assert_eq!(matrix_result, expected);
    }

    #[test]
    fn dataframe_with_non_numeric_column() {
        let df = df!["a" => [1.0, 2.0, 3.0], "b" => ["x", "y", "z"]].unwrap();
        assert!(df.to_nal_mat::<f64>().is_err());
    }

    //Helper test function
    use num_traits::{NumCast, ToPrimitive};

    fn vecs_almost_equal<N>(left: &DMatrix<N>, right: &DMatrix<N>, epsilon: f64) -> bool
    where
        N: NumCast + ToPrimitive + Copy,
    {
        if left.shape() != right.shape() {
            return false;
        }

        left.iter().zip(right.iter()).all(|(&a, &b)| {
            // Convert N to f64 for comparison, handling potential NaN values
            let a_f64 = N::to_f64(&a).unwrap_or(f64::NAN);
            let b_f64 = N::to_f64(&b).unwrap_or(f64::NAN);

            // Check if both are NaN or within epsilon range
            (a_f64.is_nan() && b_f64.is_nan()) || (a_f64 - b_f64).abs() <= epsilon
        })
    }

    #[test]
    fn dataframe_with_nulls_to_dmatrix() {
        let df =
            df!["a" => [Some(1.0), None, Some(3.0)], "b" => [Some(4.0), Some(5.0), None]].unwrap();
        let matrix_result = df.to_nal_mat::<f64>().unwrap();
        let expected =
            DMatrix::<f64>::from_row_slice(3, 2, &[1.0, 4.0, f64::NAN, 5.0, 3.0, f64::NAN]);
        let epsilon: f64 = 0.00001;
        assert!(vecs_almost_equal(&matrix_result, &expected, epsilon));
    }
}
