use avenue::{AssetExt, BoxError, Process};

#[cfg(feature = "minify-js")]
pub struct MinifyJs<T>(pub T);

#[cfg(feature = "minify-js")]
impl<T: Process> Process for MinifyJs<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let mut buf = Vec::with_capacity(src.len());
        minify_js::minify(
            &minify_js::Session::new(),
            minify_js::TopLevelMode::Global,
            src,
            &mut buf,
        )
        .map_err(|err| format!("failed to minify js: {:?}", err))?;
        Ok(buf)
    }
}

#[cfg(feature = "minify-js")]
impl<T: AssetExt> AssetExt for MinifyJs<T> {
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        self.0.mime()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        self.0.path()
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

#[cfg(feature = "lightningcss")]
pub struct MinifyCss<T>(pub T);

#[cfg(feature = "lightningcss")]
impl<T: Process> Process for MinifyCss<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        use lightningcss::{
            printer::PrinterOptions,
            stylesheet::{MinifyOptions, ParserOptions, StyleSheet},
        };

        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let src_utf8 = std::str::from_utf8(src)?;
        let mut stylesheet =
            StyleSheet::parse(src_utf8, ParserOptions::default())
                .map_err(|err| format!("failed to parse css: {:?}", err))?;
        stylesheet.minify(MinifyOptions::default())?;
        let css = stylesheet.to_css(PrinterOptions {
            minify: true,
            ..Default::default()
        })?;
        Ok(css.code.into())
    }
}

#[cfg(feature = "lightningcss")]
impl<T: AssetExt> AssetExt for MinifyCss<T> {
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        self.0.mime()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        self.0.path()
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

#[cfg(feature = "minify-html")]
pub struct MinifyHtml<T>(pub T);

#[cfg(feature = "minify-html")]
impl<T: Process> Process for MinifyHtml<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let buf = minify_html::minify(src, &minify_html::Cfg::spec_compliant());
        Ok(buf)
    }
}

#[cfg(feature = "minify-html")]
impl<T: AssetExt> AssetExt for MinifyHtml<T> {
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        self.0.mime()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        self.0.path()
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        self.0.size_hint()
    }
}

pub enum Minify<T> {
    #[cfg(feature = "minify-js")]
    Js(MinifyJs<T>),
    #[cfg(feature = "lightningcss")]
    Css(MinifyCss<T>),
    #[cfg(feature = "minify-html")]
    Html(MinifyHtml<T>),
}

impl<T: Process> Process for Minify<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        match self {
            #[cfg(feature = "minify-js")]
            Minify::Js(minify_js) => minify_js.process_full(),
            #[cfg(feature = "lightningcss")]
            Minify::Css(minify_css) => minify_css.process_full(),
            #[cfg(feature = "minify-html")]
            Minify::Html(minify_html) => minify_html.process_full(),
        }
    }
}

impl<T: AssetExt> AssetExt for Minify<T> {
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        match self {
            Minify::Js(minify_js) => minify_js.mime(),
            Minify::Css(minify_css) => minify_css.mime(),
            Minify::Html(minify_html) => minify_html.mime(),
        }
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        match self {
            Minify::Js(minify_js) => minify_js.path(),
            Minify::Css(minify_css) => minify_css.path(),
            Minify::Html(minify_html) => minify_html.path(),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        match self {
            Minify::Js(minify_js) => minify_js.size_hint(),
            Minify::Css(minify_css) => minify_css.size_hint(),
            Minify::Html(minify_html) => minify_html.size_hint(),
        }
    }
}
