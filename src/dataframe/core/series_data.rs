use std::fmt;

use crate::dataframe::{core::dtype::Dtype, utils::strings::format_f64};

#[derive(Debug, Clone)]
pub enum SeriesData {
    Int64(Vec<i64>),
    Float64(Vec<f64>),
    String(Vec<String>),
    Bool(Vec<bool>),
}

impl SeriesData {
    pub fn dtype(&self) -> Dtype {
        match self {
            SeriesData::Int64(_) => Dtype::Int64,
            SeriesData::Float64(_) => Dtype::Float64,
            SeriesData::String(_) => Dtype::String,
            SeriesData::Bool(_) => Dtype::Bool,
        }
    }

    pub fn as_strings(&self) -> Vec<String> {
        match self {
            SeriesData::Int64(v) => v.iter().map(|x| x.to_string()).collect(),
            SeriesData::Float64(v) => v.iter().map(|x| format_f64(*x)).collect(),
            SeriesData::String(v) => v.clone(),
            SeriesData::Bool(v) => v.iter().map(|x| x.to_string()).collect(),
        }
    }

    pub fn len(&self) -> usize {
        match self {
            SeriesData::Int64(v) => v.len(),
            SeriesData::Float64(v) => v.len(),
            SeriesData::String(v) => v.len(),
            SeriesData::Bool(v) => v.len(),
        }
    }
}

pub trait IntoSeriesData {
    fn into_series_data(data: Vec<Self>) -> SeriesData
    where
        Self: Sized;
}

impl IntoSeriesData for i64 {
    fn into_series_data(data: Vec<Self>) -> SeriesData {
        SeriesData::Int64(data)
    }
}

impl IntoSeriesData for f64 {
    fn into_series_data(data: Vec<Self>) -> SeriesData {
        SeriesData::Float64(data)
    }
}

impl IntoSeriesData for String {
    fn into_series_data(data: Vec<Self>) -> SeriesData {
        SeriesData::String(data)
    }
}

impl IntoSeriesData for bool {
    fn into_series_data(data: Vec<Self>) -> SeriesData {
        SeriesData::Bool(data)
    }
}
