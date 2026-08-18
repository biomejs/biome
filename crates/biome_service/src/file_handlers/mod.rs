// Some structs are used conditionally in some language-gated functions, so we
// add this **allow**.
#![allow(dead_code)]

#[cfg(feature = "lang_js")]
pub mod astro;
#[cfg(feature = "lang_css")]
pub(crate) mod css;
#[cfg(feature = "lang_graphql")]
pub(crate) mod graphql;
#[cfg(feature = "lang_grit")]
pub(crate) mod grit;
#[cfg(feature = "lang_html")]
pub(crate) mod html;
mod ignore;
#[cfg(feature = "lang_js")]
pub(crate) mod javascript;
pub(crate) mod json;
#[cfg(feature = "lang_md")]
pub(crate) mod md;
#[cfg(all(feature = "lang_js", feature = "lang_html"))]
pub mod svelte;
mod unknown;
#[cfg(all(feature = "lang_js", feature = "lang_html"))]
pub mod vue;
#[cfg(feature = "lang_yaml")]
pub(crate) mod yaml;

#[cfg(feature = "lang_css")]
use self::css::CssFileHandler;
#[cfg(feature = "lang_js")]
use self::javascript::JsFileHandler;
use self::{json::JsonFileHandler, unknown::UnknownFileHandler};
use crate::WorkspaceError;
use crate::db::WorkspaceDb;
use crate::embed::EmbedContent;
#[cfg(feature = "lang_js")]
pub use crate::file_handlers::astro::AstroFileHandler;
#[cfg(feature = "lang_graphql")]
use crate::file_handlers::graphql::GraphqlFileHandler;
use crate::file_handlers::ignore::IgnoreFileHandler;
#[cfg(all(feature = "lang_js", feature = "lang_html"))]
pub use crate::file_handlers::svelte::SvelteFileHandler;
#[cfg(all(feature = "lang_js", feature = "lang_html"))]
pub use crate::file_handlers::vue::VueFileHandler;
use crate::settings::{Settings, SettingsIdentity, SettingsWithEditor};
use crate::utils::growth_guard::GrowthGuard;
use crate::workspace::{
    CodeAction, DefinitionReference, FixAction, FixFileMode, GetSyntaxTreeResult,
    GoToDefinitionResult, PatternId, PullActionsResult, PullDiagnosticsAndActionsResult,
    RenameResult, SearchQuery,
};
use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    ActionFilter, AnalyzerAction, AnalyzerDiagnostic, AnalyzerOptions, AnalyzerPluginVec,
    AnalyzerSignal, ControlFlow, FixKind, GroupCategory, Never, PLUGIN_GROUP, Queryable,
    RegistryVisitor, Rule, RuleCategories, RuleCategory, RuleError, RuleFilter, RuleGroup,
};
use biome_configuration::Rules;
use biome_configuration::analyzer::assist::Actions;
use biome_configuration::analyzer::{AnalyzerSelector, RuleDomainValue, RuleDomains};
#[cfg(feature = "lang_css")]
use biome_css_analyze::METADATA as css_metadata;
#[cfg(feature = "lang_css")]
use biome_css_syntax::CssLanguage;
use biome_db::{AnyParsedSource, ParsedSnippet, ParsedSource};
use biome_diagnostics::{Applicability, Diagnostic, DiagnosticExt, Error, Severity, category};
use biome_formatter::Printed;
use biome_fs::BiomePath;
#[cfg(feature = "lang_graphql")]
use biome_graphql_analyze::METADATA as graphql_metadata;
#[cfg(feature = "lang_graphql")]
use biome_graphql_syntax::GraphqlLanguage;
#[cfg(feature = "lang_html")]
use biome_html_syntax::HtmlLanguage;
#[cfg(feature = "lang_js")]
use biome_js_analyze::METADATA as js_metadata;
#[cfg(feature = "lang_js")]
use biome_js_parser::{JsParserOptions, parse};
#[cfg(feature = "lang_js")]
use biome_js_syntax::{AnyJsModuleItem, JsLanguage, JsxAttribute, JsxAttributeList};
use biome_json_analyze::METADATA as json_metadata;
use biome_json_syntax::JsonLanguage;
#[cfg(feature = "lang_js")]
use biome_languages::javascript::{
    JsEmbeddingKind, JsFileSource, Language, LanguageVariant, SvelteFileKind,
};
use biome_languages::{DocumentFileSource, LanguageDb};
#[cfg(feature = "module_graph")]
use biome_module_graph::ModuleDb;
use biome_package::{Dependencies, PackageJson};
use biome_parser::AnyParse;
use biome_project_layout::ProjectLayout;
#[cfg(feature = "lang_js")]
use biome_rowan::TokenText;
use biome_rowan::{BatchMutation, NodeCache, SendNode, SyntaxNode, TextRange, TextSize};
use biome_text_edit::TextEdit;
use camino::{Utf8Path, Utf8PathBuf};
#[cfg(feature = "lang_html")]
use html::HtmlFileHandler;
#[cfg(feature = "lang_js")]
pub use javascript::JsFormatterSettings;
use rustc_hash::FxHashSet;
#[cfg(test)]
use salsa::plumbing::ZalsaDatabase;
use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
#[cfg(feature = "module_graph")]
use std::rc::Rc;
use std::sync::Arc;

/// Characters that will enable the format on type
pub const ON_TYPE_CHARS: &[char] = &['}', ']', ')'];

pub(crate) fn matches_on_type_char(value: &str) -> bool {
    if value.len() != 1 || value.is_empty() {
        return false;
    }

    // SATEFY: we checked that we have exactly one character.
    ON_TYPE_CHARS.contains(&value.chars().next().unwrap())
}

pub struct FixAllParams<'a> {
    pub(crate) parsed_source: ParsedOrigin,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) workspace_db: WorkspaceDb,
    #[cfg(feature = "module_graph")]
    pub(crate) module_db: Rc<dyn ModuleDb>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) document_file_source: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) rule_categories: RuleCategories,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    pub(crate) collect_final_diagnostics: bool,
}

/// A small wrapper for the parsed file.
#[derive(Clone, Debug)]
pub(crate) enum ParsedOrigin {
    /// The parse result has been queried from the database.
    Workspace(AnyParsedSource),
    /// The parse result has been generated from the server during an in-flight operation, and it won't
    /// be saved back to the workspace. This is usually used for in-memory operations or stateless operations.
    Interned {
        parse: AnyParse,
        diagnostic_offset: Option<TextSize>,
        snippets: Vec<ParsedSnippetOrigin>,
    },
}

#[derive(Clone, Debug)]
pub(crate) enum ParsedSnippetOrigin {
    /// The parse result has been queried from the database.
    Workspace(ParsedSnippet),
    /// The parse result has been generated from the server during an in-flight operation, and it won't
    /// be saved back to the workspace. This is usually used for in-memory operations or stateless operations.
    Interned {
        parse: AnyParse,
        content: EmbedContent,
        file_source: DocumentFileSource,
    },
}

impl ParsedSnippetOrigin {
    pub(crate) fn parsed_origin(&self) -> ParsedOrigin {
        match self {
            Self::Workspace(snippet) => (*snippet).into(),
            Self::Interned { parse, content, .. } => {
                ParsedOrigin::interned(parse.clone(), Some(content.content_offset))
            }
        }
    }

    pub(crate) fn file_source(&self, db: &WorkspaceDb) -> Option<DocumentFileSource> {
        match self {
            Self::Workspace(snippet) => db.source_from_index(snippet.document_source_index(db)),
            Self::Interned { file_source, .. } => Some(*file_source),
        }
    }

    pub(crate) fn element_range(&self, db: &WorkspaceDb) -> TextRange {
        match self {
            Self::Workspace(snippet) => snippet.element_range(db),
            Self::Interned { content, .. } => content.element_range,
        }
    }

    pub(crate) fn content_range(&self, db: &WorkspaceDb) -> TextRange {
        match self {
            Self::Workspace(snippet) => snippet.content_range(db),
            Self::Interned { content, .. } => content.content_range,
        }
    }

    pub(crate) fn content_offset(&self, db: &WorkspaceDb) -> TextSize {
        match self {
            Self::Workspace(snippet) => snippet.content_offset(db),
            Self::Interned { content, .. } => content.content_offset,
        }
    }

    pub(crate) fn diagnostics<'a>(
        &'a self,
        db: &'a WorkspaceDb,
    ) -> &'a [biome_parser::diagnostic::ParseDiagnostic] {
        match self {
            Self::Workspace(snippet) => snippet.parsed(db).diagnostics(),
            Self::Interned { parse, .. } => parse.diagnostics(),
        }
    }

    pub(crate) fn serde_diagnostics(
        &self,
        db: &WorkspaceDb,
    ) -> Vec<biome_diagnostics::serde::Diagnostic> {
        match self {
            Self::Workspace(snippet) => snippet.serde_diagnostics(db),
            Self::Interned { parse, .. } => parse.clone().into_serde_diagnostics(),
        }
    }

    pub(crate) fn has_errors(&self, db: &WorkspaceDb) -> bool {
        self.diagnostics(db)
            .iter()
            .any(|diagnostic| diagnostic.severity() >= Severity::Error)
    }

    pub(crate) fn error_count(&self, db: &WorkspaceDb) -> usize {
        self.diagnostics(db)
            .iter()
            .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
            .count()
    }
}

