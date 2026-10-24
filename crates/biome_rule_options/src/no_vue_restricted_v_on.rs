use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueRestrictedVOnOptions {
    /// Event listeners to restrict.
    ///
    /// A `v-on` directive is reported when it matches every condition of an entry.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub listeners: Option<Box<[RestrictedVOnListener]>>,
}

impl NoVueRestrictedVOnOptions {
    /// Returns [`Self::listeners`] if it is set.
    /// Otherwise, returns an empty slice.
    pub fn listeners(&self) -> &[RestrictedVOnListener] {
        self.listeners.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append lists across extended
// configurations, so a child configuration could never remove an inherited
// restriction. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for NoVueRestrictedVOnOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(listeners) = other.listeners {
            self.listeners = Some(listeners);
        }
    }
}

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct RestrictedVOnListener {
    /// The event name to restrict, such as `"click"` for `@click` and `v-on:click`.
    ///
    /// When omitted, the entry restricts `v-on` used without an event name, such as
    /// `v-on="handlers"`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub event: Option<Box<str>>,

    /// Modifiers that must all be present for the entry to apply, such as `["prevent"]`
    /// for `@click.prevent`.
    #[serde(skip_serializing_if = "<[_]>::is_empty")]
    pub modifiers: Box<[Box<str>]>,

    /// The element or component name the entry applies to, such as `"MyButton"`.
    /// When omitted, the entry applies to all elements.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub element: Option<Box<str>>,

    /// A custom message shown in the diagnostic.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub message: Option<Box<str>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::Merge;

    fn options(events: &[&str]) -> NoVueRestrictedVOnOptions {
        NoVueRestrictedVOnOptions {
            listeners: Some(
                events
                    .iter()
                    .map(|event| RestrictedVOnListener {
                        event: Some(Box::from(*event)),
                        ..Default::default()
                    })
                    .collect(),
            ),
        }
    }

    #[test]
    fn merge_replaces_inherited_listeners() {
        let mut base = options(&["click"]);
        base.merge_with(options(&["submit"]));
        assert_eq!(base, options(&["submit"]));
    }

    #[test]
    fn merge_keeps_inherited_listeners_when_unset() {
        let mut base = options(&["click"]);
        base.merge_with(NoVueRestrictedVOnOptions::default());
        assert_eq!(base, options(&["click"]));
    }
}
