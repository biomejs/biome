use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueDeprecatedRouterLinkTagPropOptions {
    /// Names of the link components to check for the `tag` prop.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub components: Option<Box<[Box<str>]>>,
}

impl biome_deserialize::Merge for NoVueDeprecatedRouterLinkTagPropOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(components) = other.components {
            self.components = Some(components);
        }
    }
}

impl NoVueDeprecatedRouterLinkTagPropOptions {
    pub const DEFAULT_COMPONENTS: &[&str] = &["RouterLink"];

    /// Returns [`Self::components`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_COMPONENTS`].
    pub fn components(&self) -> impl Iterator<Item = &str> {
        let (configured, default) = match &self.components {
            Some(components) => (components.as_ref(), [].as_slice()),
            None => ([].as_slice(), Self::DEFAULT_COMPONENTS),
        };
        configured
            .iter()
            .map(AsRef::as_ref)
            .chain(default.iter().copied())
    }
}
