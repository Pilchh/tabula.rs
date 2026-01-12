use std::fmt;

use crate::dataframe::core::{Dtype, SeriesData, series_data::IntoSeriesData};

#[derive(Debug)]
pub struct Series {
    name: String,
    data: SeriesData,
    dtype: Dtype,
}

impl Series {
    pub fn from_vec<T: IntoSeriesData>(name: &str, data: Vec<T>) -> Self {
        let name = name.to_string();
        let data = T::into_series_data(data);
        let dtype = data.dtype();

        Series { name, data, dtype }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

impl fmt::Display for Series {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Series: {{")?;
        writeln!(f, "  name: {}", self.name)?;
        writeln!(f, "  dtype: {}", self.dtype)?;
        writeln!(f, "  data: {{")?;

        for line in self.data.as_strings() {
            writeln!(f, "    {}", line)?;
        }

        writeln!(f, "  }}")?;
        writeln!(f, "}}")?;

        Ok(())
    }
}
