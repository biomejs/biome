use crate::bool::Bool;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::{Deserializable, Merge};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const GIT_IGNORE_FILE_NAME: &str = ".gitignore";

/// Set of properties to integrate Biome with a VCS software.
#[derive(
    Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Bpaf, Deserializable, Default, Merge,
)]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VcsConfiguration {
    /// Whether Biome should integrate itself with the VCS client
    #[bpaf(long("vcs-enabled"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Bool<false>>,

    /// The kind of client.
    #[bpaf(long("vcs-client-kind"), argument("git"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(bail_on_error)]
    pub client_kind: Option<VcsClientKind>,

    /// Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files
    /// specified in the ignore file.
    #[bpaf(long("vcs-use-ignore-file"), argument("true|false"))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ignore_file: Option<Bool<true>>,

    /// The folder where Biome should check for VCS files. By default, Biome will use the same
    /// folder where `biome.json` was found.
    ///
    /// If Biome can't find the configuration, it will attempt to use the current working directory.
    /// If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
    /// will be emitted
    #[bpaf(long("vcs-root"), argument("PATH"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,

    /// The main branch of the project
    #[bpaf(long("vcs-default-branch"), argument("BRANCH"), optional)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_branch: Option<String>,
}

impl VcsConfiguration {
    pub fn is_enabled(&self) -> bool {
        self.enabled.unwrap_or_default().into()
    }
    pub fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }
    pub fn should_use_ignore_file(&self) -> bool {
        self.use_ignore_file.unwrap_or_default().into()
    }
}

impl DeserializableValidator for VcsConfiguration {
    fn validate(
        &mut self,
        ctx: &mut impl DeserializationContext,
        _name: &str,
        range: biome_rowan::TextRange,
    ) -> bool {
        if self.client_kind.is_none() && self.is_enabled() {
            ctx.report(
                DeserializationDiagnostic::new(
                    "You enabled the VCS integration, but you didn't specify a client.",
                )
                .with_range(range)
                .with_note("Biome will disable the VCS integration until the issue is fixed."),
            );
            return false;
        }

        true
    }
}

#[derive(
    Clone, Copy, Debug, Default, Deserialize, Deserializable, Eq, Merge, PartialEq, Serialize,
)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VcsClientKind {
    #[default]
    /// Integration with the git client as VCS
    Git,
}

impl VcsClientKind {
    pub const fn ignore_file(&self) -> &'static str {
        match self {
            VcsClientKind::Git => GIT_IGNORE_FILE_NAME,
        }
    }
}

impl FromStr for VcsClientKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "git" => Ok(Self::Git),
            _ => Err("Value not supported for VcsClientKind"),
        }
    }
}
