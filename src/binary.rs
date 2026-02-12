use std::{
    borrow::Borrow,
    ops::{Deref, DerefMut},
};

/// A wrapper for [`Vec<u8>`]... at least until Rust supports specialization.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Binary(pub Vec<u8>);

impl Binary {
    /// Creates a new empty `Binary`.
    pub fn new() -> Self {
        Binary(Vec::new())
    }

    /// Creates a new `Binary` with the specified capacity.
    pub fn with_capacity(capacity: usize) -> Self {
        Binary(Vec::with_capacity(capacity))
    }

    /// Returns the number of bytes in the binary data.
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if the binary data contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns the capacity of the underlying vector.
    pub fn capacity(&self) -> usize {
        self.0.capacity()
    }

    /// Reserves capacity for at least `additional` more bytes.
    pub fn reserve(&mut self, additional: usize) {
        self.0.reserve(additional);
    }

    /// Appends a byte to the end of the binary data.
    pub fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    /// Removes and returns the last byte, or `None` if empty.
    pub fn pop(&mut self) -> Option<u8> {
        self.0.pop()
    }

    /// Clears the binary data, removing all bytes.
    pub fn clear(&mut self) {
        self.0.clear();
    }

    /// Truncates the binary data to the specified length.
    pub fn truncate(&mut self, len: usize) {
        self.0.truncate(len);
    }

    /// Appends all bytes from a slice to the binary data.
    pub fn extend_from_slice(&mut self, slice: &[u8]) {
        self.0.extend_from_slice(slice);
    }
}

impl From<Vec<u8>> for Binary {
    fn from(v: Vec<u8>) -> Self {
        Binary(v)
    }
}

impl From<Binary> for Vec<u8> {
    fn from(binary: Binary) -> Self {
        binary.0
    }
}

impl From<&[u8]> for Binary {
    fn from(slice: &[u8]) -> Self {
        Binary(slice.to_vec())
    }
}

impl<const N: usize> From<&[u8; N]> for Binary {
    fn from(arr: &[u8; N]) -> Self {
        Binary(arr.to_vec())
    }
}

impl<const N: usize> From<[u8; N]> for Binary {
    fn from(arr: [u8; N]) -> Self {
        Binary(arr.to_vec())
    }
}

impl AsRef<[u8]> for Binary {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl AsMut<[u8]> for Binary {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.0
    }
}

