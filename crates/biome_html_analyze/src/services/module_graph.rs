use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_module_graph::ModuleGraph;
use biome_project_layout::ProjectLayout;
use biome_rowan::AstNode;
use std::sync::Arc;

/// Service providing access to the [`ModuleGraph`] for HTML lint rules.
#[derive(Debug, Clone)]
pub struct HtmlModuleGraphService(Arc<ModuleGraph>, Arc<ProjectLayout>);

impl HtmlModuleGraphService {
    pub fn module_graph(&self) -> &ModuleGraph {
        self.0.as_ref()
    }

    pub fn project_layout(&self) -> &ProjectLayout {
        self.1.as_ref()
    }
}

impl FromServices for HtmlModuleGraphService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let module_graph: &Arc<ModuleGraph> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleGraph"]))?;

        let project_layout: &Arc<ProjectLayout> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"]))?;

        Ok(Self(module_graph.clone(), project_layout.clone()))
    }
}

impl Phase for HtmlModuleGraphService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type for HTML lint rules that require access to the [`HtmlModuleGraphService`].
///
/// Use `type Query = HtmlModuleGraph<HtmlAttribute>` to query AST nodes while
/// also having access to the module graph via `ctx.module_graph()`.
#[derive(Clone)]
pub struct HtmlModuleGraph<N>(pub N);

impl<N> Queryable for HtmlModuleGraph<N>
where
    N: AstNode<Language = HtmlLanguage> + 'static,
{
    type Input = HtmlSyntaxNode;
    type Output = N;
    type Language = HtmlLanguage;
    type Services = HtmlModuleGraphService;

    fn build_visitor(analyzer: &mut impl AddVisitor<HtmlLanguage>, _root: &HtmlRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
