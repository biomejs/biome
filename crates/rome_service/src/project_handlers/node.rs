use crate::project_handlers::{
    DeserializeResults, ProjectAnalyzerCapabilities, ProjectCapabilities, ProjectHandler,
};
use crate::WorkspaceError;
use rome_diagnostics::serde::Diagnostic as SerdeDiagnostic;
use rome_diagnostics::Severity;
use rome_fs::RomePath;
use rome_parser::AnyParse;
use rome_project::NodeJsProject;

#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct NodeProjectHandler {}

impl ProjectHandler for NodeProjectHandler {
    fn capabilities(&self) -> ProjectCapabilities {
        ProjectCapabilities {
            analyzer: ProjectAnalyzerCapabilities {
                licenses: Some(analyze_licenses),
                deserialize: Some(deserialize),
            },
        }
    }
}

fn deserialize(_: &RomePath, parse: AnyParse) -> Result<DeserializeResults, WorkspaceError> {
    let mut node_js_project = NodeJsProject::default();
    let tree = parse.tree();
    node_js_project.deserialize(tree);

    let diagnostic_count = node_js_project.diagnostics.len() as u64;
    let errors = node_js_project
        .diagnostics
        .iter()
        .filter(|diag| diag.severity() <= Severity::Error)
        .count();

    let skipped_diagnostics = diagnostic_count - node_js_project.diagnostics.len() as u64;

    Ok(DeserializeResults {
        diagnostics: node_js_project
            .diagnostics
            .into_iter()
            .map(SerdeDiagnostic::new)
            .collect(),
        errors,
        skipped_diagnostics,
    })
}

fn analyze_licenses(_path: &RomePath, _parse: AnyParse) -> Result<(), WorkspaceError> {
    Ok(())
}
