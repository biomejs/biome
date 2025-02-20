mod actions;

pub use crate::analyzer::assists::actions::*;
use biome_deserialize::StringSet;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct AssistsConfiguration {
    /// Whether Biome should enable assists via LSP.
    #[partial(bpaf(long("assists-enabled"), argument("true|false")))]
    pub enabled: bool,

    /// Whether Biome should fail in CLI if the assists were not applied to the code.
    #[partial(bpaf(pure(Default::default()), optional, hide))]
    pub actions: Actions,

    /// A list of glob patterns. Biome will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(pure(Default::default()), hide))]
    pub includes: Vec<biome_glob::Glob>,
}

impl Default for AssistsConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            actions: Actions::default(),
            includes: Default::default(),
        }
    }
}
