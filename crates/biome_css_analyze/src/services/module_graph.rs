use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssSyntaxNode};
use biome_module_graph::ModuleGraph;
use biome_rowan::AstNode;
use std::sync::Arc;

/// Service providing access to the [`ModuleGraph`] for CSS lint rules.
///
/// Only available for rules in the [`RuleDomain::Project`] domain.
#[derive(Debug, Clone)]
pub struct CssModuleGraphService(Arc<ModuleGraph>);

impl CssModuleGraphService {
    pub fn module_graph(&self) -> &ModuleGraph {
        self.0.as_ref()
    }
}

impl FromServices for CssModuleGraphService {
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
                    "The rule {rule_key} uses CssModuleGraphService, but it is not in the Project domain."
                );
            }
        }

        let module_graph: &Arc<ModuleGraph> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleGraph"]))?;

        Ok(Self(module_graph.clone()))
    }
}

impl Phase for CssModuleGraphService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type for CSS lint rules that require access to the [`CssModuleGraphService`].
///
/// Use `type Query = CssModuleGraph<CssClassSelector>` to query AST nodes while
/// also having access to the module graph via `ctx.module_graph()`.
///
/// ## Example
///
/// ```ignore
/// impl Rule for MyRule {
///     type Query = CssModuleGraph<CssClassSelector>;
///     fn run(ctx: &RuleContext<Self>) -> Self::Signals {
///         let node = ctx.query();
///         let graph = ctx.module_graph();
///         ...
///     }
/// }
/// ```
#[derive(Clone)]
pub struct CssModuleGraph<N>(pub N);

impl<N> Queryable for CssModuleGraph<N>
where
    N: AstNode<Language = CssLanguage> + 'static,
{
    type Input = CssSyntaxNode;
    type Output = N;
    type Language = CssLanguage;
    type Services = CssModuleGraphService;

    fn build_visitor(analyzer: &mut impl AddVisitor<CssLanguage>, _root: &AnyCssRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
