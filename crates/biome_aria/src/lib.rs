pub mod roles;

pub use roles::AriaRoles;

pub trait Element {
    /// Name of the current element or `None` if it cannot be retrieved.
    fn name(&self) -> Option<impl AsRef<str>>;

    /// Attributes set on the current element.
    fn attributes(&self) -> impl Iterator<Item = impl Attribute>;

    /// returns the first attribute with a name that matches `matcher`.
    fn find_attribute_by_name(&self, matcher: impl Fn(&str) -> bool) -> Option<impl Attribute> {
        self.attributes().find_map(|attribute| {
            if matcher(attribute.name()?.as_ref()) {
                Some(attribute)
            } else {
                None
            }
        })
    }
}

pub trait Attribute {
    /// Name of the current attribute or `None` if it cannot be retrieved.
    fn name(&self) -> Option<impl AsRef<str>>;

    /// Value of the current attribute or `None` if it cannot be retrieved.
    fn value(&self) -> Option<impl AsRef<str>>;
}
