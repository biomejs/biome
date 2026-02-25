use std::str::FromStr;
use std::sync::Arc;

use biome_analyze::{
    Phases, PluginDiagnosticEntry, PluginEvaluationResult, PluginTargetLanguage, RuleCategory,
    RuleDiagnostic, RuleDomain, ServiceBag,
};
use biome_console::markup;
use biome_diagnostics::{MessageAndDescription, category};
use biome_js_semantic::SemanticModel;
use biome_module_graph::ModuleResolver;
use biome_rowan::{AnySyntaxNode, RawSyntaxKind, TextRange};
use biome_wasm_plugin::WasmPluginEngine;
use camino::{Utf8Path, Utf8PathBuf};

use crate::diagnostics::CompileDiagnostic;
use crate::{AnalyzerPlugin, PluginDiagnostic};

/// An analyzer plugin backed by a WASM Component Model module.
///
/// A single WASM module may expose multiple rules. Each `AnalyzerWasmPlugin`
/// represents one rule and shares the engine via `Arc<WasmPluginEngine>`.
#[derive(Debug)]
pub struct AnalyzerWasmPlugin {
    engine: Arc<WasmPluginEngine>,
    rule_name: String,
    target_language: PluginTargetLanguage,
    query_kinds: Vec<RawSyntaxKind>,
    /// JSON-serialized options string, passed to the guest `configure` export.
    options_json: Option<String>,
    /// Rule category parsed from metadata.
    rule_category: RuleCategory,
    /// Domains this rule belongs to.
    rule_domains: Vec<RuleDomain>,
    /// Deprecation reason, if the rule is deprecated.
    rule_deprecated: Option<String>,
    /// GitHub issue number for WIP rules.
    rule_issue_number: Option<String>,
    /// Whether this rule is recommended (defaults to true if not specified).
    rule_recommended: bool,
}

impl AnalyzerWasmPlugin {
    /// Load a WASM plugin and return one `AnalyzerWasmPlugin` per rule it exposes.
    pub fn load(
        path: &Utf8Path,
        options_json: Option<String>,
    ) -> Result<Vec<Self>, PluginDiagnostic> {
        let bytes = std::fs::read(path.as_std_path()).map_err(|err| {
            PluginDiagnostic::Compile(CompileDiagnostic {
                message: MessageAndDescription::from(
                    markup! {
                        "Failed to read WASM plugin: "{err.to_string()}
                    }
                    .to_owned(),
                ),
                source: None,
            })
        })?;

        let engine = Arc::new(WasmPluginEngine::new(&bytes)?);
        let metadata = engine.metadata()?;

        let target_language = match metadata.language.as_str() {
            "javascript" => PluginTargetLanguage::JavaScript,
            "css" => PluginTargetLanguage::Css,
            "json" => PluginTargetLanguage::Json,
            other => {
                return Err(PluginDiagnostic::Compile(CompileDiagnostic {
                    message: MessageAndDescription::from(
                        markup! {
                            "WASM plugin declared unsupported target language: "
                            <Emphasis>{other}</Emphasis>
                        }
                        .to_owned(),
                    ),
                    source: None,
                }));
            }
        };

        // Treat empty or trivial "{}" options as None to skip the per-node
        // configure() WASM call when no real options are provided.
        let options_json = options_json.filter(|s| !s.is_empty() && s != "{}");

        let mut plugins = Vec::with_capacity(metadata.rule_names.len());

        for rule_name in &metadata.rule_names {
            let raw_kinds = metadata
                .query_kinds_by_rule
                .get(rule_name)
                .cloned()
                .unwrap_or_default();

            let query_kinds = raw_kinds
                .into_iter()
                .map(|k| {
                    u16::try_from(k).map(RawSyntaxKind).map_err(|_| {
                        PluginDiagnostic::Compile(CompileDiagnostic {
                            message: MessageAndDescription::from(
                                markup! {
                                    "WASM plugin query kind exceeds u16::MAX: "{k.to_string()}
                                }
                                .to_owned(),
                            ),
                            source: None,
                        })
                    })
                })
                .collect::<Result<_, _>>()?;

            // Parse per-rule metadata fields.
            let rule_meta = metadata.rule_metadata_by_rule.get(rule_name);

            let rule_category = rule_meta
                .and_then(|m| m.category.as_deref())
                .and_then(|s| RuleCategory::from_str(s).ok())
                .unwrap_or(RuleCategory::Lint);

            let rule_domains: Vec<RuleDomain> = rule_meta
                .map(|m| {
                    m.domains
                        .iter()
                        .filter_map(|s| RuleDomain::from_str(s).ok())
                        .collect()
                })
                .unwrap_or_default();

            let rule_deprecated = rule_meta.and_then(|m| m.deprecated.clone());
            let rule_issue_number = rule_meta.and_then(|m| m.issue_number.clone());

            let rule_recommended = rule_meta.is_none_or(|m| m.recommended);

            plugins.push(Self {
                engine: Arc::clone(&engine),
                rule_name: rule_name.clone(),
                target_language,
                query_kinds,
                options_json: options_json.clone(),
                rule_category,
                rule_domains,
                rule_deprecated,
                rule_issue_number,
                rule_recommended,
            });
        }

        Ok(plugins)
    }
}

