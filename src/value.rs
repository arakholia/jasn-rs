use std::collections::BTreeMap;

/// A wrapper for `Vec<u8>`, until Rust supports specialization.
#[derive(Debug, Clone, PartialEq)]
pub struct Binary(pub Vec<u8>);

/// A value type similar to JSON, but extended with separate integer and binary types.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Binary(Binary),
    List(Vec<Value>),
    Map(BTreeMap<String, Value>),
}

impl Value {
    pub fn as_bool(&self) -> Option<bool> {
        if let Value::Bool(b) = self {
            Some(*b)
        } else {
            None
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        if let Value::Int(i) = self {
            Some(*i)
        } else {
            None
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        if let Value::Float(f) = self {
            Some(*f)
        } else {
            None
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        if let Value::String(s) = self {
            Some(s)
        } else {
            None
        }
    }

    pub fn as_binary(&self) -> Option<&Binary> {
        if let Value::Binary(b) = self {
            Some(b)
        } else {
            None
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        if let Value::List(l) = self {
            Some(l)
        } else {
            None
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        if let Value::Map(m) = self {
            Some(m)
        } else {
            None
        }
    }
}

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Value::Bool(value)
    }
}

impl From<i64> for Value {
    fn from(value: i64) -> Self {
        Value::Int(value)
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value::Float(value)
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value::String(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value::String(value.to_string())
    }
}

impl From<Binary> for Value {
    fn from(value: Binary) -> Self {
        Value::Binary(value)
    }
}

impl<V> From<Vec<V>> for Value
where
    V: Into<Value>,
{
    fn from(vec: Vec<V>) -> Self {
        Value::List(vec.into_iter().map(Into::into).collect())
    }
}
