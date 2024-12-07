pub trait HasInternalName {
    fn internal_name(&self) -> &str;
    fn set_internal_name(&mut self, name: String);
}
