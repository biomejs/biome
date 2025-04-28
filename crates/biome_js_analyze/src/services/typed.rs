use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_syntax::{AnyJsExpression, AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_js_type_info::Type;
use biome_module_graph::{JsModuleInfo, ModuleGraph};
use biome_rowan::{AstNode, TextRange};
use camino::Utf8PathBuf;
use std::sync::Arc;

/// Service for use with type inference rules.
#[derive(Debug, Clone)]
pub struct TypedService {
    module_info: Option<JsModuleInfo>,
    modules: Arc<ModuleGraph>,
}

impl TypedService {
    pub fn module_graph(&self) -> &ModuleGraph {
        self.modules.as_ref()
    }

    pub fn type_for_expression(&self, expr: &AnyJsExpression) -> Type {
        self.module_info
            .as_ref()
            .map(|module_info| module_info.resolved_type_for_expression(expr, self.modules.clone()))
            .unwrap_or_default()
    }
}

impl FromServices for TypedService {
    fn from_services(
        rule_key: &RuleKey,
        rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        if cfg!(debug_assertions) {
            let has_project_domain = rule_metadata
                .domains
                .iter()
                .any(|d| d == &RuleDomain::Project);
            if !has_project_domain {
                panic!(
                    "The rule {rule_key} uses TypedService, but it is not in the project domain."
                );
            }
        }

        let file_path: &Arc<Utf8PathBuf> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["FilePath"]))?;
        let module_graph: &Arc<ModuleGraph> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleGraph"]))?;
        let module_info = module_graph.module_info_for_path(file_path.as_ref());
        Ok(Self {
            module_info,
            modules: module_graph.clone(),
        })
    }
}

impl Phase for TypedService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules that wish to perform type inference on
/// nodes.
#[derive(Clone)]
pub struct Typed<N>(N);

impl<N> QueryMatch for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N> Queryable for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TypedService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
