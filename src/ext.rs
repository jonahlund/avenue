pub trait AssetExt {
    #[cfg(feature = "mime")]
    fn mime(&self) -> Option<mime::Mime>;

    #[cfg(feature = "std")]
    fn path(&self) -> Option<&std::path::Path>;

    fn size_hint(&self) -> Option<usize>;
}