impl ParsedOrigin {
    pub(crate) fn interned(parse: AnyParse, diagnostic_offset: Option<TextSize>) -> Self {
        Self::Interned {
            parse,
            diagnostic_offset,
            snippets: Vec::new(),
        }
    }

    pub(crate) fn interned_document(parse: AnyParse, snippets: Vec<ParsedSnippetOrigin>) -> Self {
        Self::Interned {
            parse,
            diagnostic_offset: None,
            snippets,
        }
    }

    pub(crate) fn tree<N>(&self, db: &WorkspaceDb) -> N
    where
        N: biome_rowan::AstNode,
        N::Language: 'static,
    {
        match self {
            Self::Workspace(source) => source.tree(db),
            Self::Interned { parse, .. } => parse.tree(),
        }
    }

    pub(crate) fn syntax<L>(&self, db: &WorkspaceDb) -> SyntaxNode<L>
    where
        L: biome_rowan::Language + 'static,
    {
        match self {
            Self::Workspace(source) => source.syntax(db),
            Self::Interned { parse, .. } => parse.syntax(),
        }
    }

    pub(crate) fn parse(&self, db: &WorkspaceDb) -> AnyParse {
        match self {
            Self::Workspace(source) => source.any_parse(db).clone(),
            Self::Interned { parse, .. } => parse.clone(),
        }
    }

    pub(crate) fn send_node(&self, db: &WorkspaceDb) -> SendNode {
        match self {
            Self::Workspace(source) => source.any_parse(db).unwrap_as_send_node(),
            Self::Interned { parse, .. } => parse.unwrap_as_send_node(),
        }
    }

    pub(crate) fn diagnostics<'a>(
        &'a self,
        db: &'a WorkspaceDb,
    ) -> &'a [biome_parser::diagnostic::ParseDiagnostic] {
        match self {
            Self::Workspace(source) => source.diagnostics(db),
            Self::Interned { parse, .. } => parse.diagnostics(),
        }
    }

    pub(crate) fn serde_diagnostics(
        &self,
        db: &WorkspaceDb,
    ) -> Vec<biome_diagnostics::serde::Diagnostic> {
        match self {
            Self::Workspace(source) => source.serde_diagnostics(db),
            Self::Interned { parse, .. } => parse.clone().into_serde_diagnostics(),
        }
    }

    pub(crate) fn diagnostic_offset(&self, db: &WorkspaceDb) -> Option<TextSize> {
        match self {
            Self::Workspace(source) => source.diagnostic_offset(db),
            Self::Interned {
                diagnostic_offset, ..
            } => *diagnostic_offset,
        }
    }
}

impl From<AnyParsedSource> for ParsedOrigin {
    fn from(source: AnyParsedSource) -> Self {
        Self::Workspace(source)
    }
}

impl From<ParsedSource> for ParsedOrigin {
    fn from(source: ParsedSource) -> Self {
        Self::Workspace(source.into())
    }
}

impl From<ParsedSnippet> for ParsedOrigin {
    fn from(source: ParsedSnippet) -> Self {
        Self::Workspace(source.into())
    }
}

impl From<&ParsedSnippet> for ParsedOrigin {
    fn from(source: &ParsedSnippet) -> Self {
        Self::Workspace((*source).into())
    }
}

impl From<AnyParse> for ParsedOrigin {
    fn from(parse: AnyParse) -> Self {
        Self::interned(parse, None)
    }
}

pub(crate) enum SnippetsIterator<'a> {
    Workspace(std::slice::Iter<'a, ParsedSnippet>),
    Interned(std::slice::Iter<'a, ParsedSnippetOrigin>),
}

impl Iterator for SnippetsIterator<'_> {
    type Item = ParsedSnippetOrigin;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Workspace(snippets) => {
                snippets.next().copied().map(ParsedSnippetOrigin::Workspace)
            }
            Self::Interned(snippets) => snippets.next().cloned(),
        }
    }
}

pub(crate) struct FixedFileResult {
    pub(crate) root: SendNode,
    pub(crate) skipped_suggested_fixes: u32,
    pub(crate) actions: Vec<FixAction>,
    pub(crate) errors: usize,
}

#[derive(Default)]
/// The list of capabilities that are available for a language
pub struct Capabilities {
    pub(crate) parser: ParserCapabilities,
    pub(crate) debug: DebugCapabilities,
    pub(crate) analyzer: AnalyzerCapabilities,
    pub(crate) formatter: FormatterCapabilities,
    pub(crate) search: SearchCapabilities,
    pub(crate) enabled_for_path: EnabledForPath,
    pub(crate) editors: EditorCapabilities,
}

#[derive(Clone)]
pub struct ParseResult {
    pub(crate) any_parse: AnyParse,
    pub(crate) language: Option<DocumentFileSource>,
}

#[derive(Default)]
pub struct ParseEmbedResult {
    pub(crate) nodes: Vec<(AnyParse, EmbedContent, DocumentFileSource)>,
}

pub(crate) struct ParseEmbeddedParams<'a, 'settings> {
    pub(crate) any_parse: &'a AnyParse,
    pub(crate) path: &'a BiomePath,
    pub(crate) file_source: &'a DocumentFileSource,
    pub(crate) settings: &'a SettingsWithEditor<'settings>,
    pub(crate) node_cache: &'a mut NodeCache,
}

type Parse =
    fn(&BiomePath, DocumentFileSource, &str, &SettingsWithEditor, &mut NodeCache) -> ParseResult;
type ParseEmbeddedNodes =
    for<'a, 'settings> fn(ParseEmbeddedParams<'a, 'settings>) -> ParseEmbedResult;
#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,

    pub(crate) parse_embedded_nodes: Option<ParseEmbeddedNodes>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParsedSource, TextSize, WorkspaceDb) -> String;
type DebugFormatterIR = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    WorkspaceDb,
) -> Result<String, WorkspaceError>;
type DebugTypeInfo = fn(AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>;
type DebugRegisteredTypes =
    fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>;
type DebugSemanticModel =
    fn(&BiomePath, AnyParsedSource, WorkspaceDb) -> Result<String, WorkspaceError>;

#[derive(Default)]
pub struct DebugCapabilities {
    /// Prints the syntax tree
    pub(crate) debug_syntax_tree: Option<DebugSyntaxTree>,
    /// Prints the control flow graph
    pub(crate) debug_control_flow: Option<DebugControlFlow>,
    /// Prints the formatter IR
    pub(crate) debug_formatter_ir: Option<DebugFormatterIR>,
    /// Prints the type info
    pub(crate) debug_type_info: Option<DebugTypeInfo>,
    /// Prints the registered types
    pub(crate) debug_registered_types: Option<DebugRegisteredTypes>,
    /// Prints the binding/scope tree of the semantic model
    pub(crate) debug_semantic_model: Option<DebugSemanticModel>,
}

pub(crate) struct LintParams<'a> {
    pub(crate) parsed_source: ParsedOrigin,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) workspace_db: WorkspaceDb,
    #[cfg(feature = "module_graph")]
    pub(crate) module_db: Rc<dyn ModuleDb>,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) pull_code_actions: bool,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    pub(crate) max_diagnostics: Option<u32>,
    pub(crate) diagnostic_level: Severity,
    /// When true, promote assist diagnostics (`assist/*`) to error severity.
    pub(crate) enforce_assist: bool,
}

pub(crate) struct DiagnosticsAndActionsParams<'a> {
    pub(crate) parsed_source: AnyParsedSource,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) workspace_db: WorkspaceDb,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) working_directory: Option<&'a Utf8Path>,
}

#[derive(Debug, Default)]
pub(crate) struct LintResults {
    pub(crate) diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    pub(crate) errors: usize,
    pub(crate) skipped_diagnostics: u32,
    pub(crate) infos: usize,
    pub(crate) warnings: usize,
}

pub(crate) struct ProcessLint<'a> {
    diagnostic_count: u32,
    errors: usize,
    warnings: usize,
    infos: usize,
    diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
    ignores_suppression_comment: bool,
    rules: Option<Cow<'a, Rules>>,
    pull_code_actions: bool,
    diagnostic_offset: Option<TextSize>,
    max_diagnostics: Option<u32>,
    diagnostic_level: Severity,
    enforce_assist: bool,
}

impl<'a> ProcessLint<'a> {
    pub(crate) fn new(params: &'a LintParams<'_>) -> Self {
        Self {
            diagnostic_count: params.parsed_source.diagnostics(&params.workspace_db).len() as u32,
            errors: Default::default(),
            warnings: Default::default(),
            infos: Default::default(),
            diagnostics: Default::default(),
            // Do not report unused suppression comment diagnostics if:
            // - it is a syntax-only analyzer pass, or
            // - if a single rule is run, or
            // - if rules or domains are skipped.
            ignores_suppression_comment: !params.categories.contains(RuleCategory::Lint)
                || !params.only.is_empty()
                || !params.skip.is_empty(),
            rules: params.settings.linter_rules(),
            pull_code_actions: params.pull_code_actions,
            diagnostic_offset: params.parsed_source.diagnostic_offset(&params.workspace_db),
            max_diagnostics: params.max_diagnostics,
            diagnostic_level: params.diagnostic_level,
            enforce_assist: params.enforce_assist,
        }
    }

