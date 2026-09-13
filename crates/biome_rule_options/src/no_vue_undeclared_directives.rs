use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueUndeclaredDirectivesOptions {
    /// Names of directives registered globally with `app.directive(...)`,
    /// written in kebab-case without the `v-` prefix, such as
    /// `click-outside` for `v-click-outside`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub globals: Option<Box<[Box<str>]>>,
}

impl NoVueUndeclaredDirectivesOptions {
    /// Returns the configured global directive names, or an empty slice.
    pub fn globals(&self) -> &[Box<str>] {
        self.globals.as_deref().unwrap_or_default()
    }
}

// Not derived: the derived implementation would append `globals` lists across
// extended configurations, so a child configuration could never remove an
// inherited exemption. A set list replaces the inherited one instead.
impl biome_deserialize::Merge for NoVueUndeclaredDirectivesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(globals) = other.globals {
            self.globals = Some(globals);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use biome_deserialize::Merge;

    fn options(globals: &[&str]) -> NoVueUndeclaredDirectivesOptions {
        NoVueUndeclaredDirectivesOptions {
            globals: Some(globals.iter().map(|name| Box::from(*name)).collect()),
        }
    }

    #[test]
    fn merge_replaces_inherited_globals() {
        let mut base = options(&["highlight"]);
        base.merge_with(options(&["focus"]));
        assert_eq!(base.globals(), [Box::from("focus")]);
    }

    #[test]
    fn merge_clears_inherited_globals_with_an_empty_list() {
        let mut base = options(&["highlight"]);
        base.merge_with(options(&[]));
        assert!(base.globals().is_empty());
    }

    #[test]
    fn merge_keeps_inherited_globals_when_unset() {
        let mut base = options(&["highlight"]);
        base.merge_with(NoVueUndeclaredDirectivesOptions::default());
        assert_eq!(base.globals(), [Box::from("highlight")]);
    }
}
