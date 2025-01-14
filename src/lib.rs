//! Avenue - Asset manager

#![allow(async_fn_in_trait)]

use std::{convert::Infallible, time::SystemTime};

use mime::Mime;

#[cfg(any(
    feature = "compress-br",
    feature = "compress-deflate",
    feature = "compress-gzip",
    feature = "compress-zstd"
))]
mod compress;
#[cfg(any(
    feature = "compress-br",
    feature = "compress-deflate",
    feature = "compress-gzip",
    feature = "compress-zstd"
))]
pub use compress::*;
#[cfg(any(
    feature = "minify-js",
    feature = "minify-css",
    feature = "minify-html"
))]
mod minify;
#[cfg(any(
    feature = "minify-js",
    feature = "minify-css",
    feature = "minify-html"
))]
pub use minify::*;

#[cfg(feature = "fs")]
pub mod fs;

mod builder;
pub use builder::*;

pub type BoxError = Box<dyn std::error::Error + 'static>;

/// A marker trait indicating that the asset has not been modified.
///
/// This is mainly used for type-safety, ensuring you don't do operations in the
/// wrong order, like minifying an asset after it was compressed. Thus certain
/// operations like minification require an `Unmodified` bound which is
/// implemented on types that acts as the source, like `File`, but not for
/// types that may have done some modification on the contents, like `Compress`
/// or `Minify`.
pub trait Unmodified {}

/// An asset that may contain additional metadata.
pub trait AssetExt {
    /// Returns the mime type (if any).
    fn mime(&self) -> Option<Mime>;

    /// Returns the last modification time (if any).
    fn last_modified(&self) -> Option<SystemTime>;

    /// Returns the content length (if any).
    fn content_length(&self) -> Option<u64>;
}

/// A trait for defining how an asset should be processed.
///
/// This is the most basic processing trait that will do the processing in full,
/// returning the processed output. This will usually yield higher performance
/// than other processing traits such as `BuildRead` whenever you are
/// processing an entire asset.
pub trait Build {
    type Output;
    type Error;

    /// Builds an asset and returns the output.
    fn build(self) -> Result<Self::Output, Self::Error>;
}

macro_rules! impl_simple {
    ($($ty:ty)*) => {
        $(
            impl Unmodified for $ty {}

            impl Build for $ty {
                type Error = Infallible;
                type Output = Self;

                #[inline]
                fn build(self) -> Result<Self::Output, Self::Error> {
                    Ok(self)
                }
            }
        )*
    };
}

impl_simple! {
    &str String
    &[u8] Vec<u8>
}

pub trait Embed<T: 'static> {
    const ASSETS: &[T];

    fn get(key: &str) -> Option<&T>;
}
