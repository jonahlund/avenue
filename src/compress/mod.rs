#[cfg(feature = "compress-br")]
mod brotli;
#[cfg(feature = "compress-br")]
pub use brotli::*;

#[cfg(feature = "compress-deflate")]
mod deflate;
#[cfg(feature = "compress-deflate")]
pub use deflate::*;

#[cfg(feature = "compress-gzip")]
mod gzip;
#[cfg(feature = "compress-gzip")]
pub use gzip::*;

#[cfg(feature = "compress-zstd")]
mod zstd;
#[cfg(feature = "compress-zstd")]
pub use zstd::*;
