use nalgebra::{DMatrix, Scalar};
use num_traits::NumCast;
use polars::datatypes::DataType::*;
use polars::prelude::*;

pub trait Df2Nal {
    fn to_nal_mat<N>(&self) -> PolarsResult<DMatrix<N>>
    where
        N: Scalar + NumCast;
}

impl Df2Nal for DataFrame {
    fn to_nal_mat<N>(&self) -> PolarsResult<DMatrix<N>>
    where
        N: Scalar + NumCast,
    {
        let columns = self.get_columns();
        let nrows = self.height();
        let ncols = columns.len();

        if nrows == 0 || ncols == 0 {
            return Err(PolarsError::NoData("DataFrame is empty".into()));
        }

        let mut data: Vec<N> = Vec::with_capacity(nrows * ncols);

        for col in columns {
            match col.dtype() {
                // Only accept numeric and boolean types
                UInt8 | UInt16 | UInt32 | UInt64 | Int8 | Int16 | Int32 | Int64 | Float32
                | Float64 | Boolean => {
                    let col = col.cast(&DataType::Float64)?;
                    let ca = col.f64().unwrap();

                    for opt_value in ca.into_iter() {
                        let value = match opt_value {
                            Some(v) => v,
                            None => f64::NAN, // Represent null values as NaN
                        };
                        let casted_value: N = NumCast::from(value).unwrap();
                        data.push(casted_value);
                    }
                }
                other => {
                    return Err(PolarsError::ComputeError(
                        format!("Unsupported data type for conversion: {}", other).into(),
                    ));
                }
            }
        }

        Ok(DMatrix::from_vec(nrows, ncols, data))
    }
}

pub trait ChunkArr2Nal<T> {
    fn to_nal_vec(&self) -> PolarsResult<DMatrix<T>>
    where
        T: Scalar + num_traits::NumCast;
}

impl<T> ChunkArr2Nal<T::Native> for ChunkedArray<T>
where
    T: PolarsNumericType,
    T::Native: Scalar + num_traits::NumCast,
{
    fn to_nal_vec(&self) -> PolarsResult<DMatrix<T::Native>> {
        let slice = self.cont_slice()?;
        let data: Vec<T::Native> = slice.to_vec();
        Ok(DMatrix::from_vec(slice.len(), 1, data))
    }
}

pub trait ListChunk2Nal {
    fn to_nal_mat<N>(&self) -> PolarsResult<DMatrix<N>>
    where
        N: PolarsNumericType + Scalar + num_traits::NumCast;
}

impl ListChunk2Nal for ListChunked {
    fn to_nal_mat<N>(&self) -> PolarsResult<DMatrix<N>>
    where
        N: PolarsNumericType + Scalar + num_traits::NumCast,
    {
        polars_ensure!(
            self.null_count() == 0,
            ComputeError: "Creation of matrix with null values is not supported"
        );

        let mut iter = self.into_no_null_iter();
        let first_series = iter
            .next()
            .ok_or_else(|| polars_err!(NoData: "Unable to create matrix from empty ListChunked"))?;

        let nrows = self.len();
        let ncols = first_series.len();
        let mut data: Vec<N> = Vec::with_capacity(nrows * ncols);

        let append_series_data_to_vec = |s: &Series, data: &mut Vec<N>| -> PolarsResult<()> {
            let ca = s.unpack::<N>()?;
            let slice = ca.cont_slice()?;

            // Perform a numeric cast from the slice's type to N
            for &value in slice {
                let casted_value = NumCast::from(value)
                    .ok_or_else(|| polars_err!(ComputeError: "Failed to cast numeric type"))?;
                data.push(casted_value);
            }
            Ok(())
        };

        append_series_data_to_vec(&first_series, &mut data)?;

        for series in iter {
            polars_ensure!(
                series.len() == ncols,
                ShapeMismatch: "All series must have the same length"
            );
            append_series_data_to_vec(&series, &mut data)?;
        }

        Ok(DMatrix::from_vec(nrows, ncols, data))
    }
}
