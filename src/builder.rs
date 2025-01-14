use crate::*;

/// This trait defines certain operations you can do on an asset, but not how
/// the contents should be read.
pub trait Builder {
    /// Compresses this asset using a brotli encoder.
    #[cfg(feature = "compress-br")]
    #[inline]
    fn compress_br(self) -> CompressBrotli<Self>
    where
        Self: Sized,
    {
        CompressBrotli::new(self)
    }

    /// Compresses this asset using a deflate encoder.
    #[cfg(feature = "compress-deflate")]
    #[inline]
    fn compress_deflate(self) -> CompressDeflate<Self>
    where
        Self: Sized,
    {
        CompressDeflate::new(self)
    }

    /// Compresses this asset using a gzip encoder.
    #[cfg(feature = "compress-gzip")]
    #[inline]
    fn compress_gzip(self) -> CompressGzip<Self>
    where
        Self: Sized,
    {
        CompressGzip::new(self)
    }

    /// Compresses this asset using a zstd encoder.
    #[cfg(feature = "compress-zstd")]
    #[inline]
    fn compress_zstd(self) -> CompressZstd<Self>
    where
        Self: Sized,
    {
        CompressZstd::new(self)
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
        Self: Sized + Unmodified,
    {
        MinifyJs::new(self)
    }

    /// Minifies this asset using [`lightningcss`].
    ///
    /// This may fail if the contents are not valid CSS.
    ///
    /// [`lightningcss`](lightningcss)
    #[cfg(feature = "minify-css")]
    #[inline]
    fn minify_css(self) -> MinifyCss<Self>
    where
        Self: Sized + Unmodified,
    {
        MinifyCss::new(self)
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
        Self: Sized + Unmodified,
    {
        MinifyHtml::new(self)
    }

    // #[cfg(feature = "minify-json")]
    // #[inline]
    // fn minify_json(self) -> MinifyJson<Self>
    // where
    //     Self: Sized + Unmodified,
    // {
    //     MinifyJson::new(self)
    // }

    /// Attempts to minify this asset based on its mime type.
    #[cfg(any(
        feature = "minify-js",
        feature = "minify-css",
        feature = "minify-html",
        feature = "minify-json"
    ))]
    #[inline]
    fn minify(self) -> Minify<Self>
    where
        Self: Sized + Unmodified + AssetExt,
    {
        let mime = self.mime();
        let subtype = mime.as_ref().map(|m| m.subtype());

        match subtype {
            #[cfg(feature = "minify-js")]
            Some(mime::JAVASCRIPT) => Minify::Js(self.minify_js()),
            #[cfg(feature = "minify-css")]
            Some(mime::CSS) => Minify::Css(self.minify_css()),
            #[cfg(feature = "minify-html")]
            Some(mime::HTML) => Minify::Html(self.minify_html()),
            _ => todo!(),
        }
    }
}

impl<T: Build> Builder for T {}
