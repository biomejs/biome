mod actions;

pub use crate::analyzer::assist::actions::*;
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
use clap::Args;
use serde::{Deserialize, Serialize};

pub type AssistEnabled = Bool<true>;
#[derive(
    Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Args, Deserializable, Merge,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AssistConfiguration {
    /// Whether Biome should enable assist via LSP and CLI.
    // #[arg(long = "assist-enabled", value_name = "true|false")]
    pub enabled: Option<AssistEnabled>,

    /// Whether Biome should fail in CLI if the assist were not applied to the code.
    #[clap(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,

    /// A list of glob patterns. Biome will include files/folders that will
    /// match these patterns.
    #[clap(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub includes: Option<Vec<biome_glob::NormalizedGlob>>,
}

impl AssistConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }

    pub fn get_actions(&self) -> Actions {
        self.actions.clone().unwrap_or_default()
    }
}
