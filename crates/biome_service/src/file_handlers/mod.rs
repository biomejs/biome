pub mod astro;
pub(crate) mod css;
#[cfg(feature = "lang_graphql")]
pub(crate) mod graphql;
#[cfg(feature = "lang_grit")]
pub(crate) mod grit;
pub(crate) mod html;
mod ignore;
pub(crate) mod javascript;
pub(crate) mod json;
#[cfg(feature = "lang_md")]
pub(crate) mod md;
pub mod svelte;
mod unknown;
pub mod vue;
#[cfg(feature = "lang_yaml")]
pub(crate) mod yaml;

use self::{
    css::CssFileHandler, javascript::JsFileHandler, json::JsonFileHandler,
    unknown::UnknownFileHandler,
};
use crate::WorkspaceError;
pub use crate::file_handlers::astro::AstroFileHandler;
#[cfg(feature = "lang_graphql")]
use crate::file_handlers::graphql::GraphqlFileHandler;
use crate::file_handlers::ignore::IgnoreFileHandler;
pub use crate::file_handlers::svelte::SvelteFileHandler;
pub use crate::file_handlers::vue::VueFileHandler;
use crate::settings::{Settings, SettingsWithEditor};
use crate::utils::growth_guard::GrowthGuard;
use crate::workspace::document::services::embedded_bindings::EmbeddedBuilder;
use crate::workspace::{
    AnyEmbeddedSnippet, CodeAction, DefinitionReference, DocumentServices, FixAction, FixFileMode,
    FixFileResult, GetSyntaxTreeResult, GoToDefinitionResult, PatternId, PullActionsResult,
    PullDiagnosticsAndActionsResult, RenameResult, SearchQuery,
};
use biome_analyze::options::JsxRuntime;
use biome_analyze::{
    ActionFilter, AnalyzerAction, AnalyzerDiagnostic, AnalyzerOptions, AnalyzerPluginVec,
    AnalyzerSignal, ControlFlow, FixKind, GroupCategory, Never, PLUGIN_GROUP, Queryable,
    RegistryVisitor, Rule, RuleCategories, RuleCategory, RuleError, RuleFilter, RuleGroup,
};
use biome_configuration::Rules;
use biome_configuration::analyzer::{AnalyzerSelector, RuleDomainValue};
use biome_css_analyze::METADATA as css_metadata;
use biome_css_syntax::CssLanguage;
use biome_diagnostics::{Applicability, Diagnostic, DiagnosticExt, Error, Severity, category};
use biome_formatter::{FormatContext, FormatResult, Formatted, Printed, SourceMapGeneration};
use biome_fs::BiomePath;
#[cfg(feature = "lang_graphql")]
use biome_graphql_analyze::METADATA as graphql_metadata;
#[cfg(feature = "lang_graphql")]
use biome_graphql_syntax::GraphqlLanguage;
use biome_html_syntax::HtmlLanguage;
use biome_js_analyze::METADATA as js_metadata;
use biome_js_parser::{JsParserOptions, parse};
use biome_js_syntax::{
    AnyJsModuleItem, JsLanguage, JsxAttribute, JsxAttributeList, TextRange, TextSize,
};
use biome_json_analyze::METADATA as json_metadata;
use biome_json_syntax::JsonLanguage;
use biome_languages::DocumentFileSource;
use biome_languages::javascript::{
    JsEmbeddingKind, JsFileSource, Language, LanguageVariant, SvelteFileKind,
};
use biome_module_graph::{ModuleDb, ProjectDatabase};
use biome_package::PackageJson;
use biome_parser::AnyParse;
use biome_project_layout::ProjectLayout;
use biome_rowan::{BatchMutation, NodeCache, SendNode, SyntaxNode, TokenText};
use biome_text_edit::TextEdit;
use camino::Utf8Path;
use either::Either;
use html::HtmlFileHandler;
pub use javascript::JsFormatterSettings;
use papaya::HashMap;
use rustc_hash::{FxHashSet, FxHasher};
use std::borrow::Cow;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tracing::trace;

pub struct FixAllParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) fix_file_mode: FixFileMode,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    /// Whether it should format the code action
    pub(crate) should_format: bool,
    pub(crate) biome_path: &'a BiomePath,
    pub(crate) module_db: ProjectDatabase,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) document_file_source: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) rule_categories: RuleCategories,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) document_services: &'a DocumentServices,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    /// The initial indentation level to apply when printing formatted code.
    /// Used by embedded language handlers (Svelte, Vue) to preserve
    /// `indentScriptAndStyle` indentation during fix-all operations.
    pub(crate) embeds_initial_indent: u16,
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
    pub(crate) nodes: Vec<(AnyEmbeddedSnippet, DocumentFileSource)>,
}

type Parse =
    fn(&BiomePath, DocumentFileSource, &str, &SettingsWithEditor, &mut NodeCache) -> ParseResult;
type ParseEmbeddedNodes = fn(
    &AnyParse,
    &BiomePath,
    &DocumentFileSource,
    &SettingsWithEditor,
    &mut NodeCache,
    &mut EmbeddedBuilder,
) -> ParseEmbedResult;
#[derive(Default)]
pub struct ParserCapabilities {
    /// Parse a file
    pub(crate) parse: Option<Parse>,

    pub(crate) parse_embedded_nodes: Option<ParseEmbeddedNodes>,
}

type DebugSyntaxTree = fn(&BiomePath, AnyParse) -> GetSyntaxTreeResult;
type DebugControlFlow = fn(AnyParse, TextSize) -> String;
type DebugFormatterIR = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
) -> Result<String, WorkspaceError>;
type DebugTypeInfo =
    fn(&BiomePath, Option<AnyParse>, ProjectDatabase) -> Result<String, WorkspaceError>;