    pub(crate) fn process_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow {
        if let Some(mut diagnostic) = signal.diagnostic() {
            if self.ignores_suppression_comment
                && diagnostic.category() == Some(category!("suppressions/unused"))
            {
                return ControlFlow::<Never>::Continue(());
            }

            // Resolve the final severity for this diagnostic:
            // 1. Lint rules may have configured severity overrides.
            // 2. Assist diagnostics are promoted to Error when enforce_assist is set.
            let category = diagnostic.category();
            let mut severity = category
                .filter(|cat| cat.name().starts_with("lint/"))
                .and_then(|cat| {
                    self.rules.as_ref().and_then(|rules| {
                        rules.get_severity_from_category(cat, diagnostic.severity())
                    })
                })
                .or_else(|| Some(diagnostic.severity()))
                .unwrap_or(Severity::Warning);

            if self.enforce_assist && category.is_some_and(|cat| cat.name().starts_with("assist/"))
            {
                severity = Severity::Error;
            }

            if severity < self.diagnostic_level {
                return ControlFlow::<Never>::Continue(());
            }

            match severity {
                Severity::Error | Severity::Fatal => {
                    self.errors += 1;
                }
                Severity::Information => {
                    self.infos += 1;
                }
                Severity::Warning => self.warnings += 1,
                Severity::Hint => {}
            }

            if self
                .max_diagnostics
                .is_none_or(|max_diagnostics| self.diagnostic_count <= max_diagnostics)
            {
                if self.pull_code_actions {
                    for action in signal.actions(ActionFilter::rule_fix()) {
                        diagnostic = diagnostic.add_code_suggestion(action.into());
                    }
                }
                if let Some(offset) = &self.diagnostic_offset {
                    diagnostic.add_diagnostic_offset(*offset);
                }

                let error = diagnostic.with_severity(severity);

                self.diagnostics
                    .push(biome_diagnostics::serde::Diagnostic::new(error));
            }
            self.diagnostic_count += 1;
        }

        ControlFlow::<Never>::Continue(())
    }

    pub(crate) fn into_result(
        self,
        parse_diagnostics: Vec<biome_diagnostics::serde::Diagnostic>,
        analyzer_diagnostics: Vec<biome_diagnostics::Error>,
    ) -> LintResults {
        let mut parse_errors = 0usize;
        let mut parse_warnings = 0usize;
        let mut parse_infos = 0usize;
        let mut diagnostics: Vec<_> = parse_diagnostics
            .into_iter()
            .filter(|diag| diag.severity() >= self.diagnostic_level)
            .inspect(|diag| match diag.severity() {
                Severity::Error | Severity::Fatal => parse_errors += 1,
                Severity::Warning => parse_warnings += 1,
                Severity::Information => parse_infos += 1,
                Severity::Hint => {}
            })
            .collect();

        diagnostics.extend(self.diagnostics);

        let mut analyzer_errors = 0usize;
        let mut analyzer_warnings = 0usize;
        let mut analyzer_infos = 0usize;
        diagnostics.extend(
            analyzer_diagnostics
                .into_iter()
                .map(biome_diagnostics::serde::Diagnostic::new)
                .filter(|diag| diag.severity() >= self.diagnostic_level)
                .inspect(|diag| match diag.severity() {
                    Severity::Error | Severity::Fatal => analyzer_errors += 1,
                    Severity::Warning => analyzer_warnings += 1,
                    Severity::Information => analyzer_infos += 1,
                    Severity::Hint => {}
                })
                .collect::<Vec<_>>(),
        );
        let skipped_diagnostics = self
            .diagnostic_count
            .saturating_sub(diagnostics.len() as u32);

        LintResults {
            errors: parse_errors + self.errors + analyzer_errors,
            skipped_diagnostics,
            diagnostics,
            infos: parse_infos + self.infos + analyzer_infos,
            warnings: parse_warnings + self.warnings + analyzer_warnings,
        }
    }
}

/// Use this type to process fix all actions
pub(crate) struct ProcessFixAll<'a> {
    fix_file_mode: &'a FixFileMode,
    errors: usize,
    rules: Option<Cow<'a, Rules>>,
    skipped_suggested_fixes: u32,
    actions: Vec<FixAction>,
    growth_guard: GrowthGuard,
}

impl<'a> ProcessFixAll<'a> {
    pub(crate) fn new(params: &'a FixAllParams, syntax_len: u32) -> Self {
        Self {
            fix_file_mode: &params.fix_file_mode,
            errors: 0,
            rules: params.settings.linter_rules(),
            skipped_suggested_fixes: 0,
            actions: Vec::new(),
            growth_guard: GrowthGuard::new(syntax_len),
        }
    }

    /// Collects all applicable actions from the signal instead of
    /// breaking on the first one. The analyzer runs to completion, processing every signal.
    pub(crate) fn collect_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
        pending: &mut Vec<AnalyzerAction<L>>,
    ) -> ControlFlow<Never> {
        let current_diagnostic = signal.diagnostic();

        if let Some(diagnostic) = current_diagnostic.as_ref()
            && is_diagnostic_error(diagnostic, self.rules.as_deref())
        {
            self.errors += 1;
        }

        let action_filter = match self.fix_file_mode {
            FixFileMode::ApplySuppressions => ActionFilter::inline_suppression(),
            FixFileMode::SafeFixes | FixFileMode::SafeAndUnsafeFixes => ActionFilter::rule_fix(),
        };
        for action in signal.actions(action_filter) {
            match self.fix_file_mode {
                FixFileMode::ApplySuppressions => {
                    if action.is_suppression() {
                        pending.push(action);
                        // Take only the first suppression action per signal
                        // (inline), not the top-level one as well.
                        break;
                    }
                }
                FixFileMode::SafeFixes => {
                    if action.applicability == Applicability::MaybeIncorrect {
                        self.skipped_suggested_fixes += 1;
                    }
                    if action.applicability == Applicability::Always {
                        self.errors = self.errors.saturating_sub(1);
                        pending.push(action);
                    }
                }
                FixFileMode::SafeAndUnsafeFixes => {
                    if matches!(
                        action.applicability,
                        Applicability::Always | Applicability::MaybeIncorrect
                    ) {
                        self.errors = self.errors.saturating_sub(1);
                        pending.push(action);
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }

    /// Phase 1 callback: collect applicable fix actions without counting errors.
    /// Error counting is deferred to Phase 2 where all rules run on the final tree.
    pub(crate) fn collect_signal_fixes_only<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
        pending: &mut Vec<AnalyzerAction<L>>,
    ) -> ControlFlow<Never> {
        let action_filter = match self.fix_file_mode {
            FixFileMode::ApplySuppressions => ActionFilter::inline_suppression(),
            FixFileMode::SafeFixes | FixFileMode::SafeAndUnsafeFixes => ActionFilter::rule_fix(),
        };
        for action in signal.actions(action_filter) {
            match self.fix_file_mode {
                FixFileMode::ApplySuppressions => {
                    if action.is_suppression() {
                        pending.push(action);
                        break;
                    }
                }
                FixFileMode::SafeFixes => {
                    if action.applicability == Applicability::MaybeIncorrect {
                        self.skipped_suggested_fixes += 1;
                    }
                    if action.applicability == Applicability::Always {
                        pending.push(action);
                    }
                }
                FixFileMode::SafeAndUnsafeFixes => {
                    if matches!(
                        action.applicability,
                        Applicability::Always | Applicability::MaybeIncorrect
                    ) {
                        pending.push(action);
                    }
                }
            }
        }

        ControlFlow::Continue(())
    }

    /// Phase 2 callback: count remaining errors on the fixed tree without collecting actions.
    pub(crate) fn collect_diagnostic_only<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow<Never> {
        if let Some(diagnostic) = signal.diagnostic().as_ref()
            && is_diagnostic_error(diagnostic, self.rules.as_deref())
        {
            self.errors += 1;
        }
        ControlFlow::Continue(())
    }

    /// Merge pending actions from the same rule into one mutation and commit.
    ///
    /// Only actions matching the first rule are merged and applied. Remaining
    /// rules are handled in subsequent iterations of the `fix_all` loop. This
    /// avoids merging mutations from different rules which may conflict.
    ///
    /// Returns `Some(())` if any fixes were applied, `None` if pending was empty.
    pub(crate) fn process_batch_actions<T, L>(
        &mut self,
        pending: Vec<AnalyzerAction<L>>,
        mut update_tree_return_text_len: T,
    ) -> Result<Option<()>, WorkspaceError>
    where
        T: FnMut(SyntaxNode<L>) -> Option<u32>,
        L: biome_rowan::Language,
    {
        if pending.is_empty() {
            return Ok(None);
        }

        let target_rule = pending[0].rule_name;
        let mut master: Option<BatchMutation<L>> = None;
        let mut count = 0usize;

        for action in pending {
            if action.rule_name != target_rule {
                continue;
            }
            match &mut master {
                Some(m) => m.merge(action.mutation),
                None => master = Some(action.mutation),
            }
            count += 1;
        }

        let Some(master) = master else {
            return Ok(None);
        };

        let (root, text_range_and_edit) = master.commit_with_text_range_and_edit(true);
        if let Some((range, _)) = text_range_and_edit {
            let Some(curr_len) = update_tree_return_text_len(root) else {
                return Err(WorkspaceError::RuleError(
                    RuleError::ReplacedRootWithNonRootError {
                        rule_name: target_rule.map(|(g, r)| (Cow::Borrowed(g), Cow::Borrowed(r))),
                    },
                ));
            };

            for _ in 0..count {
                self.actions.push(FixAction {
                    rule_name: target_rule.map(|(g, r)| (Cow::Borrowed(g), Cow::Borrowed(r))),
                    range,
                });
            }

            if !self.growth_guard.check(curr_len) {
                let seen_rules: HashSet<_> = self
                    .actions
                    .iter()
                    .rev()
                    .take(10)
                    .filter_map(|a| a.rule_name.clone())
                    .collect();
                return Err(WorkspaceError::RuleError(
                    RuleError::ConflictingRuleFixesError {
                        rules: seen_rules.into_iter().collect(),
                    },
                ));
            }
        }

        Ok(Some(()))
    }

    /// Record a text-edit-based fix (e.g. from a plugin rewrite) that was
    /// applied outside of the normal mutation path.
    pub(crate) fn record_text_edit_fix(
        &mut self,
        range: TextRange,
        new_text_len: u32,
        rule_name: Option<(&'static str, &'static str)>,
    ) -> Result<(), WorkspaceError> {
        self.actions.push(FixAction {
            rule_name: rule_name.map(|(g, r)| (Cow::Borrowed(g), Cow::Borrowed(r))),
            range,
        });
        if !self.growth_guard.check(new_text_len) {
            return Err(WorkspaceError::RuleError(
                RuleError::ConflictingRuleFixesError {
                    rules: self
                        .actions
                        .iter()
                        .rev()
                        .take(10)
                        .filter_map(|action| action.rule_name.clone())
                        .collect::<HashSet<_>>()
                        .into_iter()
                        .collect(),
                },
            ));
        }
        Ok(())
    }

    /// Apply a plugin text edit if present and the text actually changed.
    /// Returns `Some(new_text)` if the edit was applied, `None` otherwise.
    pub(crate) fn apply_plugin_text_edit(
        &mut self,
        (range, edit): (TextRange, TextEdit),
        current_text: &str,
    ) -> Result<Option<String>, WorkspaceError> {
        let new_text = edit.new_string(current_text);
        if new_text == current_text {
            return Ok(None);
        }
        self.record_text_edit_fix(range, new_text.len() as u32, Some(("plugin", "gritql")))?;
        Ok(Some(new_text))
    }

    pub(crate) fn finish(self, root: SendNode) -> FixedFileResult {
        FixedFileResult {
            root,
            skipped_suggested_fixes: self.skipped_suggested_fixes,
            actions: self.actions,
            errors: self.errors,
        }
    }
}

pub(crate) struct ProcessDiagnosticsAndActions {
    diagnostics: Vec<(biome_diagnostics::serde::Diagnostic, Vec<CodeAction>)>,
    diagnostic_offset: Option<TextSize>,
}

impl ProcessDiagnosticsAndActions {
    pub(crate) fn new(diagnostic_offset: Option<TextSize>) -> Self {
        Self {
            diagnostics: Vec::new(),
            diagnostic_offset,
        }
    }

    pub(crate) fn process_signal<L: biome_rowan::Language>(
        &mut self,
        signal: &dyn AnalyzerSignal<L>,
    ) -> ControlFlow<Never> {
        let diagnostic = signal.diagnostic();

        if let Some(mut diagnostic) = diagnostic {
            let actions: Vec<_> = signal
                .actions(ActionFilter::all())
                .into_code_action_iter()
                .map(|item| CodeAction {
                    category: item.category.clone(),
                    rule_name: item
                        .rule_name
                        .map(|(group, name)| (Cow::Borrowed(group), Cow::Borrowed(name))),
                    applicability: Some(item.suggestion.applicability),
                    suggestion: Some(item.suggestion),
                    offset: None,
                })
                .collect();
            if !actions.is_empty() {
                if let Some(offset) = &self.diagnostic_offset {
                    diagnostic.add_diagnostic_offset(*offset);
                }
                self.diagnostics.push((
                    biome_diagnostics::serde::Diagnostic::new(Error::from(diagnostic)),
                    actions,
                ));
            }
        }

        ControlFlow::<Never>::Continue(())
    }

    pub(crate) fn finish(self) -> PullDiagnosticsAndActionsResult {
        PullDiagnosticsAndActionsResult {
            diagnostics: self.diagnostics,
        }
    }
}

pub(crate) struct CodeActionsParams<'a> {
    pub(crate) parsed_source: AnyParsedSource,
    pub(crate) range: Option<TextRange>,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) path: &'a BiomePath,
    pub(crate) workspace_db: WorkspaceDb,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) language: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) categories: RuleCategories,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    /// When `false`, actions are returned with `suggestion: None` (no mutations computed).
    pub(crate) compute_actions: bool,
}

