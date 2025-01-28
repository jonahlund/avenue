use avenue::{AssetExt, Process};
use either::Either;

use crate::*;

/// This trait defines certain operations you can do on an asset, but not how
/// the contents should be read.
pub trait Builder {
    /// Compresses this asset using a brotli encoder.
    #[cfg(feature = "brotli")]
    #[inline]
    fn compress_brotli(self) -> CompressBrotli<Self>
    where
        Self: Sized,
    {
        CompressBrotli(self)
    }

    /// Compresses this asset using a deflate encoder.
    #[cfg(feature = "flate2")]
    #[inline]
    fn compress_deflate(self) -> CompressDeflate<Self>
    where
        Self: Sized,
    {
        CompressDeflate(self)
    }

    /// Compresses this asset using a gzip encoder.
    #[cfg(feature = "flate2")]
    #[inline]
    fn compress_gzip(self) -> CompressGzip<Self>
    where
        Self: Sized,
    {
        CompressGzip(self)
    }

    /// Compresses this asset using a zstd encoder.
    #[cfg(feature = "zstd")]
    #[inline]
    fn compress_zstd(self) -> CompressZstd<Self>
    where
        Self: Sized,
    {
        CompressZstd(self)
    }

    /// Minifies this asset using [`minify-js`].
    ///
    /// This may fail if the contents are not valid JavaScript.
    ///
    /// [`minify-js`](minify_js)
    #[cfg(feature = "minify-js")]
    #[inline]
    fn minify_js(self) -> MinifyJs<Self>
    where
        Self: Sized,
    {
        MinifyJs(self)
    }

    /// Minifies this asset using [`lightningcss`].
    ///
    /// This may fail if the contents are not valid CSS.
    ///
    /// [`lightningcss`](lightningcss)
    #[cfg(feature = "lightningcss")]
    #[inline]
    fn minify_css(self) -> MinifyCss<Self>
    where
        Self: Sized,
    {
        MinifyCss(self)
    }

    /// Minifies this asset using [`minify-html`].
    ///
    /// This may fail if the contents are not valid HTML.
    ///
    /// [`minify-html`](minify_html)
    #[cfg(feature = "minify-html")]
    #[inline]
    fn minify_html(self) -> MinifyHtml<Self>
    where
        Self: Sized,
    {
        MinifyHtml(self)
    }

    /// Attempts to minify this asset based on its mime type.
    #[cfg(any(
        feature = "minify-js",
        feature = "lightningcss",
        feature = "minify-html"
    ))]
    fn minify_or_fallback(self) -> Either<Minify<Self>, Self>
    where
        Self: Sized + AssetExt,
    {
        let mime = self.mime();
        let subtype = mime.as_ref().map(|m| m.subtype());

        match subtype {
            #[cfg(feature = "minify-js")]
            Some(mime::JAVASCRIPT) => {
                Either::Left(Minify::Js(self.minify_js()))
            }
            #[cfg(feature = "lightningcss")]
            Some(mime::CSS) => Either::Left(Minify::Css(self.minify_css())),
            #[cfg(feature = "minify-html")]
            Some(mime::HTML) => Either::Left(Minify::Html(self.minify_html())),
            _ => Either::Right(self),
        }
    }
}

impl<T: Process> Builder for T {}