type DebugRegisteredTypes = fn(&BiomePath, AnyParse) -> Result<String, WorkspaceError>;
type DebugSemanticModel = fn(&BiomePath, AnyParse) -> Result<String, WorkspaceError>;

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
    pub(crate) parse: AnyParse,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) module_db: ProjectDatabase,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) pull_code_actions: bool,
    pub(crate) diagnostic_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
    pub(crate) snippet_services: Option<&'a DocumentServices>,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    pub(crate) max_diagnostics: Option<u32>,
    pub(crate) diagnostic_level: Severity,
    /// When true, promote assist diagnostics (`assist/*`) to error severity.
    pub(crate) enforce_assist: bool,
    /// Cached rules for the current analyzer pass.
    pub(crate) analyzer_cache: &'a AnalyzerVisitorCache,
}

pub(crate) struct DiagnosticsAndActionsParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) language: DocumentFileSource,
    pub(crate) path: &'a BiomePath,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) categories: RuleCategories,
    pub(crate) module_db: ProjectDatabase,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_selectors: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) diagnostic_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    // Services attached to the current embedded snippet, when diagnostics are run on snippets.
    pub(crate) snippet_services: Option<&'a DocumentServices>,
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
    pub(crate) fn new(params: &LintParams<'a>) -> Self {
        Self {
            diagnostic_count: params.parse.diagnostics().len() as u32,
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
            rules: params
                .settings
                .as_ref()
                .as_linter_rules(params.path.as_path()),
            pull_code_actions: params.pull_code_actions,
            diagnostic_offset: params.diagnostic_offset,
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
    pub(crate) fn new(
        params: &'a FixAllParams,
        rules: Option<Cow<'a, Rules>>,
        syntax_len: u32,
    ) -> Self {
        Self {
            fix_file_mode: &params.fix_file_mode,
            errors: 0,
            rules,
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
        text_edit: Option<(TextRange, TextEdit)>,
        current_text: &str,
    ) -> Result<Option<String>, WorkspaceError> {
        let Some((range, edit)) = text_edit else {
            return Ok(None);
        };
        let new_text = edit.new_string(current_text);
        if new_text == current_text {
            return Ok(None);
        }
        self.record_text_edit_fix(range, new_text.len() as u32, Some(("plugin", "gritql")))?;
        Ok(Some(new_text))
    }

    /// Finish processing the fix all actions. Returns the result of the fix-all actions. The `format_tree`
    /// is a closure that must return the new code (formatted, if needed).
    ///
    /// `initial_indent` specifies the base indentation level for printing. This is used by
    /// embedded language handlers (e.g. Svelte, Vue) to preserve `indentScriptAndStyle`
    /// indentation when formatting during fix-all operations.
    pub(crate) fn finish<F, C>(
        self,
        format_tree: F,
        initial_indent: u16,
    ) -> Result<FixFileResult, WorkspaceError>
    where
        F: FnOnce() -> Result<Either<FormatResult<Formatted<C>>, String>, WorkspaceError>,
        C: FormatContext,
    {
        let code = match format_tree()? {
            Either::Left(printed) => {
                if initial_indent > 0 {
                    printed?
                        .print_with_indent(initial_indent, SourceMapGeneration::Disabled)?
                        .into_code()
                } else {
                    printed?.print()?.into_code()
                }
            }
            Either::Right(string) => string,
        };
        Ok(FixFileResult {
            code,
            skipped_suggested_fixes: self.skipped_suggested_fixes,
            actions: self.actions,
            errors: self.errors,
        })
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
    pub(crate) parse: AnyParse,
    pub(crate) range: Option<TextRange>,
    pub(crate) settings: &'a SettingsWithEditor<'a>,
    pub(crate) path: &'a BiomePath,
    pub(crate) module_db: ProjectDatabase,
    pub(crate) project_layout: Arc<ProjectLayout>,
    pub(crate) language: DocumentFileSource,
    pub(crate) only: &'a [AnalyzerSelector],
    pub(crate) skip: &'a [AnalyzerSelector],
    pub(crate) suppression_reason: Option<String>,
    pub(crate) enabled_rules: &'a [AnalyzerSelector],
    pub(crate) plugins: AnalyzerPluginVec,
    pub(crate) categories: RuleCategories,
    pub(crate) action_offset: Option<TextSize>,
    pub(crate) document_services: &'a DocumentServices,
    pub(crate) working_directory: Option<&'a Utf8Path>,
    /// When `false`, actions are returned with `suggestion: None` (no mutations computed).
    pub(crate) compute_actions: bool,
    // Services attached to the current embedded snippet, when actions are run on snippets.
    pub(crate) snippet_services: Option<&'a DocumentServices>,
    pub(crate) analyzer_cache: &'a AnalyzerVisitorCache,
}

pub(crate) struct UpdateSnippetsNodes {
    pub(crate) range: TextRange,
    pub(crate) new_code: String,
    /// When `true`, `new_code` needs to be re-indented to match the
    /// host's nesting level. When `false`, `new_code` already has the
    /// right shape and can be spliced back as-is.
    pub(crate) needs_reindent: bool,
}

type Lint = fn(LintParams) -> LintResults;
type CodeActions = fn(CodeActionsParams) -> PullActionsResult;
type FixAll = fn(FixAllParams) -> Result<FixFileResult, WorkspaceError>;
type Rename = fn(&BiomePath, AnyParse, TextSize, String) -> Result<RenameResult, WorkspaceError>;
type UpdateSnippets = fn(AnyParse, Vec<UpdateSnippetsNodes>) -> Result<SendNode, WorkspaceError>;
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
    AnyParse,
    &SettingsWithEditor,
) -> Result<Printed, WorkspaceError>;
type FormatRange = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    TextRange,
) -> Result<Printed, WorkspaceError>;
type FormatOnType = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    TextSize,
) -> Result<Printed, WorkspaceError>;

type FormatEmbedded = fn(
    &BiomePath,
    &DocumentFileSource,
    AnyParse,
    &SettingsWithEditor,
    Vec<FormatEmbedNode>,
) -> Result<Printed, WorkspaceError>;

