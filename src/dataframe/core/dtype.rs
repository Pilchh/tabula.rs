use std::fmt;

#[derive(Debug)]
pub enum Dtype {
    None,
    Int32,
    Int64,
    Float32,
    Float64,
    String,
    Bool,
}

impl Dtype {
    pub fn to_string(&self) -> String {
        match self {
            Dtype::Int32 => "Int32".to_string(),
            Dtype::Float32 => "Float32".to_string(),
            Dtype::Int64 => "Int64".to_string(),
            Dtype::Float64 => "Float64".to_string(),
            Dtype::String => "String".to_string(),
            Dtype::Bool => "Bool".to_string(),
            Dtype::None => "None".to_string(),
        }
    }
}

impl fmt::Display for Dtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