pub(crate) struct UpdateSnippetsNodes {
    pub(crate) range: TextRange,
    pub(crate) new_code: String,
    /// When `true`, `new_code` needs to be re-indented to match the
    /// host's nesting level. When `false`, `new_code` already has the
    /// right shape and can be spliced back as-is.
    pub(crate) needs_reindent: bool,
    /// Ranges inside `new_code` that must not be re-indented: the bodies of
    /// template literals and multi-line block comments. Lines that begin
    /// inside one of these ranges are left as-is instead of receiving the
    /// host element's indentation prefix.
    pub(crate) verbatim_ranges: Vec<TextRange>,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions = fn(CodeActionsParams) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<Option<FixedFileResult>, WorkspaceError>;
type Rename = fn(
    &BiomePath,
    AnyParsedSource,
    TextSize,
    String,
    WorkspaceDb,
) -> Result<RenameResult, WorkspaceError>;
type UpdateSnippets =
    fn(ParsedOrigin, WorkspaceDb, Vec<UpdateSnippetsNodes>) -> Result<SendNode, WorkspaceError>;
type PullDiagnosticsAndActions = fn(DiagnosticsAndActionsParams) -> PullDiagnosticsAndActionsResult;

#[derive(Default)]
pub struct AnalyzerCapabilities {
    /// It lints a file
    pub(crate) lint: Option<Lint>,
    /// It extracts code actions for a file
    pub(crate) code_actions: Option<CodeActions>,
    /// Applies fixes to a file
    pub(crate) fix_all: Option<FixAll>,
    /// It renames a binding inside a file
    pub(crate) rename: Option<Rename>,
    /// It updates the snippets contained in the original root
    pub(crate) update_snippets: Option<UpdateSnippets>,
    /// Pulls diagnostics with relative code actions
    pub(crate) pull_diagnostics_and_actions: Option<PullDiagnosticsAndActions>,
}

type Format = fn(
    &BiomePath,
    &DocumentFileSource,
    ParsedOrigin,
    &SettingsWithEditor,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;
type FormatRange = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    TextRange,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;
type FormatOnType = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &SettingsWithEditor,
    TextSize,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;

pub(crate) fn format_on_type_noop(offset: TextSize) -> Printed {
    // The LSP layer treats `range: None` as a whole-file replacement.
    Printed::new(
        String::new(),
        Some(TextRange::at(offset, TextSize::from(0))),
        Vec::new(),
        Vec::new(),
    )
}

type FormatEmbedded = fn(
    &BiomePath,
    &DocumentFileSource,
    ParsedOrigin,
    &SettingsWithEditor,
    Vec<ParsedSnippetOrigin>,
    WorkspaceDb,
) -> Result<Printed, WorkspaceError>;

#[derive(Default)]
pub(crate) struct FormatterCapabilities {
    /// It formats a file
    pub(crate) format: Option<Format>,
    /// It formats a portion of text of a file
    pub(crate) format_range: Option<FormatRange>,
    /// It formats a file while typing
    pub(crate) format_on_type: Option<FormatOnType>,
    /// It formats a file with embedded nodes
    pub(crate) format_embedded: Option<FormatEmbedded>,
}

type Enabled = fn(&Utf8Path, &SettingsWithEditor) -> bool;

type Search = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParsedSource,
    &dyn SearchQuery,
    &SettingsWithEditor,
    PatternId,
    WorkspaceDb,
) -> Result<Vec<TextRange>, WorkspaceError>;

#[derive(Default)]
pub(crate) struct SearchCapabilities {
    /// It searches through a file
    pub(crate) search: Option<Search>,
}

#[derive(Default)]
pub(crate) struct EnabledForPath {
    pub(crate) formatter: Option<Enabled>,
    pub(crate) linter: Option<Enabled>,
    pub(crate) assist: Option<Enabled>,
    pub(crate) search: Option<Enabled>,
}

#[derive(Default)]
pub(crate) struct EditorCapabilities {
    pub(crate) resolve_binding: Option<ResolveBinding>,
    pub(crate) resolve_definition: Option<ResolveDefinition>,
}

pub(crate) struct ResolveBindingParams {
    pub(crate) parsed_source: AnyParsedSource,
    pub(crate) cursor_offset: TextSize,
    pub(crate) workspace_db: WorkspaceDb,
    pub(crate) path: Utf8PathBuf,
}

pub(crate) struct ResolveDefinitionParams<'a> {
    pub(crate) path: &'a BiomePath,
    pub(crate) definition_ref: &'a DefinitionReference,
    pub(crate) workspace_db: WorkspaceDb,
    pub(crate) parsed_source: AnyParsedSource,
}

