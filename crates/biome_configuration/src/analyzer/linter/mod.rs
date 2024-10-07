#[rustfmt::skip]
mod rules;

use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
pub use rules::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(rename_all = "camelCase", default, deny_unknown_fields))]
pub struct LinterConfiguration {
    /// if `false`, it disables the feature and the linter won't be executed. `true` by default
    #[partial(bpaf(hide))]
    pub enabled: bool,

    /// List of rules
    #[partial(bpaf(pure(Default::default()), optional, hide))]
    pub rules: Rules,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub ignore: StringSet,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide))]
    pub include: StringSet,
}

impl LinterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        !self.enabled
    }
}

impl Default for LinterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            rules: Default::default(),
            ignore: Default::default(),
            include: Default::default(),
        }
    }
}

impl PartialLinterConfiguration {
    pub const fn is_disabled(&self) -> bool {
        matches!(self.enabled, Some(false))
    }

    pub fn get_rules(&self) -> Rules {
        self.rules.as_ref().unwrap_or(&Rules::default()).clone()
    }
}
