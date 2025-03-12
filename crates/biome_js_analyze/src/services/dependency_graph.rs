use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch,
    Queryable, RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_dependency_graph::{DependencyGraph, ModuleDependencyData};
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange};
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DependencyGraphService(Arc<DependencyGraph>);

impl DependencyGraphService {
    pub fn imports_for_path(&self, path: &Utf8Path) -> Option<ModuleDependencyData> {
        self.0.dependency_data_for_path(path)
    }
}

impl FromServices for DependencyGraphService {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let dependency_graph: &Arc<DependencyGraph> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["DependencyGraph"])
        })?;
        Ok(Self(dependency_graph.clone()))
    }
}

impl Phase for DependencyGraphService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules that matches import statements and uses the
/// [DependencyGraph] to resolve their specifiers.
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
    type Services = DependencyGraphService;

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
