#[must_use = "builders do nothing unless built"]
pub struct CompressGzip<T> {
    inner: T,
}

impl<T> CompressGzip<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}
