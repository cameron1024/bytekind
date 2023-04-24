use core::fmt::{Debug, Formatter};
use core::hash::Hash;
use core::marker::PhantomData;

use crate::Format;

#[cfg(feature = "proptest")]
mod proptest;
mod serde;

/// A wrapper around `[u8; N]` that allows control over the serialization format
#[derive(Copy)]
pub struct ByteArray<F: Format, const N: usize> {
    inner: [u8; N],
    _marker: PhantomData<F>,
}

impl<F: Format, const N: usize> Debug for ByteArray<F, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("ByteArray")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}
impl<F: Format, const N: usize> Clone for ByteArray<F, N> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

impl<F: Format, G: Format, const N: usize> PartialEq<ByteArray<F, N>> for ByteArray<G, N> {
    fn eq(&self, other: &ByteArray<F, N>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<F: Format, const N: usize> Eq for ByteArray<F, N> {}

impl<F: Format, const N: usize> PartialEq<&[u8; N]> for ByteArray<F, N> {
    fn eq(&self, other: &&[u8; N]) -> bool {
        self.inner.eq(*other)
    }
}

impl<F: Format, const N: usize> PartialEq<[u8; N]> for ByteArray<F, N> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format, const N: usize> PartialEq<&[u8]> for ByteArray<F, N> {
    fn eq(&self, other: &&[u8]) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format, const N: usize> PartialEq<[u8]> for ByteArray<F, N> {
    fn eq(&self, other: &[u8]) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format, const N: usize> PartialOrd for ByteArray<F, N> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        <[u8; N]>::partial_cmp(&self.inner, &other.inner)
    }
}

impl<F: Format, const N: usize> Ord for ByteArray<F, N> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        <[u8; N]>::cmp(&self.inner, &other.inner)
    }
}

impl<F: Format, const N: usize> From<[u8; N]> for ByteArray<F, N> {
    fn from(inner: [u8; N]) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<F: Format, const N: usize> From<ByteArray<F, N>> for [u8; N] {
    fn from(array: ByteArray<F, N>) -> Self {
        array.inner
    }
}

impl<F: Format, const N: usize> Hash for ByteArray<F, N> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        <[u8; N]>::hash(&self.inner, state);
    }
}

impl<F: Format, const N: usize> ByteArray<F, N> {
    /// Consume `self` and return the underlying `[u8; N]`
    #[inline]
    pub fn into_inner(self) -> [u8; N] {
        self.inner
    }
}

impl<F: Format, const N: usize> AsRef<[u8]> for ByteArray<F, N> {
    fn as_ref(&self) -> &[u8] {
        &self.inner
    }
}

impl<F: Format, const N: usize> AsMut<[u8]> for ByteArray<F, N> {
    fn as_mut(&mut self) -> &mut [u8] {
        &mut self.inner
    }
}

impl<F: Format, const N: usize> AsRef<[u8; N]> for ByteArray<F, N> {
    fn as_ref(&self) -> &[u8; N] {
        &self.inner
    }
}

impl<F: Format, const N: usize> AsMut<[u8; N]> for ByteArray<F, N> {
    fn as_mut(&mut self) -> &mut [u8; N] {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{from_value, json};

    use crate::{HexString, Plain};

    use super::*;

    #[test]
    fn check_eq_impls() {
        let byte_array: ByteArray<Plain, 4> = ByteArray::from([0, 1, 2, 3]);

        let first = byte_array == [0, 1, 2, 3];
        assert!(first);

        let slice: &[u8] = &[0, 1, 2, 3];
        let second = byte_array == slice;
        assert!(second);

        let array_ref: &[u8; 4] = &[0, 1, 2, 3];
        let third = byte_array == array_ref;
        assert!(third);

        let fourth = byte_array == byte_array;
        assert!(fourth);
    }

    #[test]
    fn parses_with_leading_0x() {
        let byte_array: ByteArray<HexString, 1> = from_value(json!("0x00")).unwrap();
        assert_eq!(byte_array, [0]);
    }
}