#[derive(Debug)]
pub(crate) struct FormatEmbedNode {
    pub(crate) range: TextRange,
    pub(crate) node: AnyParse,
    pub(crate) source: DocumentFileSource,
}

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
    AnyParse,
    &dyn SearchQuery,
    &SettingsWithEditor,
    PatternId,
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

pub(crate) struct ResolveBindingParams<'a> {
    pub(crate) parse: AnyParse,
    pub(crate) cursor_offset: TextSize,
    pub(crate) services: &'a DocumentServices,
}

pub(crate) struct ResolveDefinitionParams<'a> {
    pub(crate) path: &'a BiomePath,
    pub(crate) definition_ref: &'a DefinitionReference,
    pub(crate) module_db: &'a dyn ModuleDb,
    pub(crate) offset: Option<TextSize>,
    pub(crate) services: &'a DocumentServices,
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
    js: JsFileHandler,
    json: JsonFileHandler,
    css: CssFileHandler,
    astro: AstroFileHandler,
    vue: VueFileHandler,
    svelte: SvelteFileHandler,
    unknown: UnknownFileHandler,
    #[cfg(feature = "lang_graphql")]
    graphql: GraphqlFileHandler,
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
            js: JsFileHandler {},
            json: JsonFileHandler {},
            css: CssFileHandler {},
            astro: AstroFileHandler {},
            svelte: SvelteFileHandler {},
            vue: VueFileHandler {},
            #[cfg(feature = "lang_graphql")]
            graphql: GraphqlFileHandler {},
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
            DocumentFileSource::Js(source) => match source.as_embedding_kind() {
                JsEmbeddingKind::Astro { .. } => self.astro.capabilities(),
                JsEmbeddingKind::Vue { .. } => self.vue.capabilities(),
                // `.svelte.ts` / `.svelte.js` are full JS/TS modules with Svelte
                // semantics; `.svelte` component documents still use the Svelte handler.
                JsEmbeddingKind::Svelte {
                    kind: SvelteFileKind::SourceModule,
                    ..
                } => self.js.capabilities(),
                JsEmbeddingKind::Svelte {
                    kind: SvelteFileKind::Component,
                    ..
                } => self.svelte.capabilities(),
                JsEmbeddingKind::None => self.js.capabilities(),
            },
            DocumentFileSource::Json(_) => self.json.capabilities(),
            DocumentFileSource::Css(_) => self.css.capabilities(),
            #[cfg(feature = "lang_graphql")]
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            DocumentFileSource::Html(_) => self.html.capabilities(),
            #[cfg(feature = "lang_grit")]
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            #[cfg(feature = "lang_md")]
            DocumentFileSource::Markdown(_) => self.markdown.capabilities(),
            #[cfg(feature = "lang_yaml")]
            DocumentFileSource::Yaml(_) => self.yaml.capabilities(),
            DocumentFileSource::Ignore => self.ignore.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
        }
    }

    /// Returns the [Capabilities] associated with a document source.
    pub(crate) fn get_real_capabilities(&self, language_hint: DocumentFileSource) -> Capabilities {
        match language_hint {
            DocumentFileSource::Js(_) => self.js.capabilities(),
            DocumentFileSource::Json(_) => self.json.capabilities(),
            DocumentFileSource::Css(_) => self.css.capabilities(),
            #[cfg(feature = "lang_graphql")]
            DocumentFileSource::Graphql(_) => self.graphql.capabilities(),
            DocumentFileSource::Html(_) => self.html.capabilities(),
            #[cfg(feature = "lang_grit")]
            DocumentFileSource::Grit(_) => self.grit.capabilities(),
            #[cfg(feature = "lang_md")]
            DocumentFileSource::Markdown(_) => self.markdown.capabilities(),
            #[cfg(feature = "lang_yaml")]
            DocumentFileSource::Yaml(_) => self.yaml.capabilities(),
            DocumentFileSource::Ignore => self.ignore.capabilities(),
            DocumentFileSource::Unknown => self.unknown.capabilities(),
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
pub struct ParsedLangAndSetup {
    language: Language,
    variant: LanguageVariant,
    setup: bool,
}

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
    // lint_params: &'b LintParams<'a>,
    only: Option<&'b [AnalyzerSelector]>,
    skip: Option<&'b [AnalyzerSelector]>,
    settings: &'b Settings,
    path: Option<&'b Utf8Path>,
    package_json: Option<PackageJson>,
    analyzer_options: &'b mut AnalyzerOptions,
}

impl<'a, 'b> LintVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [AnalyzerSelector]>,
        skip: Option<&'b [AnalyzerSelector]>,
        settings: &'b Settings,
        path: Option<&'b Utf8Path>,
        package_json: Option<PackageJson>,
        analyzer_options: &'b mut AnalyzerOptions,
    ) -> Self {
        Self {
            enabled_rules: Default::default(),
            disabled_rules: Default::default(),
            rules_with_fix: Default::default(),
            only,
            skip,
            settings,
            path,
            package_json,
            analyzer_options,
        }
    }

    /// It loops over the domains of the current rule, and check each domain specify
    /// a dependency.
    fn record_rule_from_manifest<R, L>(&mut self, rule_filter: RuleFilter<'static>)
    where
        L: biome_rowan::Language,
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let group = <R::Group as RuleGroup>::NAME;
        // Nursery rules must be enabled only when they are enabled from the group
        if group == "nursery" {
            return;
        }

        let path = self.path.expect("File path");

        let is_recommended = R::METADATA.recommended;
        let recommended_enabled = self.settings.linter_recommended_enabled();
        if !is_recommended || !recommended_enabled {
            return;
        }

        let no_only = self.only.is_some_and(|only| only.is_empty());
        if !no_only {
            return;
        }

        let domains = self.settings.as_linter_domains(path);
        if let Some(manifest) = &self.package_json {
            for domain in R::METADATA.domains {
                if domains
                    .as_ref()
                    .is_some_and(|domains| domains.contains_key(domain))
                {
                    continue;
                }

                let matches_a_dependency = domain
                    .manifest_dependencies()
                    .iter()
                    .any(|(dependency, range)| manifest.matches_dependency(dependency, range));

                if matches_a_dependency {
                    self.enabled_rules.insert(rule_filter);
                    self.analyzer_options
                        .push_globals(domain.globals().iter().copied().map(Into::into));
                }
            }
        }
    }

    /// It inspects the [RuleDomain] of the configuration, and if the current rule belongs to at least a configured domain, it's enabled.
    ///
    /// As per business logic, rules that have domains can be recommended, however they shouldn't be enabled when `linter.rules.recommended` is `true`.
    ///
    /// This means that
    fn record_rule_from_domains<R, L>(&mut self, rule_filter: RuleFilter<'static>)
    where
        L: biome_rowan::Language,
        R: Rule<Query: Queryable<Language = L, Output: Clone>> + 'static,
    {
        let no_only = self.only.is_some_and(|only| only.is_empty());
        let group = <R::Group as RuleGroup>::NAME;
        // Nursery rules must be enabled only when they are enabled from the group
        if group == "nursery" {
            return;
        }
        if !no_only {
            return;
        }

        let domains = self
            .settings
            .as_linter_domains(self.path.expect("File path"));

        // no domains, no need to record the rule
        if domains.as_ref().is_none_or(|d| d.is_empty()) {
            return;
        }

        // If the rule is recommended, and it has some domains, it should be disabled, but only if the configuration doesn't enable some domains.
        if R::METADATA.recommended
            && !R::METADATA.domains.is_empty()
            && domains.as_ref().is_none_or(|d| d.is_empty())
        {
            self.disabled_rules.insert(rule_filter);
            return;
        }

        for rule_domain in R::METADATA.domains {
            if let Some((configured_domain, configured_domain_value)) = domains
                .as_ref()
                .and_then(|domains| domains.get_key_value(rule_domain))
            {
                match configured_domain_value {
                    RuleDomainValue::All => {
                        self.enabled_rules.insert(rule_filter);
                        self.analyzer_options.push_globals(
                            configured_domain.globals().iter().copied().map(Into::into),
                        );
                    }
                    RuleDomainValue::None => {
                        self.disabled_rules.insert(rule_filter);
                    }
                    RuleDomainValue::Recommended => {
                        if R::METADATA.recommended {
                            self.enabled_rules.insert(rule_filter);
                            self.analyzer_options.push_globals(
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
        Vec<RuleFilter<'a>>,
    ) {
        let has_only_filter = self.only.is_none_or(|only| !only.is_empty());
        let rules = self
            .settings
            .as_linter_rules(self.path.expect("Path to be set"))
            .unwrap_or_default();
        if !has_only_filter {
            self.enabled_rules.extend(rules.as_enabled_rules());
            self.disabled_rules.extend(rules.as_disabled_rules());
        }
        let fixable_rules = self
            .enabled_rules
            .iter()
            .filter(|rule| self.rules_with_fix.contains(rule))
            .copied()
            .collect();
        (self.enabled_rules, self.disabled_rules, fixable_rules)
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

        // first we want to register rules via "magic default"
        self.record_rule_from_manifest::<R, L>(rule_filter);
        // then we want to register rules
        self.record_rule_from_domains::<R, L>(rule_filter);

        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        if let Some(only) = self.only {
            for selector in only {
                match selector {
                    AnalyzerSelector::Rule(selector) => {
                        let filter = RuleFilter::from(selector);
                        if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                            self.enabled_rules.insert(filter);
                        }
                    }
                    AnalyzerSelector::Domain(selector) => {
                        if selector.match_rule::<R>() {
                            self.enabled_rules.extend(selector.as_rule_filters());
                        }
                    }
                    AnalyzerSelector::Plugin => {}
                }
            }
        }
        if let Some(skip) = self.skip {
            for selector in skip {
                match selector {
                    AnalyzerSelector::Rule(selector) => {
                        let filter = RuleFilter::from(selector);
                        if filter.match_rule::<R>() && filter.match_group::<R::Group>() {
                            self.disabled_rules.insert(filter);
                        }
                    }
                    AnalyzerSelector::Domain(selector) => {
                        if selector.match_rule::<R>() {
                            self.disabled_rules.extend(selector.as_rule_filters());
                        }
                    }
                    AnalyzerSelector::Plugin => {}
                }
            }
        }

        if R::METADATA.fix_kind != FixKind::None {
            self.rules_with_fix.insert(rule_filter);
        }
    }
}

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

struct AssistsVisitor<'a, 'b> {
    settings: &'b Settings,
    enabled_rules: Vec<RuleFilter<'a>>,
    disabled_rules: Vec<RuleFilter<'a>>,
    /// Set of rules that have a code fix, regardless of whether they are enabled.
    /// Used after `finish()` to derive the fixable subset of `enabled_rules`.
    rules_with_fix: FxHashSet<RuleFilter<'a>>,
    only: Option<&'b [AnalyzerSelector]>,
    skip: Option<&'b [AnalyzerSelector]>,
    path: Option<&'b Utf8Path>,
}

impl<'a, 'b> AssistsVisitor<'a, 'b> {
    pub(crate) fn new(
        only: Option<&'b [AnalyzerSelector]>,
        skip: Option<&'b [AnalyzerSelector]>,
        settings: &'b Settings,
        path: Option<&'b Utf8Path>,
    ) -> Self {
        Self {
            enabled_rules: vec![],
            disabled_rules: vec![],
            rules_with_fix: Default::default(),
            settings,
            path,
            only,
            skip,
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

        // Do not report unused suppression comment diagnostics if:
        // - it is a syntax-only analyzer pass, or
        // - if a single rule is run.
        if let Some(only) = self.only {
            for selector in only {
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        let filter = RuleFilter::from(rule);
                        if filter.match_rule::<R>() {
                            self.enabled_rules.push(filter)
                        }
                    }
                    AnalyzerSelector::Domain(domain) => {
                        if domain.match_rule::<R>() {
                            self.enabled_rules.extend(domain.as_rule_filters());
                        }
                    }
                    AnalyzerSelector::Plugin => {}
                }
            }
        }

        if let Some(skip) = self.skip {
            for selector in skip {
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        let filter = RuleFilter::from(rule);
                        if filter.match_rule::<R>() {
                            self.disabled_rules.push(filter)
                        }
                    }
                    AnalyzerSelector::Domain(domain) => {
                        if domain.match_rule::<R>() {
                            self.disabled_rules.extend(domain.as_rule_filters());
                        }
                    }
                    // Plugins are lint-only; ignore them for assists.
                    AnalyzerSelector::Plugin => {}
                }
            }
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
        Vec<RuleFilter<'a>>,
    ) {
        let has_only_filter = self.only.is_none_or(|only| !only.is_empty());
        let rules = self
            .settings
            .as_assist_actions(self.path.expect("Path to be set"))
            .unwrap_or_default();
        if !has_only_filter {
            self.enabled_rules.extend(rules.as_enabled_rules());
            self.disabled_rules.extend(rules.as_disabled_rules());
        }
        let fixable_rules = self
            .enabled_rules
            .iter()
            .filter(|rule| self.rules_with_fix.contains(rule))
            .copied()
            .collect();
        (self.enabled_rules, self.disabled_rules, fixable_rules)
    }
}

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

/// Result of building the analyzer visitor: the resolved rule filters and options.
pub(crate) struct AnalyzerVisitorResult {
    pub(crate) enabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) disabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) analyzer_options: AnalyzerOptions,
    /// Subset of `enabled_rules` that have a code fix (`FixKind::Safe` or `FixKind::Unsafe`).
    pub(crate) fixable_rules: Vec<RuleFilter<'static>>,
}

pub(crate) struct AnalyzerVisitorBuilder<'a> {
    settings: &'a Settings,
    only: Option<&'a [AnalyzerSelector]>,
    skip: Option<&'a [AnalyzerSelector]>,
    path: Option<&'a Utf8Path>,
    enabled_selectors: Option<&'a [AnalyzerSelector]>,
    project_layout: Arc<ProjectLayout>,
    analyzer_options: AnalyzerOptions,
    cache: Option<&'a AnalyzerVisitorCache>,
}

