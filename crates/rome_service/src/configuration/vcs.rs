use crate::configuration::merge::MergeWith;
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const GIT_IGNORE_FILE_NAME: &str = ".gitignore";

/// Set of properties to integrate Biome with a VCS software.
#[derive(Debug, Default, Deserialize, Serialize, Clone, Bpaf)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VcsConfiguration {
    /// The kind of client.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("vcs-client-kind"), argument("git"), optional)]
    pub client_kind: Option<VcsClientKind>,

    /// Whether Biome should integrate itself with the VCS client
    #[bpaf(long("vcs-enabled"), argument("true|false"))]
    pub enabled: Option<bool>,

    /// Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files
    /// specified in the ignore file.
    #[bpaf(long("vcs-use-ignore-file"), argument("true|false"))]
    pub use_ignore_file: Option<bool>,

    /// The folder where Biome should check for VCS files. By default, Biome will use the same
    /// folder where `rome.json` was found.
    ///
    /// If Biome can't find the configuration, it will attempt to use the current working directory.
    /// If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
    /// will be emitted
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(long("vcs-root"), argument("PATH"), optional)]
    pub root: Option<String>,
}

impl VcsConfiguration {
    pub const fn is_enabled(&self) -> bool {
        matches!(self.enabled, Some(true))
    }
    pub const fn is_disabled(&self) -> bool {
        !self.is_enabled()
    }
    pub const fn ignore_file_disabled(&self) -> bool {
        matches!(self.use_ignore_file, Some(false))
    }
}

impl MergeWith<VcsConfiguration> for VcsConfiguration {
    fn merge_with(&mut self, other: VcsConfiguration) {
        if let Some(enabled) = other.enabled {
            self.enabled = Some(enabled);
        }
        if let Some(client_kind) = other.client_kind {
            self.client_kind = Some(client_kind);
        }
        if let Some(use_ignore_file) = other.use_ignore_file {
            self.use_ignore_file = Some(use_ignore_file);
        }
        if let Some(root) = other.root {
            self.root = Some(root);
        }
    }
}

#[derive(Debug, Default, Deserialize, Clone, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VcsClientKind {
    #[default]
    /// Integration with the git client as VCS
    Git,
}

impl VcsClientKind {
    pub const KNOWN_VALUES: &'static [&'static str] = &["git"];

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

impl VcsConfiguration {
    pub const KNOWN_KEYS: &'static [&'static str] =
        &["clientKind", "enabled", "useIgnoreFile", "root"];
}
