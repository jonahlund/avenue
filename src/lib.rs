#![no_std]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{borrow::Cow, boxed::Box, string::String, vec::Vec};
use core::{convert::Infallible, error::Error};

pub type BoxError = Box<dyn Error>;

pub trait Process {
    type Error: Into<BoxError>;
    type Output: AsRef<[u8]>;

    fn process_full(self) -> Result<Self::Output, Self::Error>;
}

pub trait ProcessRead {
    type Error: Into<BoxError>;

    fn process_read(&mut self) -> Result<usize, Self::Error>;
}

pub trait AssetExt {
    #[cfg(feature = "mime")]
    fn mime(&self) -> Option<mime::Mime>;

    #[cfg(feature = "std")]
    fn path(&self) -> Option<&std::path::Path>;

    fn size_hint(&self) -> Option<usize>;
}

macro_rules! impl_forward {
    ($($ty:ty)+) => {
        $(
            impl Process for $ty {
                type Error = Infallible;
                type Output = Self;

                #[inline]
                fn process_full(self) -> Result<Self::Output, Self::Error> {
                    Ok(self)
                }
            }
        )+
    }
}

impl_forward!(String &str Vec<u8> &[u8]);

impl<T: Process> Process for Box<T> {
    type Error = T::Error;
    type Output = T::Output;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        T::process_full(*self)
    }
}

#[cfg(feature = "std")]
impl Process for std::fs::File {
    type Error = std::io::Error;
    type Output = Vec<u8>;

    fn process_full(mut self) -> Result<Self::Output, Self::Error> {
        use std::io::Read as _;
        let mut buf = Vec::new();
        self.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "std")]
pub struct BufAsset<'p, 'c> {
    pub path: Cow<'p, std::path::Path>,
    pub contents: Cow<'c, [u8]>,
}

#[cfg(feature = "std")]
impl<'p, 'c> BufAsset<'p, 'c> {
    #[inline]
    pub fn new<P: Into<Cow<'p, std::path::Path>>, C: Into<Cow<'c, [u8]>>>(
        path: P,
        contents: C,
    ) -> Self {
        Self {
            path: path.into(),
            contents: contents.into(),
        }
    }
}

#[cfg(feature = "std")]
impl AsRef<[u8]> for BufAsset<'_, '_> {
    fn as_ref(&self) -> &[u8] {
        &self.contents
    }
}

#[cfg(feature = "std")]
impl<'p, 'c> Process for BufAsset<'p, 'c> {
    type Error = Infallible;
    type Output = Cow<'c, [u8]>;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        Ok(self.contents)
    }
}

#[cfg(feature = "std")]
impl AssetExt for BufAsset<'_, '_> {
    #[cfg(feature = "mime")]
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        mime_guess::MimeGuess::from_path(&self.path).first()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        Some(&self.path)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.contents.len())
    }
}

#[cfg(feature = "std")]
pub struct FileAsset<'p> {
    pub path: Cow<'p, std::path::Path>,
}

#[cfg(feature = "std")]
impl<'p> FileAsset<'p> {
    #[inline]
    pub fn new<P: Into<Cow<'p, std::path::Path>>>(path: P) -> Self {
        Self { path: path.into() }
    }
}

#[cfg(feature = "std")]
impl Process for FileAsset<'_> {
    type Error = std::io::Error;
    type Output = Vec<u8>;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        std::fs::read(self.path)
    }
}

#[cfg(feature = "std")]
impl AssetExt for FileAsset<'_> {
    #[cfg(feature = "mime")]
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        mime_guess::MimeGuess::from_path(&self.path).first()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        Some(&self.path)
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        std::fs::metadata(&self.path).map(|m| m.len() as usize).ok()
    }
}

#[cfg(feature = "std")]
pub enum Asset<'p, 'c> {
    Buf(BufAsset<'p, 'c>),
    File(FileAsset<'p>),
}

#[cfg(feature = "std")]
impl<'p, 'c> Asset<'p, 'c> {
    #[inline]
    pub fn new_file<P: Into<Cow<'p, std::path::Path>>>(path: P) -> Self {
        Self::File(FileAsset::new(path))
    }

    #[inline]
    pub fn new_buf<
        P: Into<Cow<'p, std::path::Path>>,
        C: Into<Cow<'c, [u8]>>,
    >(
        path: P,
        contents: C,
    ) -> Self {
        Self::Buf(BufAsset::new(path, contents))
    }
}

#[cfg(feature = "std")]
impl<'c> Process for Asset<'_, 'c> {
    type Error = BoxError;
    type Output = Cow<'c, [u8]>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        match self {
            Asset::Buf(buf_asset) => {
                buf_asset.process_full().map_err(Into::into)
            }
            Asset::File(file_asset) => file_asset
                .process_full()
                .map_err(Into::into)
                .map(Into::into),
        }
    }
}

#[cfg(feature = "std")]
impl AssetExt for Asset<'_, '_> {
    #[cfg(feature = "mime")]
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        match self {
            Asset::Buf(buf_asset) => buf_asset.mime(),
            Asset::File(file_asset) => file_asset.mime(),
        }
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        match self {
            Asset::Buf(buf_asset) => buf_asset.path(),
            Asset::File(file_asset) => file_asset.path(),
        }
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        match self {
            Asset::Buf(buf_asset) => buf_asset.size_hint(),
            Asset::File(file_asset) => file_asset.size_hint(),
        }
    }
}

#[cfg(feature = "either")]
impl<L: Process, R: Process> Process for either::Either<L, R> {
    type Error = BoxError;
    type Output = either::Either<L::Output, R::Output>;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        match self {
            Self::Left(left) => Ok(either::Either::Left(
                left.process_full().map_err(Into::into)?,
            )),
            Self::Right(right) => Ok(either::Either::Right(
                right.process_full().map_err(Into::into)?,
            )),
        }
    }
}

#[cfg(feature = "either")]
impl<L: AssetExt, R: AssetExt> AssetExt for either::Either<L, R> {
    #[cfg(feature = "mime")]
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        either::for_both!(*self, ref inner => inner.mime())
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        either::for_both!(*self, ref inner => inner.path())
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        either::for_both!(*self, ref inner => inner.size_hint())
    }
}
