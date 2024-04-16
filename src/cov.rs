use nalgebra::{Dim, Matrix, RawStorageMut, Scalar};
use num_traits::Float;

pub trait Stats {
    fn symmetrize_(&mut self);
}

impl<T, R, C, S> Stats for Matrix<T, R, C, S>
where
    T: Scalar + Float,
    R: Dim,
    C: Dim,
    S: RawStorageMut<T, R, C>,
{
    /// Symmetrizes the matrix in-place.
    ///
    /// This function modifies the matrix so that it becomes symmetric. It averages corresponding
    /// off-diagonal elements from the upper and lower triangles of the matrix, effectively making `a[i, j]`
    /// and `a[j, i]` equal to `(a[i, j] + a[j, i]) / 2` for all `i` and `j`.
    ///
    /// # Panics
    /// This function will panic if the matrix is not square.
    ///
    /// # Example
    /// ```
    /// use nalgebra::{Matrix3, Scalar};
    /// use num_traits::Float;
    /// use nal_stats::Stats; // Replace `your_crate_name` with the actual crate name
    ///
    /// let mut matrix = Matrix3::new(1.0, 2.0, 3.0,
    ///                               4.0, 5.0, 6.0,
    ///                               7.0, 8.0, 9.0);
    /// matrix.symmetrize_();
    /// assert_eq!(matrix, Matrix3::new(1.0, 3.0, 5.0,
    ///                                 3.0, 5.0, 7.0,
    ///                                 5.0, 7.0, 9.0));
    /// ```
    #[inline]
    fn symmetrize_(&mut self) {
        assert_eq!(
            self.nrows(),
            self.ncols(),
            "matrix must be a square matrix."
        );
        let n = self.ncols();

        for j in 0..n {
            for i in j + 1..n {
                let vl = self[(i, j)];
                let vr = self[(j, i)];
                let middle_value = (vl + vr) / T::from(2.0).expect("Type conversion failed");
                self[(i, j)] = middle_value;
                self[(j, i)] = middle_value;
            }
        }
    }
}
