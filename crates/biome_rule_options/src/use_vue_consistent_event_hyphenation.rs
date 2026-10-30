use biome_deserialize_macros::{Deserializable, Merge};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseVueConsistentEventHyphenationOptions {
    /// Whether event names in `v-on` directives must be hyphenated (`"always"`) or must not
    /// contain hyphens (`"never"`). If omitted, hyphenated names are required.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub hyphenation: Option<EventHyphenation>,

    /// List of event names to exempt from the check.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore: Option<Box<[Box<str>]>>,

    /// List of tag names whose events should not be checked.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub ignore_tags: Option<Box<[Box<str>]>>,
}

#[derive(
    Clone, Copy, Default, Debug, Deserialize, Deserializable, Merge, Eq, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum EventHyphenation {
    /// Event names must be hyphenated, e.g. `custom-event`.
    #[default]
    Always,
    /// Event names must not contain hyphens, e.g. `customEvent`.
    Never,
}

impl UseVueConsistentEventHyphenationOptions {
    pub const DEFAULT_HYPHENATION: EventHyphenation = EventHyphenation::Always;

    /// Returns [`Self::hyphenation`] if it is set.
    /// Otherwise, returns [`Self::DEFAULT_HYPHENATION`].
    pub fn hyphenation(&self) -> EventHyphenation {
        self.hyphenation.unwrap_or(Self::DEFAULT_HYPHENATION)
    }

    /// Returns the configured ignored event names, or an empty slice.
    pub fn ignore(&self) -> &[Box<str>] {
        self.ignore.as_deref().unwrap_or_default()
    }

    /// Returns the configured ignored tag names, or an empty slice.
    pub fn ignore_tags(&self) -> &[Box<str>] {
        self.ignore_tags.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append lists across extended
// configurations, so a child configuration could never remove an inherited
// exemption. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for UseVueConsistentEventHyphenationOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(hyphenation) = other.hyphenation {
            self.hyphenation = Some(hyphenation);
        }
        if let Some(ignore) = other.ignore {
            self.ignore = Some(ignore);
        }
        if let Some(ignore_tags) = other.ignore_tags {
            self.ignore_tags = Some(ignore_tags);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::Merge;

    fn options(ignore: &[&str]) -> UseVueConsistentEventHyphenationOptions {
        UseVueConsistentEventHyphenationOptions {
            ignore: Some(ignore.iter().map(|name| Box::from(*name)).collect()),
            ..Default::default()
        }
    }

    #[test]
    fn merge_replaces_inherited_lists() {
        let mut base = options(&["customEvent"]);
        base.merge_with(options(&["otherEvent"]));
        assert_eq!(base.ignore(), [Box::from("otherEvent")]);
    }

    #[test]
    fn merge_keeps_inherited_values_when_unset() {
        let mut base = options(&["customEvent"]);
        base.hyphenation = Some(EventHyphenation::Never);
        base.merge_with(UseVueConsistentEventHyphenationOptions::default());
        assert_eq!(base.ignore(), [Box::from("customEvent")]);
        assert_eq!(base.hyphenation(), EventHyphenation::Never);
    }
}
