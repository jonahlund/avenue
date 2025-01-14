#[cfg(feature = "minify-js")]
mod js;
#[cfg(feature = "minify-js")]
pub use js::*;

#[cfg(feature = "minify-css")]
mod css;
#[cfg(feature = "minify-css")]
pub use css::*;

#[cfg(feature = "minify-html")]
mod html;
#[cfg(feature = "minify-html")]
pub use html::*;

use crate::{BoxError, Build};

#[must_use = "modifiers do nothing unless built"]
pub enum Minify<T> {
    #[cfg(feature = "minify-js")]
    Js(MinifyJs<T>),
    #[cfg(feature = "minify-css")]
    Css(MinifyCss<T>),
    #[cfg(feature = "minify-html")]
    Html(MinifyHtml<T>),
}

impl<T, Out, Err> Build for Minify<T>
where
    T: Build<Output = Out, Error = Err>,
    Out: AsRef<[u8]>,
    Err: Into<BoxError>,
{
    type Error = BoxError;
    type Output = Vec<u8>;

    #[inline]
    fn build(self) -> Result<Self::Output, Self::Error> {
        match self {
            #[cfg(feature = "minify-js")]
            Minify::Js(inner) => inner.build(),
            #[cfg(feature = "minify-css")]
            Minify::Css(inner) => inner.build(),
            #[cfg(feature = "minify-html")]
            Minify::Html(inner) => inner.build(),
        }
    }
}
