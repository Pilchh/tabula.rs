use crate::dataframe::core::{Series, errors::DataFrameError};

pub struct DataFrame {
    height: usize,
    series: Vec<Series>,
}

impl DataFrame {
    pub fn empty() -> Self {
        DataFrame {
            height: 0,
            series: Vec::new(),
        }
    }

    // pub fn from_series(series: Vec<Series>) -> Result<Self, DataFrameError> {
    //     if !
    // }

    fn is_valid_series(&self, series: Series) -> bool {
        series.len() == self.height
    }

    fn are_valid_series(series: Vec<Series>) -> bool {
        if let Some(first) = series.first() {
            let length = first.len();
            series.into_iter().all(|s| s.len() == length)
        } else {
            true
        }
    }
}