impl<'b> AnalyzerVisitorBuilder<'b> {
    pub(crate) fn new(settings: &'b Settings, analyzer_options: AnalyzerOptions) -> Self {
        Self {
            settings,
            only: None,
            skip: None,
            path: None,
            enabled_selectors: None,
            project_layout: Default::default(),
            analyzer_options,
            cache: None,
        }
    }

    #[must_use]
    pub(crate) fn with_only(mut self, only: &'b [AnalyzerSelector]) -> Self {
        self.only = Some(only);
        self
    }

    #[must_use]
    pub(crate) fn with_skip(mut self, skip: &'b [AnalyzerSelector]) -> Self {
        self.skip = Some(skip);
        self
    }

    #[must_use]
    pub(crate) fn with_path(mut self, path: &'b Utf8Path) -> Self {
        self.path = Some(path);
        self
    }

    #[must_use]
    pub(crate) fn with_enabled_selectors(mut self, enabled_rules: &'b [AnalyzerSelector]) -> Self {
        self.enabled_selectors = Some(enabled_rules);
        self
    }

    #[must_use]
    pub(crate) fn with_project_layout(mut self, project_layout: Arc<ProjectLayout>) -> Self {
        self.project_layout = project_layout;
        self
    }

    #[must_use]
    pub(crate) fn with_cache(mut self, cache: &'b AnalyzerVisitorCache) -> Self {
        self.cache = Some(cache);
        self
    }

