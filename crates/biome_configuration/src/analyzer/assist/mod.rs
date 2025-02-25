mod actions;

pub use crate::analyzer::assist::actions::*;
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type AssistEnabled = Bool<true>;
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AssistConfiguration {
    /// Whether Biome should enable assist via LSP.
    #[bpaf(long("assist-enabled"), argument("true|false"))]
    pub enabled: Option<AssistEnabled>,

    /// Whether Biome should fail in CLI if the assist were not applied to the code.
    #[bpaf(pure(Default::default()), optional, hide)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,

    /// A list of glob patterns. Biome will include files/folders that will
    /// match these patterns.
    #[bpaf(hide, pure(Default::default()))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::Glob>>,
}

impl AssistConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn get_actions(&self) -> Actions {
        self.actions.clone().unwrap_or_default()
    }
}
