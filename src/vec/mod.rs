use core::{
    fmt::{Debug, Formatter},
    hash::Hash,
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::Format;

#[cfg(feature = "proptest")]
mod proptest;
mod serde;

/// A wrapper around `Vec<u8>` that allows control over the serialization format
pub struct Bytes<F: Format> {
    inner: Vec<u8>,
    _marker: PhantomData<F>,
}

impl<F: Format> Debug for Bytes<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Bytes")
            .field("inner", &self.inner)
            .finish_non_exhaustive()
    }
}

impl<F: Format> Clone for Bytes<F> {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            _marker: PhantomData,
        }
    }
}

impl<F: Format, G: Format> PartialEq<Bytes<F>> for Bytes<G> {
    fn eq(&self, other: &Bytes<F>) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<F: Format> Eq for Bytes<F> {}

impl<F: Format> PartialEq<[u8]> for Bytes<F> {
    fn eq(&self, other: &[u8]) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format, const N: usize> PartialEq<[u8; N]> for Bytes<F> {
    fn eq(&self, other: &[u8; N]) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format> PartialEq<Vec<u8>> for Bytes<F> {
    fn eq(&self, other: &Vec<u8>) -> bool {
        self.inner.eq(other)
    }
}

impl<F: Format> From<Vec<u8>> for Bytes<F> {
    #[inline]
    fn from(inner: Vec<u8>) -> Self {
        Self {
            inner,
            _marker: PhantomData,
        }
    }
}

impl<F: Format> FromIterator<u8> for Bytes<F> {
    #[inline]
    fn from_iter<T: IntoIterator<Item = u8>>(iter: T) -> Self {
        Vec::<u8>::from_iter(iter).into()
    }
}

impl<F: Format> IntoIterator for Bytes<F> {
    type Item = u8;
    type IntoIter = std::vec::IntoIter<u8>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<F: Format> AsRef<[u8]> for Bytes<F> {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.inner.as_ref()
    }
}

impl<F: Format> AsMut<[u8]> for Bytes<F> {
    #[inline]
    fn as_mut(&mut self) -> &mut [u8] {
        self.inner.as_mut()
    }
}

impl<F: Format> AsRef<Vec<u8>> for Bytes<F> {
    #[inline]
    fn as_ref(&self) -> &Vec<u8> {
        &self.inner
    }
}

impl<F: Format> AsMut<Vec<u8>> for Bytes<F> {
    #[inline]
    fn as_mut(&mut self) -> &mut Vec<u8> {
        &mut self.inner
    }
}

impl<F: Format> Deref for Bytes<F> {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner.deref()
    }
}

impl<F: Format> DerefMut for Bytes<F> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.inner.deref_mut()
    }
}

impl<F: Format> PartialOrd for Bytes<F> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Vec::<u8>::partial_cmp(&self.inner, &other.inner)
    }
}

impl<F: Format> Ord for Bytes<F> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        Vec::<u8>::cmp(&self.inner, &other.inner)
    }
}

impl<F: Format> Hash for Bytes<F> {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        Vec::<u8>::hash(&self.inner, state)
    }
}

impl<F: Format> Bytes<F> {
    /// Create a new, empty `Bytes<F>`
    ///
    /// Analagous to [`Vec::new`]
    #[inline]
    pub fn new() -> Self {
        Vec::<u8>::new().into()
    }

    /// Create a new, empty `Bytes<F>` with at least the specified capacity
    ///
    /// Analagous to [`Vec::with_capacity`]
    #[inline]
    pub fn with_capacity(capacity: usize) -> Self {
        Vec::<u8>::with_capacity(capacity).into()
    }

    /// Consume `self` and return the underlying `Vec<u8>`
    #[inline]
    pub fn into_inner(self) -> Vec<u8> {
        self.inner
    }

    /// Convert `self` to a different format:
    /// ```
    /// # use bytekind::*;
    /// let bytes: Bytes<Plain> = Bytes::new();
    /// let bytes_as_hex: Bytes<HexString> = bytes.convert();
    /// ```
    pub fn convert<G: Format>(self) -> Bytes<G> {
        Bytes {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Plain;

    use super::*;

    static_assertions::assert_impl_all!(Bytes<Plain>: Debug, Clone, PartialEq, Eq);
}
