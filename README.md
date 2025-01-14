# Usage

```rust
#[cfg(debug_assertions)]
#[derive(avenue::Embed)]
#[directory = "public"]
struct Assets;

#[cfg(not(debug_assertions))]
#[derive(avenue::Embed)]
#[directory = "public"]
#[apply(minify, compress_br, compress_gzip, compress_none)]
struct Assets;
















pub trait Embed<T: 'static> {
    const ASSETS: &[T];

    fn get(key: &str) -> Option<&T>;
}

struct FileAsset {
    path: &'static str,
}

// #[cfg(debug_assertions())]
// #[derive(avenue::Embed)]
struct Assets;

impl Assets {
    pub const PUBLIC_APP_JS: &FileAsset =
        &<Self as Embed<FileAsset>>::ASSETS[0];
    pub const PUBLIC_HERO_PNG: &FileAsset =
        &<Self as Embed<FileAsset>>::ASSETS[1];
}

impl Embed<FileAsset> for Assets {
    const ASSETS: &[FileAsset] =
        &[FileAsset { path: "" }, FileAsset { path: "" }];

    fn get(key: &str) -> Option<&FileAsset> {
        match key {
            "/public/app.js" => Some(&Self::ASSETS[0]),
            "/public/hero.png" => Some(&Self::ASSETS[1]),
            _ => None,
        }
    }
}

```
