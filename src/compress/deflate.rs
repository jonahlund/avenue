use std::io::Read;

use flate2::{Compression, bufread::DeflateEncoder};

use crate::{BoxError, Build};

#[must_use = "modifiers do nothing unless built"]
pub struct CompressDeflate<T> {
    inner: T,
}

impl<T> CompressDeflate<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, Out, Err> Build for CompressDeflate<T>
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
            DeflateEncoder::new(contents, Compression::new(4))
                .read_to_end(&mut output)?;
            Ok(output)
        }

        inner(contents.as_ref())
    }
}
