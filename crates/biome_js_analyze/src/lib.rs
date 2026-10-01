#![deny(clippy::use_self)]
#![warn(clippy::needless_pass_by_value)]
#![expect(
    clippy::disallowed_methods,
    reason = "Some analyses require complete syntax text that can span multiple tokens."
)]

pub use crate::registry::visit_registry;
pub use crate::services::control_flow::ControlFlowGraph;
use crate::services::embedded::EmbeddedService;
pub use crate::services::react_compiler::{ReactCompilerResult, ReactCompilerServices};
use crate::services::semantic::SemanticModelBuilderVisitor;
use crate::services::typed::TypedModule;
pub use crate::suppression::JsSuppression;
use crate::suppression_action::JsSuppressionAction;
use biome_analyze::{
    AddVisitor, AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerPluginSlice,
    AnalyzerSignal, BatchPluginVisitor, ControlFlow, EmbeddedSignalInspector, InspectMatcher,
    LanguageRoot, MatchQueryParams, MetadataRegistry, Phases, PluginTargetLanguage, RuleAction,
    RuleRegistry, SnippetAnalyzer,
};
use biome_aria::AriaRoles;
use biome_db::AnyParsedSource;
use biome_diagnostics::Error as DiagnosticError;
use biome_embeds::EmbeddedData;
use biome_js_syntax::{AnyJsRoot, JsLanguage};
use biome_languages::{JsFileSource, LanguageDb};
use biome_module_graph::ModuleDb;
use biome_package::TurboJson;
use biome_project_layout::ProjectLayout;
use biome_tailwind_logic::syntax_service::TwSyntaxService;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, LazyLock};

mod a11y;
pub mod assist;
mod ast_utils;
mod frameworks;
pub mod globals;
pub mod lint;
mod nextjs;
mod react;
mod registry;
mod services;
pub mod shared;
mod suppression;
mod suppression_action;
mod syntax;
mod tailwind;
pub mod utils;

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

#[derive(Default)]
pub struct JsAnalyzerServices {
    module_db: Option<Rc<dyn ModuleDb>>,
    language_db: Option<Rc<dyn LanguageDb>>,
    parsed_source: Option<AnyParsedSource>,
    embedded_data: Option<Arc<EmbeddedData>>,
    project_layout: Arc<ProjectLayout>,
    source_type: JsFileSource,
}

struct AnalyzerParams<'a, 'guest, 'registry, 'snippet, 'analyzer, B, Output> {
    root: &'a LanguageRoot<JsLanguage>,
    filter: AnalysisFilter<'a>,
    options: &'a AnalyzerOptions,
    plugins: AnalyzerPluginSlice<'a>,
    services: JsAnalyzerServices,
    snippet_inspector: Option<EmbeddedSignalInspector<'guest, 'registry>>,
    snippets: Option<&'snippet mut [Box<dyn SnippetAnalyzer<B, Output = Output> + 'analyzer>]>,
}

impl From<(Rc<dyn ModuleDb>, Arc<ProjectLayout>, JsFileSource)> for JsAnalyzerServices {
    fn from(
        (module_db, project_layout, source_type): (
            Rc<dyn ModuleDb>,
            Arc<ProjectLayout>,
            JsFileSource,
        ),
    ) -> Self {
        Self {
            module_db: Some(module_db),
            language_db: None,
            parsed_source: None,
            embedded_data: None,
            project_layout,
            source_type,
        }
    }
}

impl From<&AnyJsRoot> for JsAnalyzerServices {
    fn from(_value: &AnyJsRoot) -> Self {
        Self {
            module_db: None,
            language_db: None,
            parsed_source: None,
            embedded_data: None,
            project_layout: Arc::new(ProjectLayout::default()),
            source_type: JsFileSource::default(),
        }
    }
}

impl JsAnalyzerServices {
    pub fn with_source_type(mut self, source_type: JsFileSource) -> Self {
        self.source_type = source_type;
        self
    }

    pub fn with_module_db(mut self, module_db: Rc<dyn ModuleDb>) -> Self {
        self.module_db = Some(module_db);
        self
    }

    pub fn with_language_db(mut self, language_db: Rc<dyn LanguageDb>) -> Self {
        self.language_db = Some(language_db);
        self
    }

    pub fn with_parsed_source(mut self, source: AnyParsedSource) -> Self {
        self.parsed_source = Some(source);
        self
    }

    pub fn with_embedded_data(mut self, embedded_data: Option<Arc<EmbeddedData>>) -> Self {
        self.embedded_data = embedded_data;
        self
    }

    pub fn with_project_layout(mut self, project_layout: Arc<ProjectLayout>) -> Self {
        self.project_layout = project_layout;
        self
    }
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action.
/// Additionally, this function takes a `inspect_matcher` function that can be
/// used to inspect the "query matches" emitted by the analyzer before they are
/// processed by the lint rules registry
pub fn analyze_with_inspect_matcher<'a, V, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    inspect_matcher: V,
    options: &'a AnalyzerOptions,
    plugins: AnalyzerPluginSlice<'a>,
    services: JsAnalyzerServices,
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_inspect_matcher_and_inspector::<V, F, B, ()>(
        AnalyzerParams {
            root,
            filter,
            options,
            plugins,
            services,
            snippet_inspector: None,
            snippets: None,
        },
        inspect_matcher,
        emit_signal,
    )
}

