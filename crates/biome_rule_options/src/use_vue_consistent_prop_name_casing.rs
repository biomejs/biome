use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentPropNameCasingOptions {
    /// The casing that prop names must follow. If omitted, prop names must be in camelCase.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub casing: Option<PropNameCasing>,

    /// List of prop names to exempt from the check.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum PropNameCasing {
    /// Prop names must be in camelCase, e.g. `greetingText`.
    #[default]
    #[serde(rename = "camelCase")]
    Camel,
    /// Prop names must be in snake_case, e.g. `greeting_text`.
    #[serde(rename = "snake_case")]
    Snake,
}

impl UseVueConsistentPropNameCasingOptions {
    pub const DEFAULT_CASING: PropNameCasing = PropNameCasing::Camel;

    /// Returns [`Self::casing`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_CASING`].
    pub fn casing(&self) -> PropNameCasing {
        self.casing.unwrap_or(Self::DEFAULT_CASING)
    }

    /// Returns the configured ignored prop names, or an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append lists across extended
// configurations, so a child configuration could never remove an inherited
// exemption. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for UseVueConsistentPropNameCasingOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(casing) = other.casing {
            self.casing = Some(casing);
        }
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::Merge;

    fn options(ignore: &[&str]) -> UseVueConsistentPropNameCasingOptions {
        UseVueConsistentPropNameCasingOptions {
            ignore: Some(ignore.iter().map(|name| Box::from(*name)).collect()),
            ..Default::default()
        }
    }

    #[test]
    fn merge_replaces_inherited_lists() {
        let mut base = options(&["greeting_text"]);
        base.merge_with(options(&["other_prop"]));
        assert_eq!(base.ignore(), [Box::from("other_prop")]);
    }

    #[test]
    fn merge_keeps_inherited_values_when_unset() {
        let mut base = options(&["greeting_text"]);
        base.casing = Some(PropNameCasing::Snake);
        base.merge_with(UseVueConsistentPropNameCasingOptions::default());
        assert_eq!(base.ignore(), [Box::from("greeting_text")]);
        assert_eq!(base.casing(), PropNameCasing::Snake);
    }
}
