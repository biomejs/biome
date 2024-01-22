use biome_deserialize::{DeserializableValidator, DeserializationDiagnostic};
use biome_deserialize_macros::{Deserializable, Merge, Partial};
use bpaf::Bpaf;
use serde::{Deserialize, Serialize};
use std::str::FromStr;

const GIT_IGNORE_FILE_NAME: &str = ".gitignore";

/// Set of properties to integrate Biome with a VCS software.
#[derive(Clone, Debug, Deserialize, Eq, Partial, PartialEq, Serialize)]
#[partial(derive(Bpaf, Clone, Deserializable, Eq, Merge, PartialEq))]
#[partial(deserializable(with_validator))]
#[partial(cfg_attr(feature = "schema", derive(schemars::JsonSchema)))]
#[partial(serde(deny_unknown_fields, rename_all = "camelCase"))]
pub struct VcsConfiguration {
    /// The kind of client.
    #[partial(bpaf(long("vcs-client-kind"), argument("git"), optional))]
    #[partial(deserializable(bail_on_error))]
    pub client_kind: VcsClientKind,

    /// Whether Biome should integrate itself with the VCS client
    #[partial(bpaf(long("vcs-enabled"), argument("true|false")))]
    pub enabled: bool,

    /// Whether Biome should use the VCS ignore file. When [true], Biome will ignore the files
    /// specified in the ignore file.
    #[partial(bpaf(long("vcs-use-ignore-file"), argument("true|false")))]
    pub use_ignore_file: bool,

    /// The folder where Biome should check for VCS files. By default, Biome will use the same
    /// folder where `biome.json` was found.
    ///
    /// If Biome can't find the configuration, it will attempt to use the current working directory.
    /// If no current working directory can't be found, Biome won't use the VCS integration, and a diagnostic
    /// will be emitted
    #[partial(bpaf(long("vcs-root"), argument("PATH"), optional))]
    pub root: String,

    /// The main branch of the project
    #[partial(bpaf(long("vcs-default-branch"), argument("BRANCH"), optional))]
    pub default_branch: String,
}

impl Default for VcsConfiguration {
    fn default() -> Self {
        Self {
            client_kind: VcsClientKind::Git,
            enabled: false,
            use_ignore_file: true,
            root: Default::default(),
            default_branch: Default::default(),
        }
    }
}

impl PartialVcsConfiguration {
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

impl DeserializableValidator for PartialVcsConfiguration {
    fn validate(
        &self,
        _name: &str,
        range: biome_rowan::TextRange,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> bool {
        if self.client_kind.is_none() && self.is_enabled() {
            diagnostics.push(
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
