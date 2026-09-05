use crate::bool::Bool;
use biome_deserialize::{
    DeserializableValidator, DeserializationContext, DeserializationDiagnostic,
};
use biome_deserialize_macros::{Deserializable, Merge};
#[cfg(feature = "cli")]
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

// NOTE: when adding a new ignore file, update [DocumentFileSource::try_from_path]
pub const GIT_IGNORE_FILE_NAME: &str = ".gitignore";
pub const IGNORE_FILE_NAME: &str = ".ignore";

pub type VcsUseIgnoreFile = Bool<false>;
pub type VcsEnabled = Bool<false>;

/// Configures how Biome integrates with a version control system.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, Deserializable, Default, Merge)]
#[cfg_attr(feature = "cli", derive(Bpaf))]
#[deserializable(with_validator)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VcsConfiguration {
    /// Enables or disables version-control integration. Defaults to `false`. Enabling integration
    /// requires `vcs.clientKind` to be set.
    #[cfg_attr(feature = "cli", bpaf(long("vcs-enabled"), argument("true|false")))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VcsEnabled>,

    /// Selects the version-control client. Currently, only `git` is supported.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("vcs-client-kind"), argument("git"), optional)
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deserializable(bail_on_error)]
    pub client_kind: Option<VcsClientKind>,

    /// Controls whether Biome applies patterns from `.gitignore`, Git's local `.git/info/exclude`,
    /// and supported `.ignore` files, including nested ignore files. Patterns in the root ignore file
    /// are resolved from `vcs.root`. Patterns in nested ignore files are resolved from the directory
    /// containing that file. Defaults to `false`.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("vcs-use-ignore-file"), argument("true|false"))
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ignore_file: Option<VcsUseIgnoreFile>,

    /// Sets the directory where Biome looks for version-control files. A relative value is resolved
    /// from the directory containing the current configuration file, while an absolute path is used
    /// directly.
    ///
    /// Defaults to the directory containing `biome.json` or `biome.jsonc`. If no configuration is
    /// found, Biome uses the current working directory.
    ///
    /// If neither directory is available, Biome disables version control integration and emits a
    /// diagnostic.
    #[cfg_attr(feature = "cli", bpaf(long("vcs-root"), argument("PATH"), optional))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,

    /// Sets the base branch used by `--changed` when `--since` is not provided. If neither this option
    /// nor `--since` is set, commands using `--changed` fail because Biome cannot determine the
    /// comparison base.
    #[cfg_attr(
        feature = "cli",
        bpaf(long("vcs-default-branch"), argument("BRANCH"), optional)
    )]
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
    pub fn new_git_ignore() -> Self {
        Self {
            enabled: Some(true.into()),
            client_kind: Some(VcsClientKind::Git),
            use_ignore_file: Some(true.into()),
            root: None,
            default_branch: None,
        }
    }
}

impl DeserializableValidator for VcsConfiguration {
    fn validate(
        &mut self,
        ctx: &mut dyn DeserializationContext,
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
    /// Integration with Git as the version control client.
    Git,
}

impl VcsClientKind {
    pub const fn ignore_files(&self) -> &[&str] {
        match self {
            Self::Git => &[GIT_IGNORE_FILE_NAME, IGNORE_FILE_NAME],
        }
    }
}

impl FromStr for VcsClientKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "git" => Ok(Self::Git),
            _ => Err("Value not supported for clientKind."),
        }
    }
}
