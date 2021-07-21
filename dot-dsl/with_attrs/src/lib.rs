pub trait Attrs {
    fn with_attrs(self, attrs: &[(&str, &str)]) -> Self;
}
