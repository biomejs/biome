use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueTemplateShadowOptions {
    /// Names that template variables may reuse from an outer scope.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub allow: Option<Box<[Box<str>]>>,
}

impl NoVueTemplateShadowOptions {
    /// Returns [`Self::allow`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn allow(&self) -> &[Box<str>] {
        self.allow.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append `allow` lists across
// extended configurations, so a child configuration could never remove an
// inherited name. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for NoVueTemplateShadowOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(allow) = other.allow {
            self.allow = Some(allow);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::Merge;

    fn options(allow: &[&str]) -> NoVueTemplateShadowOptions {
        NoVueTemplateShadowOptions {
            allow: Some(allow.iter().map(|name| Box::from(*name)).collect()),
        }
    }

    #[test]
    fn merge_replaces_inherited_allow() {
        let mut base = options(&["item"]);
        base.merge_with(options(&["index"]));
        assert_eq!(base.allow(), [Box::from("index")]);
    }

    #[test]
    fn merge_clears_inherited_allow_with_an_empty_list() {
        let mut base = options(&["item"]);
        base.merge_with(options(&[]));
        assert!(base.allow().is_empty());
    }

    #[test]
    fn merge_keeps_inherited_allow_when_unset() {
        let mut base = options(&["item"]);
        base.merge_with(NoVueTemplateShadowOptions::default());
        assert_eq!(base.allow(), [Box::from("item")]);
    }
}
