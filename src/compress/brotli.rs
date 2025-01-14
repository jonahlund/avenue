use std::io::Write as _;

use brotli::CompressorWriter;

use crate::{BoxError, Build};

#[must_use = "modifiers do nothing unless built"]
pub struct CompressBrotli<A> {
    inner: A,
}

impl<T> CompressBrotli<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, Out, Err> Build for CompressBrotli<T>
where
    T: Build<Output = Out, Error = Err>,
    Out: AsRef<[u8]>,
    Err: Into<BoxError>,
{
    type Error = BoxError;
    type Output = Vec<u8>;

    fn build(self) -> Result<Self::Output, Self::Error> {
        let contents = self.inner.build().map_err(Into::into)?;

        fn inner(contents: &[u8]) -> Result<Vec<u8>, BoxError> {
            let mut output = Vec::with_capacity(contents.len() / 2);
            CompressorWriter::new(&mut output, contents.len(), 4, 22)
                .write_all(contents)?;
            Ok(output)
        }

        inner(contents.as_ref())
    }
}
