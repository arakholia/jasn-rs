use std::{borrow::Cow, collections::BTreeMap};

use crate::Binary;

/// A value type similar to JSON, but extended with separate integer and binary types.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum Value {
    #[default]
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
    pub fn is_null(&self) -> bool {
        matches!(self, Value::Null)
    }

    pub fn is_bool(&self) -> bool {
        matches!(self, Value::Bool(_))
    }

    pub fn is_int(&self) -> bool {
        matches!(self, Value::Int(_))
    }

    pub fn is_float(&self) -> bool {
        matches!(self, Value::Float(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Value::String(_))
    }

    pub fn is_binary(&self) -> bool {
        matches!(self, Value::Binary(_))
    }

    pub fn is_list(&self) -> bool {
        matches!(self, Value::List(_))
    }

    pub fn is_map(&self) -> bool {
        matches!(self, Value::Map(_))
    }

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    pub fn as_int(&self) -> Option<i64> {
        match self {
            Value::Int(i) => Some(*i),
            _ => None,
        }
    }

    pub fn as_float(&self) -> Option<f64> {
        match self {
            Value::Float(f) => Some(*f),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Value::String(s) => Some(s),
            _ => None,
        }
    }

    pub fn as_binary(&self) -> Option<&Binary> {
        match self {
            Value::Binary(b) => Some(b),
            _ => None,
        }
    }

    pub fn as_list(&self) -> Option<&[Value]> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_map(&self) -> Option<&BTreeMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }

    pub fn as_list_mut(&mut self) -> Option<&mut Vec<Value>> {
        match self {
            Value::List(l) => Some(l),
            _ => None,
        }
    }

    pub fn as_map_mut(&mut self) -> Option<&mut BTreeMap<String, Value>> {
        match self {
            Value::Map(m) => Some(m),
            _ => None,
        }
    }

    /// Takes the value, leaving `Value::Null` in its place.
    pub fn take(&mut self) -> Value {
        std::mem::replace(self, Value::Null)
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

impl<'a> From<Cow<'a, str>> for Value {
    fn from(value: Cow<'a, str>) -> Self {
        Value::String(value.into_owned())
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
        vec.into_iter().collect()
    }
}

impl<V> From<&[V]> for Value
where
    V: Into<Value> + Clone,
{
    fn from(slice: &[V]) -> Self {
        slice.iter().cloned().collect()
    }
}

impl<V, const N: usize> From<[V; N]> for Value
where
    V: Into<Value>,
{
    fn from(arr: [V; N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<V, const N: usize> From<&[V; N]> for Value
where
    V: Into<Value> + Clone,
{
    fn from(arr: &[V; N]) -> Self {
        arr.iter().cloned().collect()
    }
}

impl<V> FromIterator<V> for Value
where
    V: Into<Value>,
{
    fn from_iter<I: IntoIterator<Item = V>>(iter: I) -> Self {
        Value::List(iter.into_iter().map(Into::into).collect())
    }
}

impl<K, V> FromIterator<(K, V)> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from_iter<I: IntoIterator<Item = (K, V)>>(iter: I) -> Self {
        Value::Map(
            iter.into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        )
    }
}

impl<K, V> From<&[(K, V)]> for Value
where
    K: Into<String> + Clone,
    V: Into<Value> + Clone,
{
    fn from(slice: &[(K, V)]) -> Self {
        slice.iter().cloned().collect()
    }
}

impl<K, V, const N: usize> From<[(K, V); N]> for Value
where
    K: Into<String>,
    V: Into<Value>,
{
    fn from(arr: [(K, V); N]) -> Self {
        arr.into_iter().collect()
    }
}

impl<K, V, const N: usize> From<&[(K, V); N]> for Value
where
    K: Into<String> + Clone,
    V: Into<Value> + Clone,
{
    fn from(arr: &[(K, V); N]) -> Self {
        arr.iter().cloned().collect()
    }
}

impl<V> From<Option<V>> for Value
where
    V: Into<Value>,
{
    fn from(opt: Option<V>) -> Self {
        match opt {
            Some(v) => v.into(),
            None => Value::Null,
        }
    }
}

impl PartialEq<str> for Value {
    fn eq(&self, other: &str) -> bool {
        self.as_string() == Some(other)
    }
}

impl PartialEq<&str> for Value {
    fn eq(&self, other: &&str) -> bool {
        self.as_string() == Some(*other)
    }
}

impl PartialEq<String> for Value {
    fn eq(&self, other: &String) -> bool {
        self.as_string() == Some(other.as_str())
    }
}

impl PartialEq<i64> for Value {
    fn eq(&self, other: &i64) -> bool {
        self.as_int() == Some(*other)
    }
}

impl PartialEq<f64> for Value {
    fn eq(&self, other: &f64) -> bool {
        self.as_float() == Some(*other)
    }
}

impl PartialEq<bool> for Value {
    fn eq(&self, other: &bool) -> bool {
        self.as_bool() == Some(*other)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(Value::Null, "null")]
    #[case(Value::Bool(true), "bool")]
    #[case(Value::Int(42), "int")]
    #[case(Value::Float(3.14), "float")]
    #[case(Value::String("hello".to_string()), "string")]
    #[case(Value::Binary(Binary(vec![1, 2, 3])), "binary")]
    #[case(Value::List(vec![Value::Null]), "list")]
    #[case(Value::Map(BTreeMap::new()), "map")]
    fn test_is_methods(#[case] value: Value, #[case] value_type: &str) {
        assert_eq!(value.is_null(), value_type == "null");
        assert_eq!(value.is_bool(), value_type == "bool");
        assert_eq!(value.is_int(), value_type == "int");
        assert_eq!(value.is_float(), value_type == "float");
        assert_eq!(value.is_string(), value_type == "string");
        assert_eq!(value.is_binary(), value_type == "binary");
        assert_eq!(value.is_list(), value_type == "list");
        assert_eq!(value.is_map(), value_type == "map");
    }

    #[test]
    fn test_as_bool() {
        assert_eq!(Value::Bool(true).as_bool(), Some(true));
        assert_eq!(Value::Null.as_bool(), None);
    }

    #[test]
    fn test_as_int() {
        assert_eq!(Value::Int(42).as_int(), Some(42));
        assert_eq!(Value::Float(3.14).as_int(), None);
    }

    #[test]
    fn test_as_float() {
        assert_eq!(Value::Float(3.14).as_float(), Some(3.14));
        assert_eq!(Value::Int(42).as_float(), None);
    }

    #[test]
    fn test_as_string() {
        assert_eq!(
            Value::String("hello".to_string()).as_string(),
            Some("hello")
        );
        assert_eq!(Value::Null.as_string(), None);
    }

    #[test]
    fn test_as_binary() {
        let binary = Binary(vec![1, 2, 3]);
        let binary_val = Value::Binary(binary.clone());
        assert_eq!(binary_val.as_binary(), Some(&binary));
        assert_eq!(Value::Null.as_binary(), None);
    }

    #[test]
    fn test_as_list() {
        let list = vec![Value::Int(1), Value::Int(2)];
        let list_val = Value::List(list.clone());
        assert_eq!(list_val.as_list(), Some(list.as_slice()));
        assert_eq!(Value::Null.as_list(), None);
    }

    #[test]
    fn test_as_map() {
        let mut map = BTreeMap::new();
        map.insert("key".to_string(), Value::Int(42));
        let map_val = Value::Map(map.clone());
        assert_eq!(map_val.as_map(), Some(&map));
        assert_eq!(Value::Null.as_map(), None);
    }

    #[rstest]
    #[case(Value::from(()), Value::Null)]
    #[case(Value::from(true), Value::Bool(true))]
    #[case(Value::from(42i64), Value::Int(42))]
    #[case(Value::from(3.14f64), Value::Float(3.14))]
    #[case(Value::from("hello".to_string()), Value::String("hello".to_string()))]
    #[case(Value::from("world"), Value::String("world".to_string()))]
    fn test_from_primitives(#[case] actual: Value, #[case] expected: Value) {
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_conversions() {
        // From primitives - tested via rstest above

        // From Cow
        let owned: Cow<str> = Cow::Owned("owned".to_string());
        assert_eq!(Value::from(owned), Value::String("owned".to_string()));
        let borrowed: Cow<str> = Cow::Borrowed("borrowed");
        assert_eq!(Value::from(borrowed), Value::String("borrowed".to_string()));

        // From Binary
        let binary = Binary(vec![1, 2, 3]);
        assert_eq!(Value::from(binary.clone()), Value::Binary(binary));

        // Value::Binary from byte literal
        let value = Value::Binary(b"data".into());
        assert_eq!(value, Value::Binary(Binary(b"data".to_vec())));

        // From Vec
        let vec = vec![1i64, 2, 3];
        let list_val = Value::from(vec);
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );

        // From &[V]
        let slice: &[i64] = &[1, 2, 3];
        let list_val = Value::from(slice);
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );

        // FromIterator for List
        let list_val: Value = vec![1i64, 2, 3].into_iter().collect();
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );

        // FromIterator for Map
        let map_val: Value = vec![("a", 1i64), ("b", 2)].into_iter().collect();
        let mut expected_map = BTreeMap::new();
        expected_map.insert("a".to_string(), Value::Int(1));
        expected_map.insert("b".to_string(), Value::Int(2));
        assert_eq!(map_val, Value::Map(expected_map));

        // From &[(K, V)]
        let slice: &[(&str, i64)] = &[("x", 10), ("y", 20)];
        let map_val = Value::from(slice);
        let mut expected_map = BTreeMap::new();
        expected_map.insert("x".to_string(), Value::Int(10));
        expected_map.insert("y".to_string(), Value::Int(20));
        assert_eq!(map_val, Value::Map(expected_map));

        // From [V; N] - owned array to List
        let list_val = Value::from([1i64, 2, 3]);
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );

        // From &[V; N] - array reference to List
        let arr = [4i64, 5, 6];
        let list_val = Value::from(&arr);
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(4), Value::Int(5), Value::Int(6)])
        );

        // From [(K, V); N] - owned array to Map
        let map_val = Value::from([("a", 1i64), ("b", 2)]);
        let mut expected_map = BTreeMap::new();
        expected_map.insert("a".to_string(), Value::Int(1));
        expected_map.insert("b".to_string(), Value::Int(2));
        assert_eq!(map_val, Value::Map(expected_map));

        // From &[(K, V); N] - array reference to Map
        let arr = [("c", 3i64), ("d", 4)];
        let map_val = Value::from(&arr);
        let mut expected_map = BTreeMap::new();
        expected_map.insert("c".to_string(), Value::Int(3));
        expected_map.insert("d".to_string(), Value::Int(4));
        assert_eq!(map_val, Value::Map(expected_map));

        // From Option
        assert_eq!(Value::from(Some(42i64)), Value::Int(42));
        assert_eq!(Value::from(None::<i64>), Value::Null);
    }

    #[test]
    fn test_default() {
        assert_eq!(Value::default(), Value::Null);
    }

    #[test]
    fn test_mutable_accessors() {
        // as_list_mut
        let mut list_val = Value::List(vec![Value::Int(1), Value::Int(2)]);
        if let Some(list) = list_val.as_list_mut() {
            list.push(Value::Int(3));
            list[0] = Value::Int(10);
        }
        assert_eq!(
            list_val,
            Value::List(vec![Value::Int(10), Value::Int(2), Value::Int(3)])
        );

        // as_list_mut returns None for non-list
        let mut int_val = Value::Int(42);
        assert_eq!(int_val.as_list_mut(), None);

        // as_map_mut
        let mut map_val = Value::Map(BTreeMap::new());
        if let Some(map) = map_val.as_map_mut() {
            map.insert("key".to_string(), Value::Int(42));
            if let Some(value) = map.get_mut("key") {
                *value = Value::Int(99);
            }
        }
        let mut expected = BTreeMap::new();
        expected.insert("key".to_string(), Value::Int(99));
        assert_eq!(map_val, Value::Map(expected));

        // as_map_mut returns None for non-map
        assert_eq!(int_val.as_map_mut(), None);
    }

    #[test]
    fn test_take() {
        let mut value = Value::Int(42);
        let taken = value.take();
        assert_eq!(taken, Value::Int(42));
        assert_eq!(value, Value::Null);

        let mut list = Value::List(vec![Value::Int(1), Value::Int(2)]);
        let taken = list.take();
        assert_eq!(taken, Value::List(vec![Value::Int(1), Value::Int(2)]));
        assert_eq!(list, Value::Null);

        // Taking from Null leaves Null
        let mut null = Value::Null;
        let taken = null.take();
        assert_eq!(taken, Value::Null);
        assert_eq!(null, Value::Null);
    }

    #[test]
    fn test_partial_eq_string() {
        let string_val = Value::String("hello".to_string());
        assert_eq!(string_val, "hello");
        assert_eq!(string_val, "hello".to_string());
        assert_ne!(string_val, "world");
    }

    #[test]
    fn test_partial_eq_int() {
        let int_val = Value::Int(42);
        assert_eq!(int_val, 42i64);
        assert_ne!(int_val, 43i64);
    }

    #[test]
    fn test_partial_eq_float() {
        let float_val = Value::Float(3.14);
        assert_eq!(float_val, 3.14f64);
        assert_ne!(float_val, 2.71f64);
    }

    #[test]
    fn test_partial_eq_bool() {
        let bool_val = Value::Bool(true);
        assert_eq!(bool_val, true);
        assert_ne!(bool_val, false);
    }

    #[test]
    fn test_partial_eq_mismatched_types() {
        let int_val = Value::Int(42);
        let string_val = Value::String("hello".to_string());
        assert_ne!(int_val, "42");
        assert_ne!(string_val, 42i64);
    }
}
