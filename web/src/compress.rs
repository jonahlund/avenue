use std::io::Read;

use avenue::{BoxError, Process};

#[cfg(feature = "brotli")]
pub struct CompressBrotli<T>(pub T);

#[cfg(feature = "brotli")]
impl<T: Process> Process for CompressBrotli<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let mut src = out.as_ref();
        let mut buf = Vec::with_capacity(src.len());
        brotli::BrotliCompress(
            &mut src,
            &mut buf,
            &brotli::enc::BrotliEncoderParams::default(),
        )?;
        Ok(buf)
    }
}

#[cfg(feature = "flate2")]
pub struct CompressDeflate<T>(pub T);

#[cfg(feature = "flate2")]
impl<T: Process> Process for CompressDeflate<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Self::Output, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let mut buf = Vec::with_capacity(src.len());
        let mut enc =
            flate2::bufread::DeflateEncoder::new(src, Default::default());
        enc.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "flate2")]
pub struct CompressGzip<T>(pub T);

#[cfg(feature = "flate2")]
impl<T: Process> Process for CompressGzip<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Vec<u8>, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let mut buf = Vec::with_capacity(src.len());
        let mut enc = flate2::bufread::GzEncoder::new(src, Default::default());
        enc.read_to_end(&mut buf)?;
        Ok(buf)
    }
}

#[cfg(feature = "zstd")]
pub struct CompressZstd<T>(pub T);

#[cfg(feature = "zstd")]
impl<T: Process> Process for CompressZstd<T> {
    type Error = BoxError;
    type Output = Vec<u8>;

    fn process_full(self) -> Result<Vec<u8>, Self::Error> {
        let out = self.0.process_full().map_err(Into::into)?;
        let src = out.as_ref();
        let buf = zstd::encode_all(src, 0)?;
        Ok(buf)
    }
}