type ResolveBinding = fn(ResolveBindingParams) -> Option<DefinitionReference>;
type ResolveDefinition = fn(ResolveDefinitionParams) -> Option<GoToDefinitionResult>;

/// Main trait to use to add a new language to Biome
pub(crate) trait ExtensionHandler {
    /// Capabilities that can applied to a file
    fn capabilities(&self) -> Capabilities {
        Capabilities::default()
    }
}

/// Features available for each language
pub(crate) struct Features {
    #[cfg(feature = "lang_js")]
    js: JsFileHandler,
    json: JsonFileHandler,
    #[cfg(feature = "lang_css")]
    css: CssFileHandler,
    #[cfg(feature = "lang_js")]
    astro: AstroFileHandler,
    #[cfg(all(feature = "lang_js", feature = "lang_html"))]
    vue: VueFileHandler,
    #[cfg(all(feature = "lang_js", feature = "lang_html"))]
    svelte: SvelteFileHandler,
    unknown: UnknownFileHandler,
    #[cfg(feature = "lang_graphql")]
    graphql: GraphqlFileHandler,
    #[cfg(feature = "lang_html")]
    html: HtmlFileHandler,
    #[cfg(feature = "lang_grit")]
    grit: grit::GritFileHandler,
    #[cfg(feature = "lang_md")]
    markdown: md::MarkdownFileHandler,
    #[cfg(feature = "lang_yaml")]
    yaml: yaml::YamlFileHandler,
    ignore: IgnoreFileHandler,
}

impl Features {
    pub(crate) fn new() -> Self {
        Self {
            #[cfg(feature = "lang_js")]
            js: JsFileHandler {},
            json: JsonFileHandler {},
            #[cfg(feature = "lang_css")]
            css: CssFileHandler {},
            #[cfg(feature = "lang_js")]
            astro: AstroFileHandler {},
            #[cfg(all(feature = "lang_js", feature = "lang_html"))]
            svelte: SvelteFileHandler {},
            #[cfg(all(feature = "lang_js", feature = "lang_html"))]
            vue: VueFileHandler {},
            #[cfg(feature = "lang_graphql")]
            graphql: GraphqlFileHandler {},
            #[cfg(feature = "lang_html")]
            html: HtmlFileHandler {},
            #[cfg(feature = "lang_grit")]
            grit: grit::GritFileHandler {},
            #[cfg(feature = "lang_md")]
            markdown: md::MarkdownFileHandler {},
            ignore: IgnoreFileHandler {},
            #[cfg(feature = "lang_yaml")]
            yaml: yaml::YamlFileHandler {},
            unknown: UnknownFileHandler::default(),
        }
    }

    /// Returns the [Capabilities] associated with a document source.
    ///
    /// ## Warning
    ///
    /// This method is deprecated and shouldn't be used unless you're working on a feature for the deprecated
    /// partial support of vue/svelte/astro
    // TODO: remove match once we remove vue/astro/svelte handlers
    pub(crate) fn get_deprecated_capabilities(
        &self,
        language_hint: DocumentFileSource,
    ) -> Capabilities {
        match language_hint {
            #[cfg(feature = "lang_js")]
            DocumentFileSource::Js(source) => match source.as_embedding_kind() {
                JsEmbeddingKind::Astro { .. } => self.astro.capabilities(),
                #[cfg(feature = "lang_html")]
                JsEmbeddingKind::Vue { .. } => self.vue.capabilities(),
                #[cfg(not(feature = "lang_html"))]
                JsEmbeddingKind::Vue { .. } => self.js.capabilities(),
                // `.svelte.ts` / `.svelte.js` are full JS/TS modules with Svelte
                // semantics; `.svelte` component documents still use the Svelte handler.
                JsEmbeddingKind::Svelte {
                    file_kind: SvelteFileKind::SourceModule,
                    ..
                } => self.js.capabilities(),
                #[cfg(feature = "lang_html")]
                JsEmbeddingKind::Svelte {
                    file_kind: SvelteFileKind::Component,
                    ..
                } => self.svelte.capabilities(),
                #[cfg(not(feature = "lang_html"))]
                JsEmbeddingKind::Svelte {
                    file_kind: SvelteFileKind::Component,
                    ..
                } => self.js.capabilities(),
                JsEmbeddingKind::None => self.js.capabilities(),
            },
            DocumentFileSource::Json(_) => self.json.capabilities(),
            #[cfg(feature = "lang_css")]
            DocumentFileSource::Css(_) => self.css.capabilities(),
            #[cfg(feature = "lang_graphql")]
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            #[cfg(feature = "lang_html")]
            DocumentFileSource::Html(_) => self.html.capabilities(),
            #[cfg(feature = "lang_grit")]
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            #[cfg(feature = "lang_md")]
            DocumentFileSource::Markdown(_) => self.markdown.capabilities(),
            #[cfg(feature = "lang_yaml")]
            DocumentFileSource::Yaml(_) => self.yaml.capabilities(),
            DocumentFileSource::Ignore => self.ignore.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
            #[expect(
                clippy::allow_attributes,
                reason = "`unreachable_patterns` is feature-dependent here; `expect(unreachable_patterns)` is unfulfilled in reduced language builds."
            )]
            #[allow(
                unreachable_patterns,
                reason = "The fallback is reachable when dependency feature unification exposes source variants without enabling their handlers."
            )]
            _ => self.unknown.capabilities(),
        }
    }

    /// Returns the [Capabilities] associated with a document source.
    pub(crate) fn get_real_capabilities(&self, language_hint: DocumentFileSource) -> Capabilities {
        match language_hint {
            #[cfg(feature = "lang_js")]
            DocumentFileSource::Js(_) => self.js.capabilities(),
            DocumentFileSource::Json(_) => self.json.capabilities(),
            #[cfg(feature = "lang_css")]
            DocumentFileSource::Css(_) => self.css.capabilities(),
            #[cfg(feature = "lang_graphql")]
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            #[cfg(feature = "lang_html")]
            DocumentFileSource::Html(_) => self.html.capabilities(),
            #[cfg(feature = "lang_grit")]
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            #[cfg(feature = "lang_md")]
            DocumentFileSource::Markdown(_) => self.markdown.capabilities(),
            #[cfg(feature = "lang_yaml")]
            DocumentFileSource::Yaml(_) => self.yaml.capabilities(),
            DocumentFileSource::Ignore => self.ignore.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
            #[expect(
                clippy::allow_attributes,
                reason = "`unreachable_patterns` is feature-dependent here; `expect(unreachable_patterns)` is unfulfilled in reduced language builds."
            )]
            #[allow(
                unreachable_patterns,
                reason = "The fallback is reachable when dependency feature unification exposes source variants without enabling their handlers."
            )]
            _ => self.unknown.capabilities(),
        }
    }
}

/// Checks whether a diagnostic coming from the analyzer is an [error](Severity::Error)
///
/// The function checks the diagnostic against the current configured rules.
// TODO: this function works only with lint rules, but it should work with assist actions too
pub(crate) fn is_diagnostic_error(
    diagnostic: &'_ AnalyzerDiagnostic,
    rules: Option<&'_ Rules>,
) -> bool {
    let severity = diagnostic
        .category()
        .filter(|category| category.name().starts_with("lint/"))
        .map_or_else(
            || diagnostic.severity(),
            |category| {
                rules
                    .and_then(|rules| {
                        rules.get_severity_from_category(category, diagnostic.severity())
                    })
                    .unwrap_or(Severity::Warning)
            },
        );

    severity >= Severity::Error
}

#[derive(Default)]
#[cfg(feature = "lang_js")]
pub struct ParsedLangAndSetup {
    language: Language,
    variant: LanguageVariant,
    setup: bool,
}

#[cfg(feature = "lang_js")]
fn get_module_item_attributes(item: AnyJsModuleItem) -> Option<JsxAttributeList> {
    let expression = item
        .as_any_js_statement()?
        .as_js_expression_statement()?
        .expression()
        .ok()?;
    let tag = expression.as_jsx_tag_expression()?.tag().ok()?;
    let opening_element = tag.as_jsx_element()?.opening_element().ok()?;
    Some(opening_element.attributes())
}

#[cfg(feature = "lang_js")]
fn get_attribute_value(attribute: &JsxAttribute) -> Option<TokenText> {
    let attribute_value = attribute.initializer()?.value().ok()?;
    let attribute_inner_string = attribute_value.as_jsx_string()?.inner_string_text().ok()?;
    Some(attribute_inner_string)
}

