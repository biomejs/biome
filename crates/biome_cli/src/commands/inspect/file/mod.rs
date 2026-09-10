use crate::{CliDiagnostic, CliSession};
use biome_configuration::ConfigurationPathHint;
use biome_console::fmt::Formatter;
use biome_console::{ConsoleExt, markup};
use biome_diagnostics::{
    Advices, Category, Diagnostic, LogCategory, PrintDiagnostic, Severity, Visit, category,
};
use biome_fs::BiomePath;
use biome_service::WorkspaceError;
use biome_service::configuration::load_configuration;
use biome_service::settings::ModuleGraphResolutionKind;
use biome_service::workspace::{
    FileContent, GetFormatterIRParams, GetSemanticModelParams, GetSyntaxTreeParams,
    GetSyntaxTreeResult, OpenFileParams, OpenProjectParams, UpdateSettingsParams,
};
use camino::Utf8PathBuf;
use std::ffi::OsString;

pub(crate) struct InspectFile<'a> {
    pub(crate) path: OsString,
    pub(crate) ast: bool,
    pub(crate) ir: bool,
    pub(crate) semantic: bool,
    pub(crate) session: CliSession<'a>,
}

pub(crate) fn inspect_file(payload: InspectFile) -> Result<(), CliDiagnostic> {
    let InspectFile {
        path,
        ast,
        ir,
        semantic,
        session,
    } = payload;

    let workspace = &*session.app.workspace;
    let console = session.app.console;
    let fs = workspace.fs();

    let path = BiomePath::new(Utf8PathBuf::try_from(path).unwrap());

    if !fs.path_is_file(&path) {
        return Err(CliDiagnostic::unexpected_argument(
            "The path must be a file",
            "inspect file file.js",
        ));
    }

    let path_hint = ConfigurationPathHint::default();
    let loaded_configuration = load_configuration(fs, path_hint)?;
    let extended_configurations = loaded_configuration.extended_configurations();
    let configuration = loaded_configuration.resolved_configuration();

    let project_directory = BiomePath::new(
        loaded_configuration
            .directory_path()
            .map(Utf8PathBuf::from)
            .unwrap_or_default(),
    );

    let result = workspace.open_project(OpenProjectParams {
        path: project_directory.clone(),
        open_uninitialized: true,
    })?;
    let project_key = result.project_key;

    workspace.update_settings(UpdateSettingsParams {
        project_key,
        configuration,
        workspace_directory: Some(project_directory),
        extended_configurations: extended_configurations
            .into_iter()
            .map(|(path, config)| (BiomePath::from(path), config))
            .collect(),
        module_graph_resolution_kind: ModuleGraphResolutionKind::default(),
    })?;

    let file_content = fs
        .read_file_from_path(path.as_path())
        .map_err(WorkspaceError::from)?;

    workspace.open_file(OpenFileParams {
        project_key,
        path: path.clone(),
        content: FileContent::FromClient {
            content: file_content,
            version: 0,
        },
        document_file_source: None,
        persist_node_cache: false,
        inline_config: None,
        editor_features: None,
    })?;

    let GetSyntaxTreeResult {
        cst: cst_string,
        ast: ast_string,
    } = workspace.get_syntax_tree(GetSyntaxTreeParams {
        project_key,
        path: path.clone(),
    })?;

    let mut diagnostic = InspectFileDiagnostic::new(cst_string);

    if ast {
        diagnostic.ast = Some(ast_string);
    }

    if ir {
        let ir_string = workspace.get_formatter_ir(GetFormatterIRParams {
            project_key,
            path: path.clone(),
        })?;
        diagnostic.ir = Some(ir_string);
    }

    if semantic {
        // Not all languages have a semantic model, and those that don't, will throw an error.
        let result = workspace.get_semantic_model(GetSemanticModelParams {
            project_key,
            path: path.clone(),
        });
        if let Ok(semantic) = result {
            diagnostic.semantic = Some(semantic);
        }
    }

    console.log(markup! {
        {PrintDiagnostic::simple(&diagnostic)}
    });

    Ok(())
}

#[derive(Debug, Default)]
struct InspectFileDiagnostic {
    cst: String,
    ast: Option<String>,
    ir: Option<String>,
    semantic: Option<String>,
}

impl InspectFileDiagnostic {
    fn new(cst: String) -> Self {
        Self {
            cst,
            ast: None,
            ir: None,
            semantic: None,
        }
    }
}

impl Diagnostic for InspectFileDiagnostic {
    fn category(&self) -> Option<&'static Category> {
        Some(category!("inspect"))
    }

    fn severity(&self) -> Severity {
        Severity::Information
    }

    fn message(&self, fmt: &mut Formatter<'_>) -> std::io::Result<()> {
        fmt.write_str("Below the information requested.")
    }
    fn advices(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        self.record(visitor)
    }
}

impl Advices for InspectFileDiagnostic {
    fn record(&self, visitor: &mut dyn Visit) -> std::io::Result<()> {
        visitor.record_log(LogCategory::None, &markup! {"CST"})?;
        visitor.record_log(LogCategory::None, &self.cst.as_str())?;

        if let Some(ast) = &self.ast.as_ref() {
            visitor.record_log(LogCategory::None, &markup! {"AST"})?;
            visitor.record_log(LogCategory::None, &ast.as_str())?;
        }

        if let Some(ir) = &self.ir.as_ref() {
            visitor.record_log(LogCategory::None, &markup! {"IR"})?;
            visitor.record_log(LogCategory::None, &ir.as_str())?;
        }

        if let Some(semantic) = &self.semantic.as_ref() {
            visitor.record_log(LogCategory::None, &markup! {"Semantic model"})?;
            visitor.record_log(LogCategory::None, &semantic.as_str())?;
        }
        Ok(())
    }
}
