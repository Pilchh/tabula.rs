use std::fmt;

#[derive(Debug, Clone)]
pub enum Dtype {
    Int64,
    Float64,
    String,
    Bool,
}

impl Dtype {
    pub fn to_string(&self) -> String {
        match self {
            Dtype::Int64 => "Int64".to_string(),
            Dtype::Float64 => "Float64".to_string(),
            Dtype::String => "String".to_string(),
            Dtype::Bool => "Bool".to_string(),
        }
    }
}

impl fmt::Display for Dtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
