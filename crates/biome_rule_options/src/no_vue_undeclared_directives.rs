use biome_deserialize_macros::Deserializable;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoVueUndeclaredDirectivesOptions {
    /// Names of directives registered globally with `app.directive(...)`,
    /// such as `highlight` for `v-highlight`.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub globals: Option<Box<[Box<str>]>>,
}

impl NoVueUndeclaredDirectivesOptions {
    /// Returns the configured global directive names, or an empty slice.
    pub fn globals(&self) -> &[Box<str>] {
        self.globals.as_deref().unwrap_or_default()
    }
}

impl biome_deserialize::Merge for NoVueUndeclaredDirectivesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(globals) = other.globals {
            self.globals = Some(globals);
        }
    }
}
