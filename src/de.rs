use serde::de::{
    self, Deserialize, DeserializeSeed, IntoDeserializer, MapAccess, SeqAccess, Visitor,
};

use crate::{parse, Value};

/// Error type for deserialization.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Custom deserialization error.
    #[error("custom error: {0}")]
    Custom(String),
    /// Parse error from the JASN parser.
    #[error("parse error: {0}")]
    Parse(#[from] crate::parser::Error),
    /// Type mismatch during deserialization.
    #[error("expected {expected}, got {got}")]
    TypeMismatch {
        /// Expected type.
        expected: String,
        /// Actual type.
        got: String,
    },
    /// Invalid value encountered.
    #[error("invalid value: {0}")]
    InvalidValue(String),
}

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Error::Custom(msg.to_string())
    }
}

type Result<T> = std::result::Result<T, Error>;

/// Deserialize a JASN string into a Rust value.
pub fn from_str<T>(s: &str) -> Result<T>
where
    T: for<'de> Deserialize<'de>,
{
    let value = parse(s)?;
    from_value(&value)
}

/// Deserialize a JASN `Value` into a Rust value.
pub fn from_value<'de, T>(value: &'de Value) -> Result<T>
where
    T: Deserialize<'de>,
{
    T::deserialize(ValueDeserializer { value })
}

struct ValueDeserializer<'de> {
    value: &'de Value,
}

