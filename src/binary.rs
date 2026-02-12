use std::ops::{Deref, DerefMut};

/// A wrapper for [`Vec<u8>`]... at least until Rust supports specialization.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Binary(pub Vec<u8>);

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
    }
}