impl AnalyzerPlugin for AnalyzerWasmPlugin {
    fn language(&self) -> PluginTargetLanguage {
        self.target_language
    }

    fn phase(&self) -> Phases {
        // JS plugins run in the Semantic phase to access the semantic model.
        // CSS/JSON plugins stay in Syntax since they have no semantic model support yet.
        match self.target_language {
            PluginTargetLanguage::JavaScript => Phases::Semantic,
            _ => Phases::Syntax,
        }
    }

    fn query(&self) -> Vec<RawSyntaxKind> {
        self.query_kinds.clone()
    }

    fn rule_name(&self) -> &str {
        &self.rule_name
    }

    fn category(&self) -> RuleCategory {
        self.rule_category
    }

    fn domains(&self) -> &[RuleDomain] {
        &self.rule_domains
    }

    fn is_recommended(&self) -> bool {
        self.rule_recommended
    }

    fn deprecated(&self) -> Option<&str> {
        self.rule_deprecated.as_deref()
    }

    fn issue_number(&self) -> Option<&str> {
        self.rule_issue_number.as_deref()
    }

    fn evaluate(
        &self,
        node: AnySyntaxNode,
        path: Arc<Utf8PathBuf>,
        services: &ServiceBag,
    ) -> PluginEvaluationResult {
        // Extract SemanticModel from services when available (JS semantic phase).
        let semantic_model = services.get_service::<SemanticModel>().cloned();

        // Extract ModuleResolver from services when available (JS type inference).
        let module_resolver: Option<Arc<ModuleResolver>> = services
            .get_service::<Option<Arc<ModuleResolver>>>()
            .and_then(|opt| opt.as_ref().map(Arc::clone));

        let file_path = path.as_str().to_string();

        match self.engine.check_node(
            node,
            &self.rule_name,
            self.target_language,
            semantic_model,
            module_resolver,
            file_path,
            self.options_json.as_deref(),
        ) {
            Ok(diagnostics) => PluginEvaluationResult { diagnostics },
            Err(error) => PluginEvaluationResult {
                diagnostics: vec![PluginDiagnosticEntry {
                    diagnostic: RuleDiagnostic::new(
                        category!("plugin"),
                        None::<TextRange>,
                        markup!(
                            <Emphasis>{&self.rule_name}</Emphasis>
                            " errored: "<Error>{error.to_string()}</Error>
                        ),
                    )
                    .subcategory(self.rule_name.clone()),
                    actions: vec![],
                }],
            },
        }
    }
}