    fn compute_cache_key(&self) -> Option<AnalyzerCacheKey> {
        let path = self.path?;

        // 1. override_indices: which override patterns match this file.
        let override_indices: Vec<_> = self
            .settings
            .override_settings
            .patterns
            .iter()
            .enumerate()
            .filter_map(|(i, pattern)| pattern.is_file_included(path).then_some(i))
            .collect();

        // 2. manifest_id: identity of the nearest package.json.
        let manifest_id =
            self.project_layout
                .find_node_manifest_for_path(path)
                .map(|(manifest_path, _)| {
                    let mut h = FxHasher::default();
                    manifest_path.hash(&mut h);
                    h.finish()
                });

        // 3. jsx_factory_hash: JSX factory/fragment from tsconfig.
        let jsx_factory_hash =
            if self.analyzer_options.jsx_runtime() == Some(JsxRuntime::ReactClassic) {
                let mut h = FxHasher::default();
                // Query tsconfig for the jsx factory — same lookup as finish() does later.
                let factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig.jsx_factory_identifier().map(|s| s.to_string())
                    })
                    .flatten();
                let fragment_factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig
                            .jsx_fragment_factory_identifier()
                            .map(|s| s.to_string())
                    })
                    .flatten();
                factory.hash(&mut h);
                fragment_factory.hash(&mut h);
                h.finish()
            } else {
                0
            };

        // 4. filter_hash: only/skip/enabled_selectors from the command invocation.
        let filter_hash = {
            let mut h = FxHasher::default();
            if let Some(only) = self.only {
                for selector in only {
                    selector.hash(&mut h);
                }
            }
            if let Some(skip) = self.skip {
                for selector in skip {
                    selector.hash(&mut h);
                }
            }
            if let Some(enabled) = self.enabled_selectors {
                for selector in enabled {
                    selector.hash(&mut h);
                }
            }
            h.finish()
        };

        Some(AnalyzerCacheKey {
            override_indices,
            manifest_id,
            jsx_factory_hash,
            filter_hash,
        })
    }

    #[must_use]
    pub(crate) fn finish(self) -> AnalyzerVisitorResult {
        let key = self.compute_cache_key();
        // 1. Try cache hit
        if let Some(cache) = &self.cache
            && let Some(key) = key.clone()
            && let Some(cached) = cache.get_entry(&key)
        {
            // Cache HIT: apply stored mutations to the builder's owned AnalyzerOptions
            let mut opts = self.analyzer_options;
            opts.push_globals(cached.globals.iter().cloned());
            if let Some(f) = &cached.jsx_factory {
                opts.set_jsx_factory(Some(f.clone()));
            }
            if let Some(f) = &cached.jsx_fragment_factory {
                opts.set_jsx_fragment_factory(Some(f.clone()));
            }
            return AnalyzerVisitorResult {
                enabled_rules: cached.enabled_rules.clone(),
                disabled_rules: cached.disabled_rules.clone(),
                fixable_rules: cached.fixable_rules.clone(),
                analyzer_options: opts,
            };
        }

        let mut analyzer_options = self.analyzer_options;
        let mut disabled_rules = vec![];
        let mut enabled_rules = vec![];

        if let Some(enabled_selectors) = self.enabled_selectors {
            for selector in enabled_selectors {
                match selector {
                    AnalyzerSelector::Rule(rule) => {
                        enabled_rules.push(RuleFilter::from(rule));
                    }
                    AnalyzerSelector::Domain(domain) => {
                        enabled_rules.extend(domain.as_rule_filters())
                    }
                    // Plugins are configured through the `plugins` field, not `rules`.
                    AnalyzerSelector::Plugin => {}
                }
            }
        }

        // Plugins run by default. Turn them off by adding the reserved `plugin` group to
        // `disabled_rules`, which `AnalysisFilter::plugins_enabled` reads. Plugins are
        // disabled when explicitly skipped, or when `--only` restricts the run to other
        // selectors. `--skip` takes precedence over `--only`, matching rule semantics.
        let only_excludes_plugins = self
            .only
            .is_some_and(|only| !only.is_empty() && !only.contains(&AnalyzerSelector::Plugin));
        let plugins_skipped = self
            .skip
            .is_some_and(|skip| skip.contains(&AnalyzerSelector::Plugin));
        if only_excludes_plugins || plugins_skipped {
            disabled_rules.push(RuleFilter::Group(PLUGIN_GROUP));
        }

        let mut syntax = SyntaxVisitor::default();

        biome_js_analyze::visit_registry(&mut syntax);
        biome_css_analyze::visit_registry(&mut syntax);
        biome_json_analyze::visit_registry(&mut syntax);
        #[cfg(feature = "lang_graphql")]
        biome_graphql_analyze::visit_registry(&mut syntax);
        biome_html_analyze::visit_registry(&mut syntax);
        enabled_rules.extend(syntax.enabled_rules);

        let package_json = self
            .path
            .and_then(|path| self.project_layout.find_node_manifest_for_path(path))
            .map(|(_, manifest)| manifest);

        // Query tsconfig.json for JSX factory settings if jsx_runtime is ReactClassic
        // and the factory settings are not already set
        if let Some(path) = self.path
            && analyzer_options.jsx_runtime()
                == Some(biome_analyze::options::JsxRuntime::ReactClassic)
        {
            if analyzer_options.jsx_factory().is_none() {
                let factory = self
                    .project_layout
                    .query_tsconfig_for_path(path, |tsconfig| {
                        tsconfig.jsx_factory_identifier().map(|s| s.to_string())
                    })
                    .flatten();
                analyzer_options.set_jsx_factory(factory.map(|s| s.into()));
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
                analyzer_options.set_jsx_fragment_factory(fragment_factory.map(|s| s.into()));
            }
        }

        let mut lint = LintVisitor::new(
            self.only,
            self.skip,
            self.settings,
            self.path,
            package_json,
            &mut analyzer_options,
        );

        biome_js_analyze::visit_registry(&mut lint);
        biome_css_analyze::visit_registry(&mut lint);
        biome_json_analyze::visit_registry(&mut lint);
        #[cfg(feature = "lang_graphql")]
        biome_graphql_analyze::visit_registry(&mut lint);
        biome_html_analyze::visit_registry(&mut lint);
        let (linter_enabled_rules, linter_disabled_rules, linter_fixable_rules) = lint.finish();
        enabled_rules.extend(linter_enabled_rules);
        disabled_rules.extend(linter_disabled_rules);

        let mut assist = AssistsVisitor::new(self.only, self.skip, self.settings, self.path);

        biome_js_analyze::visit_registry(&mut assist);
        biome_css_analyze::visit_registry(&mut assist);
        biome_json_analyze::visit_registry(&mut assist);
        #[cfg(feature = "lang_graphql")]
        biome_graphql_analyze::visit_registry(&mut assist);
        biome_html_analyze::visit_registry(&mut assist);
        let (assists_enabled_rules, assists_disabled_rules, assists_fixable_rules) =
            assist.finish();
        enabled_rules.extend(assists_enabled_rules);
        disabled_rules.extend(assists_disabled_rules);

        let mut fixable_rules = linter_fixable_rules;
        fixable_rules.extend(assists_fixable_rules);

        let result = AnalyzerVisitorResult {
            enabled_rules,
            disabled_rules,
            analyzer_options,
            fixable_rules,
        };

        if let Some(key) = key.as_ref()
            && let Some(cache) = self.cache.as_ref()
        {
            let cached = CachedRuleSet::extract_from(&result);
            cache.insert_entry(key.clone(), cached);
        }

        result
    }
}

