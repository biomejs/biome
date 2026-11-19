mod actions;

pub use crate::analyzer::assist::actions::*;
use crate::bool::Bool;
use biome_deserialize_macros::{Deserializable, Merge};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};

pub type AssistEnabled = Bool<true>;
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize, Deserializable, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AssistConfiguration {
    /// Enables configured assist actions in the CLI and editor integrations. Defaults to `true`.
    #[cfg_attr(feature = "cli", bpaf(long("assist-enabled"), argument("true|false")))]
    pub enabled: Option<AssistEnabled>,

    /// The assist-action configuration.
    #[cfg_attr(feature = "cli", bpaf(pure(Default::default()), optional, hide))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Actions>,

    /// A list of glob patterns selecting files on which assist actions can run. If omitted, all files
    /// selected by `files.includes` remain eligible for assist actions. An empty list selects no
    /// files. This option can only narrow the files selected by `files.includes`.
    #[cfg_attr(feature = "cli", bpaf(hide, pure(Default::default())))]
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
