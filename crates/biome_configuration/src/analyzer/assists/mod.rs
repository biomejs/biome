mod rules;

pub use crate::analyzer::assists::rules::*;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct Assists {
    /// Whether Biome should enable assists via LSP.
    #[partial(bpaf(long("assists-enabled"), argument("true|false")))]
    pub enabled: Option<bool>,

    /// Whether Biome should fail in CLI if the assists were not applied to the code.
    #[partial(bpaf(pure(Default::default()), optional, hide))]
    pub rules: Rules,
}

impl Default for Assists {
    fn default() -> Self {
        Self {
            enabled: Some(false),
            rules: Rules::default(),
        }
    }
}
