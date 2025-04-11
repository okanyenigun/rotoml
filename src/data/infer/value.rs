#[derive(Debug, Clone, PartialEq)]
pub enum DataValue {
    Numeric(f64),
    Integer(i64),
    Text(String),
    Boolean(bool),
    Null,
}

impl DataValue {
    #[allow(dead_code)]
    pub fn type_name(&self) -> &'static str {
        match self {
            Self::Numeric(_) => "numeric",
            Self::Integer(_) => "integer",
            Self::Text(_) => "text",
            Self::Boolean(_) => "boolean",
            Self::Null => "null",
        }
    }
}
