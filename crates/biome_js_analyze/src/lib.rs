#![deny(clippy::use_self)]
#![warn(clippy::needless_pass_by_value)]

pub use crate::registry::visit_registry;
pub use crate::services::control_flow::ControlFlowGraph;
use crate::services::embedded_bindings::EmbeddedBindings;
use crate::services::embedded_value_references::EmbeddedValueReferences;
use crate::suppression_action::JsSuppressionAction;
use biome_analyze::{
    AnalysisFilter, Analyzer, AnalyzerContext, AnalyzerOptions, AnalyzerPluginSlice,
    AnalyzerSignal, AnalyzerSuppression, ControlFlow, InspectMatcher, LanguageRoot,
    MatchQueryParams, MetadataRegistry, Phases, PluginTargetLanguage, PluginVisitor, RuleAction,
    RuleRegistry, to_analyzer_suppressions,
};
use biome_aria::AriaRoles;
use biome_diagnostics::Error as DiagnosticError;
use biome_js_syntax::{JsFileSource, JsLanguage};
use biome_module_graph::{ModuleGraph, ModuleResolver};
use biome_package::TurboJson;
use biome_project_layout::ProjectLayout;
use biome_rowan::{TextRange, TokenText};
use biome_suppression::{SuppressionDiagnostic, parse_suppression_comment};
use rustc_hash::FxHashMap;
use std::ops::Deref;
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
mod suppression_action;
mod syntax;
pub mod utils;

pub(crate) type JsRuleAction = RuleAction<JsLanguage>;

pub static METADATA: LazyLock<MetadataRegistry> = LazyLock::new(|| {
    let mut metadata = MetadataRegistry::default();
    visit_registry(&mut metadata);
    metadata
});

#[derive(Default)]
pub struct JsAnalyzerServices {
    module_graph: Arc<ModuleGraph>,
    project_layout: Arc<ProjectLayout>,
    source_type: JsFileSource,
    embedded_bindings: Vec<FxHashMap<TextRange, TokenText>>,
    embedded_value_references: Vec<FxHashMap<TextRange, TokenText>>,
}

impl From<(Arc<ModuleGraph>, Arc<ProjectLayout>, JsFileSource)> for JsAnalyzerServices {
    fn from(
        (module_graph, project_layout, source_type): (
            Arc<ModuleGraph>,
            Arc<ProjectLayout>,
            JsFileSource,
        ),
    ) -> Self {
        Self {
            module_graph,
            project_layout,
            source_type,
            embedded_bindings: Default::default(),
            embedded_value_references: Default::default(),
        }
    }
}

impl JsAnalyzerServices {
    pub fn set_embedded_bindings(&mut self, bindings: Vec<FxHashMap<TextRange, TokenText>>) {
        self.embedded_bindings = bindings;
    }

    pub fn set_embedded_value_references(&mut self, refs: Vec<FxHashMap<TextRange, TokenText>>) {
        self.embedded_value_references = refs;
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
    mut emit_signal: F,
) -> (Option<B>, Vec<DiagnosticError>)
where
    V: FnMut(&MatchQueryParams<JsLanguage>) + 'a,
    F: FnMut(&dyn AnalyzerSignal<JsLanguage>) -> ControlFlow<B> + 'a,
    B: 'a,
{
    fn parse_linter_suppression_comment(
        text: &str,
        piece_range: TextRange,
    ) -> Vec<Result<AnalyzerSuppression<'_>, SuppressionDiagnostic>> {
        let mut result = Vec::new();

        for comment in parse_suppression_comment(text) {
            let suppression = match comment {
                Ok(suppression) => suppression,
                Err(err) => {
                    result.push(Err(err));
                    continue;
                }
            };

            let analyzer_suppressions: Vec<_> = to_analyzer_suppressions(suppression, piece_range)
                .into_iter()
                .map(Ok)
                .collect();

            result.extend(analyzer_suppressions)
        }

        result
    }

    let mut registry = RuleRegistry::builder(&filter, root);
    visit_registry(&mut registry);

    let JsAnalyzerServices {
        module_graph,
        project_layout,
        source_type,
        embedded_bindings,
        embedded_value_references,
    } = services;

    let (registry, mut services, diagnostics, visitors) = registry.build();

    // Bail if we can't parse a rule option
    if !diagnostics.is_empty() {
        return (None, diagnostics);
    }

    let mut analyzer = Analyzer::new(
        METADATA.deref(),
        InspectMatcher::new(registry, inspect_matcher),
        parse_linter_suppression_comment,
        Box::new(JsSuppressionAction),
        &mut emit_signal,
    );

    for ((phase, _), visitor) in visitors {
        analyzer.add_visitor(phase, visitor);
    }

    for plugin in plugins {
        // SAFETY: The plugin target language is correctly checked here.
        unsafe {
            if plugin.language() == PluginTargetLanguage::JavaScript {
                analyzer.add_visitor(
                    Phases::Syntax,
                    Box::new(PluginVisitor::new_unchecked(plugin.clone())),
                )
            }
        }
    }

    let file_path = options.file_path.clone();

    let node_manifest = project_layout
        .find_node_manifest_for_path(file_path.as_ref())
        .map(|(path, manifest)| (path, Arc::new(manifest)));

    let turborepo_configs: Vec<Arc<TurboJson>> =
        project_layout.find_all_turbo_json_for_path(file_path.as_ref());

    let type_resolver = module_graph
        .js_module_info_for_path(file_path.as_ref())
        .map(|module_info| ModuleResolver::for_module(module_info, module_graph.clone()))
        .map(Arc::new);

    services.insert_service(Arc::new(AriaRoles));
    services.insert_service(source_type);
    services.insert_service(module_graph);
    services.insert_service(node_manifest);
    services.insert_service(turborepo_configs);
    services.insert_service(file_path);
    services.insert_service(type_resolver);
    services.insert_service(project_layout);
    services.insert_service(EmbeddedBindings(embedded_bindings));
    services.insert_service(EmbeddedValueReferences(embedded_value_references));

    (
        analyzer.run(AnalyzerContext {
            root: root.clone(),
            range: filter.range,
            services,
            options,
        }),
        diagnostics,
    )
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
    analyze_with_inspect_matcher(
        root,
        filter,
        |_| {},
        options,
        plugins,
        services,
        emit_signal,
    )
}

#[cfg(test)]
#[path = "suppressions.tests.rs"]
mod tests;
