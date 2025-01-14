use crate::Builder;

#[must_use = "modifiers do nothing unless processed"]
pub struct CompressZstd<T> {
    inner: T,
}

impl<T> CompressZstd<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
