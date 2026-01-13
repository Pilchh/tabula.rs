use std::fmt;

use crate::dataframe::core::{Series, errors::DataFrameError, series_data::SeriesData};

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

    pub fn from_series(series: Vec<Series>) -> Result<Self, DataFrameError> {
        if series.len() == 0 {
            return Err(DataFrameError::SeriesVectorEmpty);
        }

        let series_length = series
            .first()
            .expect("series vector must contain at least one series")
            .len();

        if !DataFrame::are_valid_series(&series) {
            return Err(DataFrameError::SeriesLengthNotEqual);
        }

        Ok(DataFrame {
            height: series_length,
            series,
        })
    }

    pub fn shape(&self) -> (usize, usize) {
        (self.height, self.series.len())
    }

    fn is_valid_series(&self, series: Series) -> bool {
        series.len() == self.height
    }

    fn are_valid_series(series: &Vec<Series>) -> bool {
        if let Some(first) = series.first() {
            let length = first.len();
            series.into_iter().all(|s| s.len() == length)
        } else {
            true
        }
    }
}

const PADDING: usize = 15;

impl fmt::Display for DataFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.series.len() == 0 {
            writeln!(f, "Empty Dataframe")?;
            return Ok(());
        }

        let shape = self.shape();
        writeln!(f, "Height: {}, Width: {}", shape.0, shape.1)?;
        writeln!(f, "{}", "-".repeat(&self.series.len() * PADDING))?;

        for s in &self.series {
            write!(f, "{:<p$}", &s.name, p = PADDING)?
        }
        writeln!(f, "\n{}", "-".repeat(&self.series.len() * PADDING))?;

        for i in 0..self.height {
            for s in &self.series {
                match &s.data {
                    SeriesData::String(v) => {
                        let s = format!("\"{}\"", v[i]);
                        write!(f, "{:<p$}", s, p = PADDING)?;
                    }
                    SeriesData::Int64(v) => {
                        write!(f, "{:<p$}", v[i], p = PADDING)?;
                    }
                    SeriesData::Float64(v) => {
                        let s = format!("{:.1}", v[i]);
                        write!(f, "{:<p$}", s, p = PADDING)?;
                    }
                    SeriesData::Bool(v) => {
                        write!(f, "{:<p$}", v[i], p = PADDING)?;
                    }
                }
            }
            writeln!(f)?;
        }

        writeln!(f, "{}", "-".repeat(&self.series.len() * PADDING))?;

        writeln!(f, "dtypes:")?;

        for s in &self.series {
            writeln!(f, "{}: {}", s.name, s.dtype)?;
        }

        writeln!(f, "{}", "-".repeat(&self.series.len() * PADDING))?;

        Ok(())
    }
}
