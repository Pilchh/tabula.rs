use thiserror::Error;

#[derive(Debug, Error)]
pub enum DataFrameError {
    #[error("series length mismatch: expected {expected}, got {actual}")]
    SeriesLengthMismatch { expected: usize, actual: usize },
}