/// Parse the "lang" and "setup" attributes from the opening tag of the "\<script\>" block in Svelte or Vue files.
/// This function will return the language based on the existence or the value of the "lang" attribute.
/// We use the JSX parser at the moment to parse the opening tag. So the opening tag should be first
/// matched by regular expressions.
///
// TODO: We should change the parser when HTMLish languages are supported.
#[cfg(feature = "lang_js")]
pub(crate) fn parse_lang_and_setup_from_script_opening_tag(
    script_opening_tag: &str,
) -> ParsedLangAndSetup {
    let Some(tree) = parse(
        script_opening_tag,
        JsFileSource::jsx(),
        JsParserOptions::default(),
    )
    .try_tree() else {
        return ParsedLangAndSetup::default();
    };

    let Some(js_module) = tree.as_js_module() else {
        return ParsedLangAndSetup::default();
    };

    let mut lang_and_setup = ParsedLangAndSetup::default();
    for item in js_module.items() {
        let Some(attributes) = get_module_item_attributes(item) else {
            continue;
        };
        if attributes.find_by_name("setup").is_some() {
            lang_and_setup.setup = true;
        }
        if let Some(lang_attribute) = attributes.find_by_name("lang")
            && let Some(lang_value) = get_attribute_value(&lang_attribute)
        {
            match lang_value.text() {
                "ts" => {
                    lang_and_setup.language = Language::TypeScript {
                        definition_file: false,
                    };
                    lang_and_setup.variant = LanguageVariant::Standard;
                }
                "tsx" => {
                    lang_and_setup.language = Language::TypeScript {
                        definition_file: false,
                    };
                    lang_and_setup.variant = LanguageVariant::Jsx;
                }
                "jsx" => {
                    lang_and_setup.language = Language::JavaScript;
                    lang_and_setup.variant = LanguageVariant::Jsx;
                }
                "js" => {
                    lang_and_setup.language = Language::JavaScript;
                    lang_and_setup.variant = LanguageVariant::Standard;
                }
                _ => {}
            }
        }
    }
    lang_and_setup
}

/// Type meant to register all the syntax rules for each language supported by Biome
///
/// When a new language is introduced, it must be implemented it. Syntax rules aren't negotiable via configuration, so it's safe
/// to pull all of them.
#[derive(Default, Debug)]
struct SyntaxVisitor<'a> {
    pub(crate) enabled_rules: Vec<RuleFilter<'a>>,
}

#[cfg(feature = "lang_js")]
impl RegistryVisitor<JsLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

impl RegistryVisitor<JsonLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

#[cfg(feature = "lang_css")]
impl RegistryVisitor<CssLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

#[cfg(feature = "lang_graphql")]
impl RegistryVisitor<GraphqlLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

#[cfg(feature = "lang_html")]
impl RegistryVisitor<HtmlLanguage> for SyntaxVisitor<'_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Syntax {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.enabled_rules.push(RuleFilter::Rule(
            <R::Group as RuleGroup>::NAME,
            R::METADATA.name,
        ))
    }
}

/// Type meant to register all the lint rules for each language supported by Biome
///
#[derive(Debug)]
struct LintVisitor<'a, 'b> {
    pub(crate) enabled_rules: FxHashSet<RuleFilter<'a>>,
    pub(crate) disabled_rules: FxHashSet<RuleFilter<'a>>,
    /// Set of rules that have a code fix, regardless of whether they are enabled.
    /// Used after `finish()` to derive the fixable subset of `enabled_rules`.
    pub(crate) rules_with_fix: FxHashSet<RuleFilter<'a>>,
    rules: Option<&'b Rules>,
    domains: Option<&'b RuleDomains>,
    globals: &'b mut Vec<Box<str>>,
}

impl<'a, 'b> LintVisitor<'a, 'b> {
    pub(crate) fn new(
        rules: Option<&'b Rules>,
        domains: Option<&'b RuleDomains>,
        globals: &'b mut Vec<Box<str>>,
    ) -> Self {
        Self {
            enabled_rules: Default::default(),
            disabled_rules: Default::default(),
            rules_with_fix: Default::default(),
            rules,
            domains,
            globals,
        }
    }

    /// Applies configured domains to the rule.
    ///
    /// Global recommended presets exclude rules with domains. A matching domain
    /// set to `all` enables the rule, `recommended` enables it only when the rule
    /// is recommended, and `none` disables it.
    fn record_rule_from_domains<R, L>(&mut self, rule_filter: RuleFilter<'static>)
    where
        L: biome_rowan::Language,
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let group = <R::Group as RuleGroup>::NAME;
        // Nursery rules must be enabled only when they are enabled from the group
        if group == "nursery" {
            return;
        }
        // no domains, no need to record the rule
        if self.domains.is_none_or(|domains| domains.is_empty()) {
            return;
        }

        for rule_domain in R::METADATA.domains {
            if let Some((configured_domain, configured_domain_value)) = self
                .domains
                .and_then(|domains| domains.get_key_value(rule_domain))
            {
                match configured_domain_value {
                    RuleDomainValue::All => {
                        self.enabled_rules.insert(rule_filter);
                        self.globals
                            .extend(configured_domain.globals().iter().copied().map(Into::into));
                    }
                    RuleDomainValue::None => {
                        self.disabled_rules.insert(rule_filter);
                    }
                    RuleDomainValue::Recommended => {
                        if R::METADATA.recommended {
                            self.enabled_rules.insert(rule_filter);
                            self.globals.extend(
                                configured_domain.globals().iter().copied().map(Into::into),
                            );
                        }
                    }
                }
            }
        }
    }

    fn finish(
        mut self,
    ) -> (
        FxHashSet<RuleFilter<'a>>,
        FxHashSet<RuleFilter<'a>>,
        FxHashSet<RuleFilter<'a>>,
    ) {
        let rules = self.rules.cloned().unwrap_or_default();
        self.enabled_rules.extend(rules.as_enabled_rules());
        self.disabled_rules.extend(rules.as_disabled_rules());
        (self.enabled_rules, self.disabled_rules, self.rules_with_fix)
    }

    fn push_rule<R, L>(&mut self, rule_filter: Option<RuleFilter<'static>>)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
        L: biome_rowan::Language,
    {
        let Some(rule_filter) = rule_filter.filter(|rule_filter| rule_filter.match_rule::<R>())
        else {
            return;
        };

        self.record_rule_from_domains::<R, L>(rule_filter);

        if R::METADATA.fix_kind != FixKind::None {
            self.rules_with_fix.insert(rule_filter);
        }
    }
}

#[cfg(feature = "lang_js")]
impl RegistryVisitor<JsLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            js_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}
impl RegistryVisitor<JsonLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsonLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            json_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_css")]
impl RegistryVisitor<CssLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = CssLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            css_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_graphql")]
impl RegistryVisitor<GraphqlLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = GraphqlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            graphql_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_html")]
impl RegistryVisitor<HtmlLanguage> for LintVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = HtmlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            biome_html_analyze::METADATA
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ManifestVisitorResult {
    enabled_rules: Vec<RuleFilter<'static>>,
    fixable_rules: Vec<RuleFilter<'static>>,
    globals: Vec<Box<str>>,
}

/// A package manifest whose identity includes only dependency matching data.
#[derive(Clone, Debug)]
struct ManifestDependencies(PackageJson);

impl ManifestDependencies {
    fn matches_dependency(&self, specifier: &str, range: &str) -> bool {
        self.0.matches_dependency(specifier, range)
    }
}

impl PartialEq for ManifestDependencies {
    fn eq(&self, other: &Self) -> bool {
        dependencies_equal(&self.0.dependencies, &other.0.dependencies)
            && dependencies_equal(&self.0.dev_dependencies, &other.0.dev_dependencies)
            && dependencies_equal(&self.0.peer_dependencies, &other.0.peer_dependencies)
            && catalogs_equal(self.0.catalog.as_ref(), other.0.catalog.as_ref())
    }
}

impl Eq for ManifestDependencies {}

impl Hash for ManifestDependencies {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.dependencies.0.hash(state);
        self.0.dev_dependencies.0.hash(state);
        self.0.peer_dependencies.0.hash(state);

        let Some(catalog) = &self.0.catalog else {
            false.hash(state);
            return;
        };
        true.hash(state);
        catalog.default.as_ref().map(|items| &items.0).hash(state);

        let mut named = catalog.named.iter().collect::<Vec<_>>();
        named.sort_unstable_by_key(|(name, _)| *name);
        for (name, dependencies) in named {
            name.hash(state);
            dependencies.0.hash(state);
        }
    }
}

fn dependencies_equal(left: &Dependencies, right: &Dependencies) -> bool {
    left.0 == right.0
}

fn catalogs_equal(
    left: Option<&biome_package::Catalogs>,
    right: Option<&biome_package::Catalogs>,
) -> bool {
    match (left, right) {
        (None, None) => true,
        (Some(left), Some(right)) => {
            match (&left.default, &right.default) {
                (None, None) => {}
                (Some(left), Some(right)) if dependencies_equal(left, right) => {}
                _ => return false,
            }
            left.named.len() == right.named.len()
                && left.named.iter().all(|(name, dependencies)| {
                    right
                        .named
                        .get(name.as_ref())
                        .is_some_and(|other| dependencies_equal(dependencies, other))
                })
        }
        _ => false,
    }
}

struct ManifestVisitor<'a> {
    domains: Option<&'a RuleDomains>,
    manifest: &'a ManifestDependencies,
    recommended_enabled: bool,
    enabled_rules: FxHashSet<RuleFilter<'static>>,
    fixable_rules: FxHashSet<RuleFilter<'static>>,
    globals: Vec<Box<str>>,
}

impl<'a> ManifestVisitor<'a> {
    fn new(
        domains: Option<&'a RuleDomains>,
        manifest: &'a ManifestDependencies,
        recommended_enabled: bool,
    ) -> Self {
        Self {
            domains,
            manifest,
            recommended_enabled,
            enabled_rules: FxHashSet::default(),
            fixable_rules: FxHashSet::default(),
            globals: Vec::new(),
        }
    }