impl<'de> de::Deserializer<'de> for ValueDeserializer<'de> {
    type Error = Error;

    fn deserialize_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_unit(),
            Value::Bool(v) => visitor.visit_bool(*v),
            Value::Int(v) => visitor.visit_i64(*v),
            Value::Float(v) => visitor.visit_f64(*v),
            Value::String(v) => visitor.visit_str(v),
            Value::Binary(v) => visitor.visit_bytes(&v.0),
            Value::Timestamp(_) => Err(Error::InvalidValue(
                "timestamps must be deserialized explicitly".to_string(),
            )),
            Value::List(v) => visitor.visit_seq(SeqDeserializer {
                iter: v.iter(),
            }),
            Value::Map(v) => visitor.visit_map(MapDeserializer {
                iter: v.iter(),
                value: None,
            }),
        }
    }

    fn deserialize_bool<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Bool(v) => visitor.visit_bool(*v),
            other => Err(Error::TypeMismatch {
                expected: "bool".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_i8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_i8(*v as i8),
            other => Err(Error::TypeMismatch {
                expected: "i8".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_i16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_i16(*v as i16),
            other => Err(Error::TypeMismatch {
                expected: "i16".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_i32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_i32(*v as i32),
            other => Err(Error::TypeMismatch {
                expected: "i32".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_i64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_i64(*v),
            other => Err(Error::TypeMismatch {
                expected: "i64".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_u8<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_u8(*v as u8),
            other => Err(Error::TypeMismatch {
                expected: "u8".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_u16<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_u16(*v as u16),
            other => Err(Error::TypeMismatch {
                expected: "u16".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_u32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_u32(*v as u32),
            other => Err(Error::TypeMismatch {
                expected: "u32".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_u64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Int(v) => visitor.visit_u64(*v as u64),
            other => Err(Error::TypeMismatch {
                expected: "u64".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_f32<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Float(v) => visitor.visit_f32(*v as f32),
            Value::Int(v) => visitor.visit_f32(*v as f32),
            other => Err(Error::TypeMismatch {
                expected: "f32".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_f64<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Float(v) => visitor.visit_f64(*v),
            Value::Int(v) => visitor.visit_f64(*v as f64),
            other => Err(Error::TypeMismatch {
                expected: "f64".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_char<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(v) => {
                let mut chars = v.chars();
                if let Some(ch) = chars.next() {
                    if chars.next().is_none() {
                        return visitor.visit_char(ch);
                    }
                }
                Err(Error::InvalidValue(format!("expected single character, got: {}", v)))
            }
            other => Err(Error::TypeMismatch {
                expected: "char".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_str<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(v) => visitor.visit_str(v),
            other => Err(Error::TypeMismatch {
                expected: "string".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_string<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_bytes<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Binary(v) => visitor.visit_bytes(&v.0),
            other => Err(Error::TypeMismatch {
                expected: "bytes".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_byte_buf<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_bytes(visitor)
    }

    fn deserialize_option<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_none(),
            _ => visitor.visit_some(self),
        }
    }

    fn deserialize_unit<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Null => visitor.visit_unit(),
            other => Err(Error::TypeMismatch {
                expected: "null".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_unit_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_unit(visitor)
    }

    fn deserialize_newtype_struct<V>(self, _name: &'static str, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_newtype_struct(self)
    }

    fn deserialize_seq<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::List(v) => visitor.visit_seq(SeqDeserializer {
                iter: v.iter(),
            }),
            other => Err(Error::TypeMismatch {
                expected: "array".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_tuple<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_tuple_struct<V>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_seq(visitor)
    }

    fn deserialize_map<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Map(v) => visitor.visit_map(MapDeserializer {
                iter: v.iter(),
                value: None,
            }),
            other => Err(Error::TypeMismatch {
                expected: "map".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_map(visitor)
    }

    fn deserialize_enum<V>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::String(s) => visitor.visit_enum(s.as_str().into_deserializer()),
            Value::Map(m) => {
                if m.len() == 1 {
                    let (key, value) = m.iter().next().unwrap();
                    visitor.visit_enum(EnumDeserializer { key, value })
                } else {
                    Err(Error::InvalidValue(
                        "enum must be a string or single-key map".to_string(),
                    ))
                }
            }
            other => Err(Error::TypeMismatch {
                expected: "enum".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn deserialize_identifier<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        self.deserialize_str(visitor)
    }

    fn deserialize_ignored_any<V>(self, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        visitor.visit_unit()
    }
}

struct SeqDeserializer<'de> {
    iter: std::slice::Iter<'de, Value>,
}

impl<'de> SeqAccess<'de> for SeqDeserializer<'de> {
    type Error = Error;

    fn next_element_seed<T>(&mut self, seed: T) -> Result<Option<T::Value>>
    where
        T: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some(value) => seed.deserialize(ValueDeserializer { value }).map(Some),
            None => Ok(None),
        }
    }
}

struct MapDeserializer<'de> {
    iter: std::collections::btree_map::Iter<'de, String, Value>,
    value: Option<&'de Value>,
}

impl<'de> MapAccess<'de> for MapDeserializer<'de> {
    type Error = Error;

    fn next_key_seed<K>(&mut self, seed: K) -> Result<Option<K::Value>>
    where
        K: DeserializeSeed<'de>,
    {
        match self.iter.next() {
            Some((key, value)) => {
                self.value = Some(value);
                seed.deserialize(key.as_str().into_deserializer()).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V>(&mut self, seed: V) -> Result<V::Value>
    where
        V: DeserializeSeed<'de>,
    {
        match self.value.take() {
            Some(value) => seed.deserialize(ValueDeserializer { value }),
            None => Err(Error::Custom("value is missing".to_string())),
        }
    }
}

struct EnumDeserializer<'de> {
    key: &'de String,
    value: &'de Value,
}

impl<'de> de::EnumAccess<'de> for EnumDeserializer<'de> {
    type Error = Error;
    type Variant = VariantDeserializer<'de>;

    fn variant_seed<V>(self, seed: V) -> Result<(V::Value, Self::Variant)>
    where
        V: DeserializeSeed<'de>,
    {
        use serde::de::value::StrDeserializer;
        let deserializer: StrDeserializer<Error> = self.key.as_str().into_deserializer();
        let variant = seed.deserialize(deserializer)?;
        Ok((variant, VariantDeserializer { value: self.value }))
    }
}

struct VariantDeserializer<'de> {
    value: &'de Value,
}

impl<'de> de::VariantAccess<'de> for VariantDeserializer<'de> {
    type Error = Error;

    fn unit_variant(self) -> Result<()> {
        Ok(())
    }

    fn newtype_variant_seed<T>(self, seed: T) -> Result<T::Value>
    where
        T: DeserializeSeed<'de>,
    {
        seed.deserialize(ValueDeserializer { value: self.value })
    }

    fn tuple_variant<V>(self, _len: usize, visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::List(v) => visitor.visit_seq(SeqDeserializer {
                iter: v.iter(),
            }),
            other => Err(Error::TypeMismatch {
                expected: "array for tuple variant".to_string(),
                got: type_name(other),
            }),
        }
    }

    fn struct_variant<V>(self, _fields: &'static [&'static str], visitor: V) -> Result<V::Value>
    where
        V: Visitor<'de>,
    {
        match self.value {
            Value::Map(v) => visitor.visit_map(MapDeserializer {
                iter: v.iter(),
                value: None,
            }),
            other => Err(Error::TypeMismatch {
                expected: "map for struct variant".to_string(),
                got: type_name(other),
            }),
        }
    }
}

fn type_name(value: &Value) -> String {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Int(_) => "int",
        Value::Float(_) => "float",
        Value::String(_) => "string",
        Value::Binary(_) => "binary",
        Value::Timestamp(_) => "timestamp",
        Value::List(_) => "lists",
        Value::Map(_) => "map",
    }
    .to_string()
}
