use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_module_graph::{JsModuleInfo, ModuleGraph};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange};
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ModuleGraphService(Arc<ModuleGraph>, Arc<ProjectLayout>);

impl ModuleGraphService {
    pub fn module_graph(&self) -> &ModuleGraph {
        self.0.as_ref()
    }

    pub fn module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.0.module_info_for_path(path)
    }

    pub fn project_layout(&self) -> &ProjectLayout {
        self.1.as_ref()
    }
}

impl FromServices for ModuleGraphService {
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
                    "The rule {rule_key} uses ModuleGraphService, but it is not in the project domain."
                );
            }
        }
        let module_graph: &Arc<ModuleGraph> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleGraph"]))?;

        let project_layout: &Arc<ProjectLayout> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"]))?;

        Ok(Self(module_graph.clone(), project_layout.clone()))
    }
}

impl Phase for ModuleGraphService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules that matches import statements and uses the
/// [ModuleGraph] to resolve their specifiers.
#[derive(Clone)]
pub struct ResolvedImports<N>(N);

impl<N, L> QueryMatch for ResolvedImports<N>
where
    L: Language,
    N: AstNode<Language = L> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N, L> Queryable for ResolvedImports<N>
where
    L: Language + 'static,
    N: AstNode<Language = L> + 'static,
{
    type Input = SyntaxNode<L>;
    type Output = N;

    type Language = L;
    type Services = ModuleGraphService;

    fn build_visitor(analyzer: &mut impl AddVisitor<L>, _: &L::Root) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
