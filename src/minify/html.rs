use minify_html::{Cfg, minify};

use crate::{BoxError, Build, Builder};

#[must_use = "modifiers do nothing unless processed"]
pub struct MinifyHtml<T> {
    inner: T,
}

impl<T> MinifyHtml<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, Out, Err> Build for MinifyHtml<T>
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
            Ok(minify(contents, &Cfg::spec_compliant()))
        }

        inner(contents.as_ref())
    }
}
