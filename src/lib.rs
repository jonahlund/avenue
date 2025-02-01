#![no_std]

mod asset;
mod ext;

pub use asset::{Asset, BufAsset, FileAsset};
pub use ext::AssetExt;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

use alloc::{boxed::Box, string::String, vec::Vec};
use core::{convert::Infallible, error::Error};

pub type BoxError = Box<dyn Error>;

pub trait Process {
    type Error: Into<BoxError>;
    type Output: AsRef<[u8]>;

    fn process_full(self) -> Result<Self::Output, Self::Error>;
}

pub trait ProcessRead {
    type Error: Into<BoxError>;

    fn process_read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error>;
}

macro_rules! impl_move {
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

impl_move!(String &str Vec<u8> &[u8]);

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

    #[cfg(feature = "std")]
    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        either::for_both!(*self, ref inner => inner.path())
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        either::for_both!(*self, ref inner => inner.size_hint())
    }
}