/// Reusable cache when building the [AnalysisFilter]
#[derive(Debug, Default)]
pub(crate) struct AnalyzerVisitorCache(HashMap<AnalyzerCacheKey, Arc<CachedRuleSet>>);

impl AnalyzerVisitorCache {
    // NOTE: for now, we want to have a more conservative approach.
    // We don't know what's the effect of the cache on the daemon, considering that it can hold multiple projects.
    /// Max entries for the current cache
    const MAX_ENTRIES: usize = 256;

    /// Evicts all entries from the current cache
    pub(crate) fn evict_cache(&self) {
        self.0.pin().clear();
    }

    pub(crate) fn get_entry(&self, key: &AnalyzerCacheKey) -> Option<Arc<CachedRuleSet>> {
        self.0.pin().get(key).cloned()
    }
    pub(crate) fn insert_entry(&self, key: AnalyzerCacheKey, entry: CachedRuleSet) {
        if self.0.len() > Self::MAX_ENTRIES {
            trace!("Evicted cache entry: {:?}", key);
            self.evict_cache();
        }

        self.0.pin().insert(key, Arc::new(entry));
    }
}

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub(crate) struct AnalyzerCacheKey {
    /// Which override patterns matched (indices into settings overrides).
    override_indices: Vec<usize>,
    /// Identity of the nearest package.json (path hash, or None).
    manifest_id: Option<u64>,
    /// JSX factory + fragment factory from tsconfig (if ReactClassic).
    jsx_factory_hash: u64,
    /// Hash of only/skip/enabled_selectors (constant within a batch).
    filter_hash: u64,
}