    fn push_rule<R, L>(&mut self, rule_filter: Option<RuleFilter<'static>>)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
        L: biome_rowan::Language,
    {
        let Some(rule_filter) = rule_filter.filter(|rule_filter| rule_filter.match_rule::<R>())
        else {
            return;
        };
        if <R::Group as RuleGroup>::NAME == "nursery"
            || !R::METADATA.recommended
            || !self.recommended_enabled
        {
            return;
        }

        for domain in R::METADATA.domains {
            if self
                .domains
                .is_some_and(|domains| domains.contains_key(domain))
            {
                continue;
            }
            if domain
                .manifest_dependencies()
                .iter()
                .any(|(dependency, range)| self.manifest.matches_dependency(dependency, range))
            {
                self.enabled_rules.insert(rule_filter);
                if R::METADATA.fix_kind != FixKind::None {
                    self.fixable_rules.insert(rule_filter);
                }
                self.globals
                    .extend(domain.globals().iter().copied().map(Into::into));
            }
        }
    }

    fn finish(self) -> ManifestVisitorResult {
        ManifestVisitorResult {
            enabled_rules: self.enabled_rules.into_iter().collect(),
            fixable_rules: self.fixable_rules.into_iter().collect(),
            globals: self.globals,
        }
    }
}

#[cfg(feature = "lang_js")]
impl RegistryVisitor<JsLanguage> for ManifestVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            js_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

impl RegistryVisitor<JsonLanguage> for ManifestVisitor<'_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = JsonLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            json_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_css")]
impl RegistryVisitor<CssLanguage> for ManifestVisitor<'_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = CssLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            css_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_graphql")]
impl RegistryVisitor<GraphqlLanguage> for ManifestVisitor<'_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = GraphqlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            graphql_metadata
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

#[cfg(feature = "lang_html")]
impl RegistryVisitor<HtmlLanguage> for ManifestVisitor<'_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Lint {
            C::record_groups(self)
        }
    }

    fn record_group<G: RuleGroup<Language = HtmlLanguage>>(&mut self) {
        G::record_rules(self)
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>(
            biome_html_analyze::METADATA
                .find_rule(R::Group::NAME, R::METADATA.name)
                .map(RuleFilter::from),
        )
    }
}

struct AssistsVisitor<'a, 'b> {
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
    /// Set of rules that have a code fix, regardless of whether they are enabled.
    /// Used after `finish()` to derive the fixable subset of `enabled_rules`.
    rules_with_fix: FxHashSet<RuleFilter<'a>>,
    actions: Option<&'b Actions>,
}

impl<'a, 'b> AssistsVisitor<'a, 'b> {
    pub(crate) fn new(actions: Option<&'b Actions>) -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            rules_with_fix: Default::default(),
            actions,
        }
    }

    pub(crate) fn push_rule<R, L>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        // We deem refactors **safe**, other assists aren't safe
        if R::Group::NAME != "source" {
            return;
        }

        if R::METADATA.fix_kind != FixKind::None {
            self.rules_with_fix.insert(RuleFilter::Rule(
                <R::Group as RuleGroup>::NAME,
                R::METADATA.name,
            ));
        }
    }

    fn finish(
        mut self,
    ) -> (
        Vec<RuleFilter<'a>>,
        Vec<RuleFilter<'a>>,
        FxHashSet<RuleFilter<'a>>,
    ) {
        let actions = self.actions.map(Cow::Borrowed).unwrap_or_default();
        self.enabled_rules.extend(actions.as_enabled_rules());
        self.disabled_rules.extend(actions.as_disabled_rules());
        (self.enabled_rules, self.disabled_rules, self.rules_with_fix)
    }
}

#[cfg(feature = "lang_js")]
impl RegistryVisitor<JsLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsLanguage, Output: Clone>> + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

impl RegistryVisitor<JsonLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = JsonLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = JsonLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

#[cfg(feature = "lang_css")]
impl RegistryVisitor<CssLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = CssLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = CssLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

#[cfg(feature = "lang_graphql")]
impl RegistryVisitor<GraphqlLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = GraphqlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = GraphqlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

#[cfg(feature = "lang_html")]
impl RegistryVisitor<HtmlLanguage> for AssistsVisitor<'_, '_> {
    fn record_category<C: GroupCategory<Language = HtmlLanguage>>(&mut self) {
        if C::CATEGORY == RuleCategory::Action {
            C::record_groups(self)
        }
    }

    fn record_rule<R>(&mut self)
    where
        R: Rule<Options: Default, Query: Queryable<Language = HtmlLanguage, Output: Clone>>
            + 'static,
    {
        self.push_rule::<R, <R::Query as Queryable>::Language>();
    }
}

#[derive(Clone, Debug, PartialEq)]
struct AnalyzerVisitorComputedResult {
    syntax_rules: Vec<RuleFilter<'static>>,
    configured_enabled_rules: Vec<RuleFilter<'static>>,
    configured_disabled_rules: Vec<RuleFilter<'static>>,
    rules_with_fix: FxHashSet<RuleFilter<'static>>,
    globals: Vec<Box<str>>,
}

#[salsa::interned]
struct AnalyzerInput {
    #[returns(ref)]
    settings: SettingsIdentity,
    #[returns(ref)]
    override_indices: Box<[usize]>,
}

#[salsa::interned]
struct ManifestAnalyzerInput {
    #[returns(ref)]
    settings: SettingsIdentity,
    #[returns(ref)]
    override_indices: Box<[usize]>,
    #[returns(ref)]
    manifest: ManifestDependencies,
}

#[salsa::tracked(returns(ref))]
fn resolved_analyzer_visitor<'db>(
    db: &'db dyn salsa::Database,
    input: AnalyzerInput<'db>,
) -> AnalyzerVisitorComputedResult {
    compute_analyzer_visitor(input.settings(db).as_ref(), input.override_indices(db))
}

#[cfg(test)]
pub(crate) fn analyzer_input_count_for_test(db: &WorkspaceDb) -> usize {
    let query_db = db.settings_query_db();
    AnalyzerInput::ingredient(query_db.zalsa())
        .entries(query_db.zalsa())
        .count()
}

#[salsa::tracked(returns(ref))]
fn resolved_manifest_visitor<'db>(
    db: &'db dyn salsa::Database,
    input: ManifestAnalyzerInput<'db>,
) -> ManifestVisitorResult {
    compute_manifest_visitor(
        input.settings(db).as_ref(),
        input.override_indices(db),
        input.manifest(db),
    )
}

#[cfg(test)]
pub(crate) fn resolved_manifest_visitor_for_test(
    db: &WorkspaceDb,
    settings: SettingsIdentity,
    override_indices: Box<[usize]>,
    manifest: PackageJson,
) {
    let query_db = db.settings_query_db();
    let input = ManifestAnalyzerInput::new(
        &query_db,
        settings,
        override_indices,
        ManifestDependencies(manifest),
    );
    resolved_manifest_visitor(&query_db, input);
}

fn compute_analyzer_visitor(
    settings: &Settings,
    override_indices: &[usize],
) -> AnalyzerVisitorComputedResult {
    let mut syntax = SyntaxVisitor::default();

    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut syntax);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut syntax);
    biome_json_analyze::visit_registry(&mut syntax);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut syntax);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut syntax);

    let linter_rules = settings.as_linter_rules_by_indices(override_indices);
    let linter_domains = settings.as_linter_domains_by_indices(override_indices);
    let assist_actions = settings.as_assist_actions_by_indices(override_indices);

    let mut globals = Vec::new();
    let mut lint = LintVisitor::new(
        linter_rules.as_deref(),
        linter_domains.as_deref(),
        &mut globals,
    );

    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut lint);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut lint);
    biome_json_analyze::visit_registry(&mut lint);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut lint);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut lint);
    let (linter_enabled_rules, linter_disabled_rules, mut rules_with_fix) = lint.finish();

    let mut assist = AssistsVisitor::new(assist_actions.as_deref());

    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut assist);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut assist);
    biome_json_analyze::visit_registry(&mut assist);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut assist);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut assist);
    let (assists_enabled_rules, assists_disabled_rules, assists_rules_with_fix) = assist.finish();
    rules_with_fix.extend(assists_rules_with_fix);

    AnalyzerVisitorComputedResult {
        syntax_rules: syntax.enabled_rules,
        configured_enabled_rules: linter_enabled_rules
            .into_iter()
            .chain(assists_enabled_rules)
            .collect(),
        configured_disabled_rules: linter_disabled_rules
            .into_iter()
            .chain(assists_disabled_rules)
            .collect(),
        rules_with_fix,
        globals,
    }
}

fn compute_manifest_visitor(
    settings: &Settings,
    override_indices: &[usize],
    manifest: &ManifestDependencies,
) -> ManifestVisitorResult {
    let domains = settings.as_linter_domains_by_indices(override_indices);
    let mut visitor = ManifestVisitor::new(
        domains.as_deref(),
        manifest,
        settings.linter_recommended_enabled(),
    );

    #[cfg(feature = "lang_js")]
    biome_js_analyze::visit_registry(&mut visitor);
    #[cfg(feature = "lang_css")]
    biome_css_analyze::visit_registry(&mut visitor);
    biome_json_analyze::visit_registry(&mut visitor);
    #[cfg(feature = "lang_graphql")]
    biome_graphql_analyze::visit_registry(&mut visitor);
    #[cfg(feature = "lang_html")]
    biome_html_analyze::visit_registry(&mut visitor);

    visitor.finish()
}

