use alloc::{borrow::Cow, vec::Vec};
use core::{convert::Infallible, mem};

use crate::{AssetExt, BoxError, Process};

#[derive(Debug, Clone)]
pub struct BufAsset<'c, K> {
    /// A key identifying this asset.
    ///
    /// This will typically be a relative logical path for the asset, but it
    /// could be anything.
    pub key: K,
    /// The asset contents.
    pub contents: Cow<'c, [u8]>,
}

impl<'c, K> BufAsset<'c, K> {
    #[inline]
    pub fn new<C: Into<Cow<'c, [u8]>>>(key: K, contents: C) -> Self {
        Self {
            key,
            contents: contents.into(),
        }
    }

    #[cfg(feature = "std")]
    pub fn into_file<'p, P: Into<Cow<'p, std::path::Path>>>(
        self,
        path: P,
    ) -> std::io::Result<FileAsset<'p, K>> {
        let path = path.into();
        std::fs::write(&path, self.contents)?;
        Ok(FileAsset::new(self.key, path))
    }
}

impl<K> AsRef<[u8]> for BufAsset<'_, K> {
    fn as_ref(&self) -> &[u8] {
        &self.contents
    }
}

impl<'c, K> Process for BufAsset<'c, K> {
    type Error = Infallible;
    type Output = Cow<'c, [u8]>;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        Ok(self.contents)
    }
}

#[cfg(feature = "std")]
impl<K> AssetExt for BufAsset<'_, K>
where
    K: AsRef<std::path::Path>,
{
    #[cfg(feature = "mime")]
    #[inline]
    fn mime(&self) -> Option<mime::Mime> {
        mime_guess::MimeGuess::from_path(&self.key).first()
    }

    #[inline]
    fn path(&self) -> Option<&std::path::Path> {
        Some(self.key.as_ref())
    }

    #[inline]
    fn size_hint(&self) -> Option<usize> {
        Some(self.contents.len())
    }
}

#[cfg(feature = "std")]
#[derive(Debug, Clone)]
pub struct FileAsset<'p, K> {
    /// A key identifying this asset.
    ///
    /// This will typically be a relative logical path for the asset, but it
    /// could be anything.
    pub key: K,
    /// A path to an existing file.
    ///
    /// Unlike `key`, this should be a valid file path.
    pub path: Cow<'p, std::path::Path>,
}

#[cfg(feature = "std")]
impl<'p, K> FileAsset<'p, K> {
    #[inline]
    pub fn new<P: Into<Cow<'p, std::path::Path>>>(key: K, path: P) -> Self {
        Self {
            key,
            path: path.into(),
        }
    }

    pub fn into_buf(self) -> std::io::Result<BufAsset<'static, K>> {
        let contents = std::fs::read(&self.path)?;
        Ok(BufAsset {
            key: self.key,
            contents: contents.into(),
        })
    }
}

#[cfg(feature = "std")]
impl<K> Process for FileAsset<'_, K> {
    type Error = std::io::Error;
    type Output = Vec<u8>;

    #[inline]
    fn process_full(self) -> Result<Self::Output, Self::Error> {
        std::fs::read(self.path)
    }
}

#[cfg(feature = "std")]
impl<K> AssetExt for FileAsset<'_, K> {
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

#[derive(Debug, Clone)]
pub enum Asset<'p, 'c, K> {
    Buf(BufAsset<'c, K>),
    #[cfg(feature = "std")]
    File(FileAsset<'p, K>),
}

impl<'p, 'c, K> Asset<'p, 'c, K> {
    #[cfg(feature = "std")]
    #[inline]
    pub fn new_file<P: Into<Cow<'p, std::path::Path>>>(
        key: K,
        path: P,
    ) -> Self {
        Self::File(FileAsset::new(key, path))
    }

    #[inline]
    pub fn new_buf<C: Into<Cow<'c, [u8]>>>(key: K, contents: C) -> Self {
        Self::Buf(BufAsset::new(key, contents))
    }

    #[inline]
    pub const fn key(&self) -> &K {
        match self {
            Asset::Buf(BufAsset { key, .. }) => key,
            #[cfg(feature = "std")]
            Asset::File(FileAsset { key, .. }) => key,
        }
    }

    #[cfg(feature = "std")]
    #[inline]
    pub const fn is_file(&self) -> bool {
        matches!(self, Self::File(_))
    }

    #[inline]
    pub const fn is_buf(&self) -> bool {
        matches!(self, Self::Buf(_))
    }
}

impl<'p, 'c, K: Default> Asset<'p, 'c, K> {
    pub fn into_buf(self) -> std::io::Result<BufAsset<'c, K>> {
        match self {
            Asset::Buf(buf) => Ok(buf),
            Asset::File(file) => file.into_buf(),
        }
    }

    pub fn ensure_buf(&mut self) -> std::io::Result<()> {
        if let Asset::File(asset) = self {
            let tmp = FileAsset {
                key: Default::default(),
                path: Default::default(),
            };
            let asset = mem::replace(asset, tmp);
            *self = asset.into_buf()?.into();
        }
        Ok(())
    }

    #[cfg(feature = "std")]
    pub fn into_file<P: Into<Cow<'p, std::path::Path>>>(
        self,
        path: P,
    ) -> std::io::Result<FileAsset<'p, K>> {
        match self {
            Asset::Buf(buf) => buf.into_file(path),
            Asset::File(file) => Ok(file),
        }
    }

    #[cfg(feature = "std")]
    pub fn ensure_file<P: Into<Cow<'p, std::path::Path>>>(
        &mut self,
        path: P,
    ) -> std::io::Result<()> {
        if let Asset::Buf(asset) = self {
            let tmp = BufAsset {
                key: Default::default(),
                contents: Default::default(),
            };
            let asset = mem::replace(asset, tmp);
            *self = asset.into_file(path)?.into();
        }
        Ok(())
    }
}

impl<'c, K> Process for Asset<'_, 'c, K> {
    type Error = BoxError;
    type Output = Cow<'c, [u8]>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        match self {
            Asset::Buf(buf_asset) => {
                buf_asset.process_full().map_err(Into::into)
            }
            #[cfg(feature = "std")]
            Asset::File(file_asset) => file_asset
                .process_full()
                .map_err(Into::into)
                .map(Into::into),
        }
    }
}

#[cfg(feature = "std")]
impl<K> AssetExt for Asset<'_, '_, K>
where
    K: AsRef<std::path::Path>,
{
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

impl<'c, K> From<BufAsset<'c, K>> for Asset<'_, 'c, K> {
    fn from(value: BufAsset<'c, K>) -> Self {
        Self::Buf(value)
    }
}

#[cfg(feature = "std")]
impl<'p, K> From<FileAsset<'p, K>> for Asset<'p, '_, K> {
    fn from(value: FileAsset<'p, K>) -> Self {
        Self::File(value)
    }
}
