mod actions;

pub use crate::analyzer::assist::actions::*;
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct AssistConfiguration {
    /// Whether Biome should enable assist via LSP.
    #[partial(bpaf(long("assist-enabled"), argument("true|false")))]
    pub enabled: bool,

    /// Whether Biome should fail in CLI if the assist were not applied to the code.
    #[partial(bpaf(pure(Default::default()), optional, hide))]
    pub actions: Actions,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide, pure(Default::default())))]
    pub ignore: indexmap::IndexSet<String>,

    /// A list of Unix shell style patterns. The formatter will include files/folders that will
    /// match these patterns.
    #[partial(bpaf(hide, pure(Default::default())))]
    pub include: indexmap::IndexSet<String>,
}

impl Default for AssistConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            actions: Default::default(),
            ignore: Default::default(),
            include: Default::default(),
        }
    }
}