fn analyze_with_inspect_matcher_and_inspector<'a, V, F, B, Output>(
    params: AnalyzerParams<'a, '_, '_, '_, '_, B, Output>,
    inspect_matcher: V,
    mut emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let AnalyzerParams {
        root,
        filter,
        options,
        plugins,
        services,
        snippet_inspector,
        snippets,
    } = params;
    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let JsAnalyzerServices {
        module_db,
        language_db: embedded_db,
        parsed_source,
        embedded_data,
        project_layout,
        source_type,
    } = services;

    let (registry, mut services, diagnostics, mut visitors) = registry.build();

    let plugins: Vec<_> = plugins
        .iter()
        .filter(|p| p.language() == PluginTargetLanguage::JavaScript)
        .cloned()
        .collect();
    if filter.match_plugins()
        && plugins.iter().any(|plugin| {
            plugin.requires_semantic_model() && plugin.applies_to_file(&options.file_path)
        })
    {
        visitors.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
    }

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = Analyzer::new(
        METADATA.deref(),
        InspectMatcher::new(registry, inspect_matcher),
        Box::new(JsSuppression),
        Box::new(JsSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    if filter.match_plugins() && !plugins.is_empty() {
        // SAFETY: All plugins have been verified to target JavaScript above.
        unsafe {
            analyzer.add_visitor(
                Phases::Syntax,
                Box::new(BatchPluginVisitor::new_unchecked(&plugins)),
            );
        }
    }

    let file_path = options.file_path.clone();

    let node_manifest = project_layout
        .find_node_manifest_for_path(file_path.as_ref())
        .map(|(path, manifest)| (path, Arc::new(manifest)));

    let turborepo_configs: Vec<Arc<TurboJson>> =
        project_layout.find_all_turbo_json_for_path(file_path.as_ref());

    let type_resolver = module_db.as_ref().and_then(|db| {
        db.module_for_path(file_path.as_ref())
            .map(|module| TypedModule::new(db.clone(), module))
    });

    if let Some(parsed_source) = parsed_source {
        services.insert_service(parsed_source);
    }

    services.insert_service(Arc::new(AriaRoles));
    services.insert_service(TwSyntaxService::default());
    services.insert_service(source_type);
    if let Some(module_db) = module_db {
        services.insert_service(module_db);
    }
    services.insert_service(node_manifest);
    services.insert_service(turborepo_configs);
    services.insert_service(file_path);
    services.insert_service(type_resolver);
    services.insert_service(project_layout);
    if let Some(db) = &embedded_db {
        services.insert_service(db.clone());
    }
    if let Some(embedded_data) = embedded_data {
        services.insert_service(EmbeddedService::from_data(embedded_data));
    } else if let Some(embedded_db) = embedded_db {
        services.insert_service(EmbeddedService::new(embedded_db, options.file_path.clone()));
    }

    let ctx = AnalyzerContext {
        root: root.clone(),
        range: filter.range,
        services,
        options,
    };
    let result = match snippet_inspector {
        Some(inspector) => analyzer.run_snippet(ctx, inspector),
        None => match snippets {
            Some(snippets) => analyzer.run_with_snippets(ctx, snippets),
            None => analyzer.run(ctx),
        },
    };

    (result, diagnostics)
}

/// Run the analyzer on the provided `root`: this process will use the given `filter`
/// to selectively restrict analysis to specific rules / a specific source range,
/// then call `emit_signal` when an analysis rule emits a diagnostic or action
pub fn analyze<'a, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    plugins: AnalyzerPluginSlice<'a>,
    services: JsAnalyzerServices,
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_inspector::<F, B, ()>(
        AnalyzerParams {
            root,
            filter,
            options,
            plugins,
            services,
            snippet_inspector: None,
            snippets: None,
        },
        emit_signal,
    )
}

/// Analyzes JavaScript and embedded snippets together, allowing JavaScript
/// ignore comments to apply to findings from the snippets.
pub fn analyze_with_snippets<'a, F, B, Output>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    plugins: AnalyzerPluginSlice<'a>,
    services: JsAnalyzerServices,
    snippets: &mut [Box<dyn SnippetAnalyzer<B, Output = Output> + '_>],
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_inspector(
        AnalyzerParams {
            root,
            filter,
            options,
            plugins,
            services,
            snippet_inspector: None,
            snippets: Some(snippets),
        },
        emit_signal,
    )
}

/// Analyzes JavaScript embedded in another file, honoring ignore comments in both.
pub fn analyze_snippet<'a, F, B>(
    root: &LanguageRoot<JsLanguage>,
    filter: AnalysisFilter,
    options: &'a AnalyzerOptions,
    plugins: AnalyzerPluginSlice<'a>,
    services: JsAnalyzerServices,
    inspector: EmbeddedSignalInspector<'_, '_>,
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    analyze_with_optional_inspector::<F, B, ()>(
        AnalyzerParams {
            root,
            filter,
            options,
            plugins,
            services,
            snippet_inspector: Some(inspector),
            snippets: None,
        },
        emit_signal,
    )
}

fn analyze_with_optional_inspector<'a, F, B, Output>(
    params: AnalyzerParams<'a, '_, '_, '_, '_, B, Output>,
    emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    let module_db = params.services.module_db.clone();
    let language_db = params.services.language_db.clone();
    analyze_with_inspect_matcher_and_inspector(
        params,
        move |_| {
            if let Some(db) = module_db.as_ref() {
                db.unwind_if_revision_cancelled();
            }
            if let Some(db) = language_db.as_ref() {
                db.unwind_if_revision_cancelled();
            }
        },
        emit_signal,
    )
}

#[cfg(test)]
#[path = "suppressions.tests.rs"]
mod tests;