fn extend_rule_filters(
    filters: &mut Vec<RuleFilter<'static>>,
    selectors: Option<&[AnalyzerSelector]>,
) {
    let Some(selectors) = selectors else {
        return;
    };
    for selector in selectors {
        match selector {
            AnalyzerSelector::Rule(rule) => filters.push(RuleFilter::from(rule)),
            AnalyzerSelector::Domain(domain) => filters.extend(domain.as_rule_filters()),
            AnalyzerSelector::Plugin => {}
        }
    }
}

/// Result of building the analyzer visitor: the resolved rule filters and options.
pub(crate) struct AnalyzerVisitorResult {
    pub(crate) enabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) disabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) analyzer_options: AnalyzerOptions,
    /// Subset of `enabled_rules` that have a code fix (`FixKind::Safe` or `FixKind::Unsafe`).
    pub(crate) fixable_rules: Vec<RuleFilter<'static>>,
}

pub(crate) struct AnalyzerVisitorBuilder<'a, 'settings> {
    settings: &'a SettingsWithEditor<'settings>,
    db: &'a WorkspaceDb,
    only: Option<&'a [AnalyzerSelector]>,
    skip: Option<&'a [AnalyzerSelector]>,
    path: Option<&'a Utf8Path>,
    enabled_selectors: Option<&'a [AnalyzerSelector]>,
    project_layout: Arc<ProjectLayout>,
    analyzer_options: AnalyzerOptions,
}

impl<'a, 'settings> AnalyzerVisitorBuilder<'a, 'settings> {
    pub(crate) fn new(
        settings: &'a SettingsWithEditor<'settings>,
        db: &'a WorkspaceDb,
        analyzer_options: AnalyzerOptions,
    ) -> Self {
        Self {
            settings,
            db,
            only: None,
            skip: None,
            path: None,
            enabled_selectors: None,
            project_layout: Default::default(),
            analyzer_options,
        }
    }

    #[must_use]
    pub(crate) fn with_only(mut self, only: &'a [AnalyzerSelector]) -> Self {
        self.only = Some(only);
        self
    }

    #[must_use]
    pub(crate) fn with_skip(mut self, skip: &'a [AnalyzerSelector]) -> Self {
        self.skip = Some(skip);
        self
    }

    #[must_use]
    pub(crate) fn with_path(mut self, path: &'a Utf8Path) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub(crate) fn with_enabled_selectors(mut self, enabled_rules: &'a [AnalyzerSelector]) -> Self {
        self.enabled_selectors = Some(enabled_rules);
        self
    }

    #[must_use]
    pub(crate) fn with_project_layout(mut self, project_layout: Arc<ProjectLayout>) -> Self {
        self.project_layout = project_layout;
        self
    }

    #[must_use]
    pub(crate) fn finish(self) -> AnalyzerVisitorResult {
        let query = self.settings.query();
        let (selected_settings, query_db, computed) =
            if let Some(inline_settings) = query.inline_settings() {
                let selected_settings = inline_settings.clone();
                let computed =
                    compute_analyzer_visitor(selected_settings.as_ref(), query.override_indices());
                (selected_settings, None, computed)
            } else {
                let selected_settings = query
                    .selection()
                    .selected_settings(self.db, query.project());
                let query_db = self.db.settings_query_db();
                let input = AnalyzerInput::new(
                    &query_db,
                    selected_settings.clone(),
                    query.override_indices(),
                );
                let computed = resolved_analyzer_visitor(&query_db, input).clone();
                (selected_settings, Some(query_db), computed)
            };

        let AnalyzerVisitorComputedResult {
            syntax_rules,
            configured_enabled_rules,
            configured_disabled_rules,
            rules_with_fix,
            globals: configured_globals,
        } = computed;

        let mut enabled_rules = Vec::new();
        extend_rule_filters(&mut enabled_rules, self.enabled_selectors);
        enabled_rules.extend(syntax_rules);
        extend_rule_filters(&mut enabled_rules, self.only);

        let mut disabled_rules = Vec::new();
        extend_rule_filters(&mut disabled_rules, self.skip);

        let use_configured_rules = self.only.is_some_and(|only| only.is_empty());
        let mut globals = Vec::new();
        if use_configured_rules {
            enabled_rules.extend(configured_enabled_rules);
            disabled_rules.extend(configured_disabled_rules);
            globals = configured_globals;
        }

        let only_excludes_plugins = self
            .only
            .is_some_and(|only| !only.is_empty() && !only.contains(&AnalyzerSelector::Plugin));
        let plugins_skipped = self
            .skip
            .is_some_and(|skip| skip.contains(&AnalyzerSelector::Plugin));
        if only_excludes_plugins || plugins_skipped {
            disabled_rules.push(RuleFilter::Group(PLUGIN_GROUP));
        }

        if use_configured_rules
            && let Some((_, manifest)) = self
                .path
                .and_then(|path| self.project_layout.find_node_manifest_for_path(path))
        {
            let manifest = ManifestDependencies(manifest);
            let manifest_result = if let Some(query_db) = &query_db {
                let input = ManifestAnalyzerInput::new(
                    query_db,
                    selected_settings.clone(),
                    query.override_indices(),
                    manifest,
                );
                resolved_manifest_visitor(query_db, input).clone()
            } else {
                compute_manifest_visitor(
                    selected_settings.as_ref(),
                    query.override_indices(),
                    &manifest,
                )
            };
            enabled_rules.extend(manifest_result.enabled_rules);
            globals.extend(manifest_result.globals);
        }

        let fixable_rules = enabled_rules
            .iter()
            .filter(|rule| rules_with_fix.contains(rule))
            .copied()
            .collect();

        let mut analyzer_options = self.analyzer_options;
        analyzer_options.push_globals(globals);

        if let Some(path) = self.path
            && analyzer_options.jsx_runtime() == Some(JsxRuntime::ReactClassic)
        {
            if analyzer_options.jsx_factory().is_none() {
                let factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig.jsx_factory_identifier().map(|s| s.to_string())
                    })
                    .flatten();
                analyzer_options.set_jsx_factory(factory.map(Into::into));
            }
            if analyzer_options.jsx_fragment_factory().is_none() {
                let fragment_factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig
                            .jsx_fragment_factory_identifier()
                            .map(|s| s.to_string())
                    })
                    .flatten();
                analyzer_options.set_jsx_fragment_factory(fragment_factory.map(Into::into));
            }
        }

        AnalyzerVisitorResult {
            enabled_rules,
            disabled_rules,
            analyzer_options,
            fixable_rules,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DocumentFileSource, Features};
    #[cfg(feature = "lang_js")]
    use super::{ManifestDependencies, compute_analyzer_visitor, compute_manifest_visitor};
    #[cfg(feature = "lang_js")]
    use crate::settings::Settings;
    #[cfg(feature = "lang_js")]
    use biome_analyze::RuleFilter;
    #[cfg(feature = "lang_js")]
    use biome_package::{Dependencies, PackageJson};
    use camino::Utf8Path;

    #[cfg(feature = "lang_js")]
    #[test]
    fn manifest_dependencies_enable_recommended_domain_rules() {
        let manifest = PackageJson {
            dev_dependencies: Dependencies(
                vec![("vitest".into(), "^1.0.0".into())].into_boxed_slice(),
            ),
            ..PackageJson::default()
        };
        let result =
            compute_manifest_visitor(&Settings::default(), &[], &ManifestDependencies(manifest));
        let no_focused_tests = RuleFilter::Rule("suspicious", "noFocusedTests");

        assert!(result.enabled_rules.contains(&no_focused_tests));
        assert!(result.fixable_rules.contains(&no_focused_tests));
        assert!(
            result
                .globals
                .iter()
                .any(|global| global.as_ref() == "test")
        );
    }

    #[cfg(feature = "lang_js")]
    #[test]
    fn default_assist_actions_enable_organize_imports() {
        let result = compute_analyzer_visitor(&Settings::default(), &[]);
        let organize_imports = RuleFilter::Rule("source", "organizeImports");

        assert!(result.configured_enabled_rules.contains(&organize_imports));
        assert!(result.rules_with_fix.contains(&organize_imports));
    }

    #[test]
    fn svelte_source_modules_use_js_capabilities() {
        let features = Features::new();
        let path = Utf8Path::new("file.svelte.js");
        let capabilities =
            features.get_deprecated_capabilities(DocumentFileSource::from_path(path, false));

        assert!(capabilities.analyzer.rename.is_some());
        assert!(capabilities.analyzer.pull_diagnostics_and_actions.is_some());
    }

    #[test]
    fn svelte_typescript_source_modules_use_js_capabilities() {
        let features = Features::new();
        let path = Utf8Path::new("file.svelte.ts");
        let capabilities =
            features.get_deprecated_capabilities(DocumentFileSource::from_path(path, false));

        assert!(capabilities.analyzer.rename.is_some());
        assert!(capabilities.analyzer.pull_diagnostics_and_actions.is_some());
    }

    #[test]
    fn svelte_component_files_keep_svelte_capabilities() {
        let features = Features::new();
        let path = Utf8Path::new("file.svelte");
        let capabilities =
            features.get_deprecated_capabilities(DocumentFileSource::from_path(path, false));

        assert!(capabilities.analyzer.rename.is_none());
        assert!(capabilities.analyzer.pull_diagnostics_and_actions.is_none());
    }
}
