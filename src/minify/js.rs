use minify_js::{Session, TopLevelMode, minify};

use crate::{BoxError, Build, Builder};

#[must_use = "builders do nothing unless processed"]
pub struct MinifyJs<T> {
    inner: T,
}

impl<T> MinifyJs<T> {
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, Out, Err> Build for MinifyJs<T>
where
    T: Build<Output = Out, Error = Err>,
    Out: AsRef<[u8]>,
    Err: Into<BoxError>,
{
    type Error = BoxError;
    type Output = Vec<u8>;

    fn build(self) -> Result<Vec<u8>, Self::Error> {
        let contents = self.inner.build().map_err(Into::into)?;
        fn inner(contents: &[u8]) -> Result<Vec<u8>, BoxError> {
            let mut output = Vec::with_capacity(contents.len());
            minify(
                &Session::new(),
                TopLevelMode::Global,
                contents,
                &mut output,
            )
            .unwrap();
            Ok(output)
        }
        inner(contents.as_ref())
    }
}
