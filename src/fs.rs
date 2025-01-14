use std::{fs, io, os::unix::fs::MetadataExt, path::PathBuf, time::SystemTime};

use mime::Mime;
use mime_guess::MimeGuess;

use crate::{AssetExt, Build, Unmodified};

pub struct File {
    path: PathBuf,
}

impl File {
    #[inline]
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self { path: path.into() }
    }
}

impl Unmodified for File {}

impl AssetExt for File {
    #[inline]
    fn mime(&self) -> Option<Mime> {
        MimeGuess::from_path(&self.path).first()
    }

    #[inline]
    fn last_modified(&self) -> Option<SystemTime> {
        fs::metadata(&self.path).and_then(|m| m.modified()).ok()
    }

    #[inline]
    fn content_length(&self) -> Option<u64> {
        fs::metadata(&self.path).map(|m| m.size()).ok()
    }
}

impl Build for File {
    type Error = io::Error;
    type Output = Vec<u8>;

    #[inline]
    fn build(self) -> Result<Self::Output, Self::Error> {
        fs::read(&self.path)
    }
}
