use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataFrameError {
    #[error("series lengths are not equal")]
    SeriesLengthNotEqual,
    #[error("series length mismatch: expected {expected}, got {actual}")]
    SeriesLengthMismatch { expected: usize, actual: usize },
    #[error("series vector is empty")]
    SeriesVectorEmpty,
}
