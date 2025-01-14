use std::str::from_utf8;

use lightningcss::{
    printer::PrinterOptions,
    stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
};

use crate::{BoxError, Build, Builder};

#[must_use = "modifiers do nothing unless built"]
pub struct MinifyCss<A> {
    inner: A,
}

impl<T> MinifyCss<T> {
    #[inline]
    pub fn new(inner: T) -> Self {
        Self { inner }
    }
}

impl<T, Out, Err> Build for MinifyCss<T>
where
    T: Build<Output = Out, Error = Err>,
    Out: AsRef<[u8]>,
    Err: Into<BoxError>,
{
    type Error = BoxError;
    type Output = Vec<u8>;

    fn build(self) -> Result<Self::Output, Self::Error> {
        let contents = self.inner.build().map_err(Into::into)?;

        fn inner(contents: &str) -> Result<Vec<u8>, BoxError> {
            let mut stylesheet =
                StyleSheet::parse(contents, ParserOptions::default()).unwrap();
            stylesheet.minify(MinifyOptions::default())?;
            let css = stylesheet.to_css(PrinterOptions {
                minify: true,
                ..Default::default()
            })?;
            Ok(css.code.into())
        }

        inner(from_utf8(contents.as_ref())?)
    }
}