/// The cacheable output
#[derive(Debug, Clone)]
pub(crate) struct CachedRuleSet {
    pub(crate) enabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) disabled_rules: Vec<RuleFilter<'static>>,
    pub(crate) fixable_rules: Vec<RuleFilter<'static>>,
    /// Globals added during domain/manifest resolution.
    pub(crate) globals: Vec<Box<str>>,
    /// JSX factory from tsconfig (None if not ReactClassic or not resolved).
    pub(crate) jsx_factory: Option<Box<str>>,
    pub(crate) jsx_fragment_factory: Option<Box<str>>,
}

impl CachedRuleSet {
    fn extract_from(result: &AnalyzerVisitorResult) -> Self {
        Self {
            enabled_rules: result.enabled_rules.clone(),
            disabled_rules: result.disabled_rules.clone(),
            fixable_rules: result.fixable_rules.clone(),
            globals: result
                .analyzer_options
                .globals()
                .iter()
                .map(|s| s.to_string().into_boxed_str())
                .collect::<Vec<_>>(),
            jsx_factory: result
                .analyzer_options
                .jsx_factory()
                .map(|s| s.to_string().into_boxed_str()),
            jsx_fragment_factory: result
                .analyzer_options
                .jsx_fragment_factory()
                .map(|s| s.to_string().into_boxed_str()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DocumentFileSource, Features};
    use camino::Utf8Path;

    mod analyzer_cache {
        use super::super::{
            AnalyzerCacheKey, AnalyzerVisitorBuilder, AnalyzerVisitorCache, CachedRuleSet,
        };
        use biome_analyze::{AnalyzerOptions, RuleFilter};
        use biome_project_layout::ProjectLayout;
        use camino::Utf8Path;
        use std::sync::Arc;

        use crate::settings::Settings;

        #[test]
        fn insert_and_retrieve_entry() {
            let cache = AnalyzerVisitorCache::default();
            let key = AnalyzerCacheKey {
                override_indices: vec![],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            let entry = CachedRuleSet {
                enabled_rules: vec![RuleFilter::Rule("style", "noVar")],
                disabled_rules: vec![],
                fixable_rules: vec![],
                globals: vec![],
                jsx_factory: None,
                jsx_fragment_factory: None,
            };

            cache.insert_entry(key.clone(), entry);

            let retrieved = cache.get_entry(&key);
            assert!(retrieved.is_some());
            let retrieved = retrieved.unwrap();
            assert_eq!(retrieved.enabled_rules.len(), 1);
            assert_eq!(
                retrieved.enabled_rules[0],
                RuleFilter::Rule("style", "noVar")
            );
        }

        #[test]
        fn different_keys_produce_separate_entries() {
            let cache = AnalyzerVisitorCache::default();
            let key_a = AnalyzerCacheKey {
                override_indices: vec![0],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            let key_b = AnalyzerCacheKey {
                override_indices: vec![1],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };

            cache.insert_entry(
                key_a.clone(),
                CachedRuleSet {
                    enabled_rules: vec![RuleFilter::Rule("style", "noVar")],
                    disabled_rules: vec![],
                    fixable_rules: vec![],
                    globals: vec![],
                    jsx_factory: None,
                    jsx_fragment_factory: None,
                },
            );
            cache.insert_entry(
                key_b.clone(),
                CachedRuleSet {
                    enabled_rules: vec![RuleFilter::Rule("correctness", "noUnusedVariables")],
                    disabled_rules: vec![],
                    fixable_rules: vec![],
                    globals: vec![],
                    jsx_factory: None,
                    jsx_fragment_factory: None,
                },
            );

            let a = cache.get_entry(&key_a).unwrap();
            let b = cache.get_entry(&key_b).unwrap();
            assert_eq!(a.enabled_rules[0], RuleFilter::Rule("style", "noVar"));
            assert_eq!(
                b.enabled_rules[0],
                RuleFilter::Rule("correctness", "noUnusedVariables")
            );
        }

        #[test]
        fn evict_cache_clears_all_entries() {
            let cache = AnalyzerVisitorCache::default();
            let key = AnalyzerCacheKey {
                override_indices: vec![],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            cache.insert_entry(
                key.clone(),
                CachedRuleSet {
                    enabled_rules: vec![],
                    disabled_rules: vec![],
                    fixable_rules: vec![],
                    globals: vec![],
                    jsx_factory: None,
                    jsx_fragment_factory: None,
                },
            );

            assert!(cache.get_entry(&key).is_some());
            cache.evict_cache();
            assert!(cache.get_entry(&key).is_none());
        }

        #[test]
        fn max_entries_triggers_eviction() {
            let cache = AnalyzerVisitorCache::default();

            // Insert MAX_ENTRIES + 2 entries. Eviction triggers on the (MAX_ENTRIES+2)th
            // insert because `len() > MAX_ENTRIES` is checked before each insert.
            for i in 0..=(AnalyzerVisitorCache::MAX_ENTRIES + 1) {
                let key = AnalyzerCacheKey {
                    override_indices: vec![i],
                    manifest_id: None,
                    jsx_factory_hash: 0,
                    filter_hash: 0,
                };
                cache.insert_entry(
                    key,
                    CachedRuleSet {
                        enabled_rules: vec![],
                        disabled_rules: vec![],
                        fixable_rules: vec![],
                        globals: vec![],
                        jsx_factory: None,
                        jsx_fragment_factory: None,
                    },
                );
            }

            // After eviction, only the last inserted entry should remain.
            let first_key = AnalyzerCacheKey {
                override_indices: vec![0],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            assert!(
                cache.get_entry(&first_key).is_none(),
                "early entries should be evicted after exceeding MAX_ENTRIES"
            );

            // The last entry (inserted after the clear) should exist.
            let last_key = AnalyzerCacheKey {
                override_indices: vec![AnalyzerVisitorCache::MAX_ENTRIES + 1],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            assert!(
                cache.get_entry(&last_key).is_some(),
                "entry inserted after eviction should be present"
            );
        }

        #[test]
        fn same_path_produces_same_cache_key() {
            let settings = Settings::default();
            let project_layout = Arc::new(ProjectLayout::default());
            let options = AnalyzerOptions::default();
            let path = Utf8Path::new("src/index.ts");

            let builder_a = AnalyzerVisitorBuilder::new(&settings, options.clone())
                .with_path(path)
                .with_project_layout(project_layout.clone());
            let key_a = builder_a.compute_cache_key();

            let builder_b = AnalyzerVisitorBuilder::new(&settings, options)
                .with_path(path)
                .with_project_layout(project_layout);
            let key_b = builder_b.compute_cache_key();

            assert_eq!(key_a, key_b);
        }

        #[test]
        fn different_only_filters_produce_different_keys() {
            use biome_configuration::analyzer::AnalyzerSelector;
            use std::str::FromStr;

            let settings = Settings::default();
            let project_layout = Arc::new(ProjectLayout::default());
            let path = Utf8Path::new("src/index.ts");

            let only_a: Vec<AnalyzerSelector> = vec![];
            let builder_a = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                .with_path(path)
                .with_only(&only_a)
                .with_project_layout(project_layout.clone());
            let key_a = builder_a.compute_cache_key();

            let only_b = vec![AnalyzerSelector::from_str("lint/suspicious/noVar").unwrap()];
            let builder_b = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                .with_path(path)
                .with_only(&only_b)
                .with_project_layout(project_layout);
            let key_b = builder_b.compute_cache_key();

            assert_ne!(key_a, key_b);
        }

        #[test]
        fn no_path_returns_none_key() {
            let settings = Settings::default();
            let builder = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default());
            assert!(builder.compute_cache_key().is_none());
        }

        #[test]
        fn cache_hit_applies_globals_from_cached_entry() {
            let cache = AnalyzerVisitorCache::default();
            let settings = Settings::default();
            let project_layout = Arc::new(ProjectLayout::default());
            let path = Utf8Path::new("src/index.ts");

            let key = AnalyzerCacheKey {
                override_indices: vec![],
                manifest_id: None,
                jsx_factory_hash: 0,
                filter_hash: 0,
            };
            cache.insert_entry(
                key,
                CachedRuleSet {
                    enabled_rules: vec![RuleFilter::Rule("style", "noVar")],
                    disabled_rules: vec![],
                    fixable_rules: vec![],
                    globals: vec!["React".into(), "JSX".into()],
                    jsx_factory: Some("h".into()),
                    jsx_fragment_factory: Some("Fragment".into()),
                },
            );

            let result = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                .with_path(path)
                .with_project_layout(project_layout)
                .with_cache(&cache)
                .finish();

            assert_eq!(
                result.enabled_rules,
                vec![RuleFilter::Rule("style", "noVar")]
            );
            assert!(result.analyzer_options.globals().contains(&"React".into()));
            assert!(result.analyzer_options.globals().contains(&"JSX".into()));
            assert_eq!(result.analyzer_options.jsx_factory(), Some("h"));
            assert_eq!(
                result.analyzer_options.jsx_fragment_factory(),
                Some("Fragment")
            );
        }

        #[test]
        fn cache_miss_populates_cache_for_next_call() {
            let cache = AnalyzerVisitorCache::default();
            let settings = Settings::default();
            let project_layout = Arc::new(ProjectLayout::default());
            let path = Utf8Path::new("src/index.ts");

            // First call — cache miss, populates the cache
            let result_a = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                .with_path(path)
                .with_project_layout(project_layout.clone())
                .with_cache(&cache)
                .finish();

            // Second call — should be a cache hit with identical result
            let result_b = AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                .with_path(path)
                .with_project_layout(project_layout)
                .with_cache(&cache)
                .finish();

            assert_eq!(result_a.enabled_rules, result_b.enabled_rules);
            assert_eq!(result_a.disabled_rules, result_b.disabled_rules);
            assert_eq!(result_a.fixable_rules, result_b.fixable_rules);
        }

        #[test]
        fn filter_hash_distinguishes_skip_from_empty() {
            use biome_configuration::analyzer::AnalyzerSelector;
            use std::str::FromStr;

            let settings = Settings::default();
            let project_layout = Arc::new(ProjectLayout::default());
            let path = Utf8Path::new("src/index.ts");

            let empty: Vec<AnalyzerSelector> = vec![];
            let builder_no_skip =
                AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                    .with_path(path)
                    .with_skip(&empty)
                    .with_project_layout(project_layout.clone());
            let key_no_skip = builder_no_skip.compute_cache_key();

            let skip = vec![AnalyzerSelector::from_str("lint/suspicious/noVar").unwrap()];
            let builder_with_skip =
                AnalyzerVisitorBuilder::new(&settings, AnalyzerOptions::default())
                    .with_path(path)
                    .with_skip(&skip)
                    .with_project_layout(project_layout);
            let key_with_skip = builder_with_skip.compute_cache_key();

            assert_ne!(key_no_skip, key_with_skip);
        }
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
