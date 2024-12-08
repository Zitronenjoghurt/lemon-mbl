pub trait HasInternalName {
    fn internal_name(&self) -> &str;
    fn with_internal_name(self, name: String) -> Self;
}