use crate::project_handlers::node::NodeProjectHandler;
use crate::project_handlers::unknown::UnknownProjectHandler;
use crate::WorkspaceError;
use rome_fs::RomePath;
use rome_parser::AnyParse;
use std::path::Path;

mod node;
mod unknown;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum Manifests {
    Node,
    #[default]
    Unknown,
}

impl Manifests {
    pub const KNOWN_MANIFESTS: [&'static str; 1] = ["package.json"];

    pub fn is_manifest(path: &Path) -> bool {
        path.file_name()
            .and_then(|s| s.to_str())
            .map(|file_name| Self::KNOWN_MANIFESTS.contains(&file_name))
            .unwrap_or_default()
    }

    pub(crate) fn or(self, other: Manifests) -> Self {
        if self != Manifests::Unknown {
            self
        } else {
            other
        }
    }
}

pub(crate) trait ProjectHandler {
    fn capabilities(&self) -> ProjectCapabilities {
        ProjectCapabilities::default()
    }
}

#[derive(Default)]
pub(crate) struct ProjectCapabilities {
    pub(crate) analyzer: ProjectAnalyzerCapabilities,
}

#[derive(Default)]
pub(crate) struct ProjectAnalyzerCapabilities {
    pub(crate) licenses: Option<Licenses>,
    pub(crate) deserialize: Option<Deserialize>,
}

type Licenses = fn(&RomePath, AnyParse) -> Result<(), WorkspaceError>;
type Deserialize = fn(&RomePath, AnyParse) -> Result<DeserializeResults, WorkspaceError>;

pub(crate) struct KnownProjectHandlers {
    node: NodeProjectHandler,
    unknown: UnknownProjectHandler,
}

pub(crate) struct DeserializeResults {
    pub(crate) diagnostics: Vec<rome_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u64,
}
impl KnownProjectHandlers {
    pub(crate) fn new() -> Self {
        KnownProjectHandlers {
            node: NodeProjectHandler::default(),
            unknown: UnknownProjectHandler::default(),
        }
    }

    pub(crate) fn get_manifest(path: &RomePath) -> Manifests {
        path.components()
            .last()
            .and_then(|component| component.as_os_str().to_str())
            .map(|file_name| match file_name {
                "package.json" => Manifests::Node,
                _ => Manifests::Unknown,
            })
            .unwrap_or_default()
    }

    /// Returns the [ProjectCapabilities] associated with a [RomePath]
    pub(crate) fn get_capabilities(
        &self,
        manifest_path: &RomePath,
        manifest: Manifests,
    ) -> ProjectCapabilities {
        let manifest = Self::get_manifest(manifest_path).or(manifest);
        match manifest {
            Manifests::Node => self.node.capabilities(),
            Manifests::Unknown => self.unknown.capabilities(),
        }
    }
}