impl Deref for Binary {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Binary {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Borrow<[u8]> for Binary {
    fn borrow(&self) -> &[u8] {
        &self.0
    }
}

impl PartialEq<[u8]> for Binary {
    fn eq(&self, other: &[u8]) -> bool {
        self.0 == other
    }
}

impl PartialEq<&[u8]> for Binary {
    fn eq(&self, other: &&[u8]) -> bool {
        &self.0 == other
    }
}

impl PartialEq<Vec<u8>> for Binary {
    fn eq(&self, other: &Vec<u8>) -> bool {
        &self.0 == other
    }
}

impl<const N: usize> PartialEq<[u8; N]> for Binary {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.0 == other
    }
}

impl<const N: usize> PartialEq<&[u8; N]> for Binary {
    fn eq(&self, other: &&[u8; N]) -> bool {
        &self.0 == other
    }
}

impl FromIterator<u8> for Binary {
    fn from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Self {
        Binary(iter.into_iter().collect())
    }
}

impl Extend<u8> for Binary {
    fn extend<I: IntoIterator<Item = u8>>(&mut self, iter: I) {
        self.0.extend(iter);
    }
}

impl IntoIterator for Binary {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a Binary {
    type Item = &'a u8;
    type IntoIter = std::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a mut Binary {
    type Item = &'a mut u8;
    type IntoIter = std::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_conversions() {
        // Binary from Vec<u8>
        let binary = Binary::from(vec![1u8, 2, 3]);
        assert_eq!(binary, Binary(vec![1, 2, 3]));

        // Binary from &[u8]
        let bytes: &[u8] = b"hello";
        let binary = Binary::from(bytes);
        assert_eq!(binary, Binary(b"hello".to_vec()));

        // Binary from byte literal &[u8; N]
        let binary = Binary::from(b"world");
        assert_eq!(binary, Binary(b"world".to_vec()));

        // Binary to Vec<u8>
        let binary = Binary(vec![1, 2, 3]);
        let vec: Vec<u8> = binary.clone().into();
        assert_eq!(vec, vec![1, 2, 3]);

        // Binary AsRef<[u8]>
        let binary = Binary(vec![4, 5, 6]);
        let slice: &[u8] = binary.as_ref();
        assert_eq!(slice, &[4, 5, 6]);

        // Binary Deref to [u8]
        let binary = Binary(vec![7, 8, 9]);
        assert_eq!(&*binary, &[7, 8, 9]);
        assert_eq!(binary.len(), 3);
        assert_eq!(binary[0], 7);

        // Binary AsMut<[u8]>
        let mut binary = Binary(vec![1, 2, 3]);
        let slice_mut: &mut [u8] = binary.as_mut();
        slice_mut[0] = 99;
        assert_eq!(binary.as_ref(), &[99, 2, 3]);

        // Binary DerefMut to [u8]
        let mut binary = Binary(vec![4, 5, 6]);
        binary[1] = 88;
        assert_eq!(&*binary, &[4, 88, 6]);

        // Binary from owned array [u8; N]
        let binary = Binary::from([1u8, 2, 3]);
        assert_eq!(binary, Binary(vec![1, 2, 3]));
    }

    #[test]
    fn test_binary_construction() {
        // Binary::new()
        let binary = Binary::new();
        assert_eq!(binary.len(), 0);
        assert!(binary.is_empty());

        // Binary::with_capacity()
        let binary = Binary::with_capacity(10);
        assert_eq!(binary.len(), 0);
        assert!(binary.capacity() >= 10);

        // Binary::default()
        let binary = Binary::default();
        assert_eq!(binary, Binary::new());
    }

    #[test]
    fn test_binary_methods() {
        let mut binary = Binary::new();

        // push
        binary.push(1);
        binary.push(2);
        binary.push(3);
        assert_eq!(binary.len(), 3);
        assert_eq!(&*binary, &[1, 2, 3]);

        // pop
        assert_eq!(binary.pop(), Some(3));
        assert_eq!(binary.len(), 2);
        assert_eq!(&*binary, &[1, 2]);

        // extend_from_slice
        binary.extend_from_slice(&[4, 5, 6]);
        assert_eq!(&*binary, &[1, 2, 4, 5, 6]);

        // truncate
        binary.truncate(3);
        assert_eq!(binary.len(), 3);
        assert_eq!(&*binary, &[1, 2, 4]);

        // clear
        binary.clear();
        assert_eq!(binary.len(), 0);
        assert!(binary.is_empty());

        // pop on empty
        assert_eq!(binary.pop(), None);
    }

    #[test]
    fn test_binary_iterators() {
        let binary = Binary::from(vec![1u8, 2, 3]);

        // IntoIterator for Binary (consuming)
        let vec: Vec<u8> = binary.clone().into_iter().collect();
        assert_eq!(vec, vec![1, 2, 3]);

        // IntoIterator for &Binary
        let sum: u8 = (&binary).into_iter().sum();
        assert_eq!(sum, 6);

        // IntoIterator for &mut Binary
        let mut binary = Binary::from(vec![1u8, 2, 3]);
        for byte in &mut binary {
            *byte *= 2;
        }
        assert_eq!(&*binary, &[2, 4, 6]);

        // FromIterator<u8>
        let binary: Binary = vec![7u8, 8, 9].into_iter().collect();
        assert_eq!(&*binary, &[7, 8, 9]);

        // Extend<u8>
        let mut binary = Binary::from(vec![1u8, 2]);
        binary.extend(vec![3, 4, 5]);
        assert_eq!(&*binary, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_binary_partial_eq() {
        let binary = Binary::from(vec![1u8, 2, 3]);

        // PartialEq<[u8]>
        assert_eq!(binary, [1, 2, 3]);
        assert_ne!(binary, [1, 2]);

        // PartialEq<&[u8]>
        let slice: &[u8] = &[1, 2, 3];
        assert_eq!(binary, slice);

        // PartialEq<Vec<u8>>
        assert_eq!(binary, vec![1u8, 2, 3]);
        assert_ne!(binary, vec![1u8, 2]);

        // PartialEq<[u8; N]>
        assert_eq!(binary, [1u8, 2, 3]);
        assert_ne!(binary, [1u8, 2, 3, 4]);

        // PartialEq<&[u8; N]>
        let arr: &[u8; 3] = &[1, 2, 3];
        assert_eq!(binary, arr);

        // Byte literals
        let binary = Binary::from(b"hello");
        assert_eq!(binary, b"hello");
        assert_eq!(binary, b"hello".as_slice());
        assert_ne!(binary, b"world");
    }

    #[test]
    fn test_binary_borrow() {
        use std::collections::HashMap;

        let mut map: HashMap<Binary, &str> = HashMap::new();
        let key = Binary::from(vec![1u8, 2, 3]);
        map.insert(key.clone(), "value");

        // Can lookup with &[u8] thanks to Borrow<[u8]>
        let lookup: &[u8] = &[1, 2, 3];
        assert_eq!(map.get(lookup), Some(&"value"));

        // Can lookup with byte literal
        let key2 = Binary::from(b"hello");
        map.insert(key2.clone(), "world");
        assert_eq!(map.get(b"hello".as_slice()), Some(&"world"));
    }
}
