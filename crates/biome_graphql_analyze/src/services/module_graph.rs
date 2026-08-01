use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_graphql_syntax::{GraphqlLanguage, GraphqlRoot, GraphqlSyntaxNode};
use biome_module_graph::ModuleDb;
use biome_rowan::AstNode;
use std::rc::Rc;

/// Service providing access to the optional module database for GraphQL lint rules.
#[derive(Clone, Default)]
pub struct GraphqlDbService(Option<Rc<dyn ModuleDb>>);

impl std::fmt::Debug for GraphqlDbService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GraphqlDbService").finish_non_exhaustive()
    }
}

impl GraphqlDbService {
    pub fn db(&self) -> Option<&dyn ModuleDb> {
        self.0.as_deref()
    }
}

impl FromServices for GraphqlDbService {
    fn from_services(
        _rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let module_db = services.get_service::<Rc<dyn ModuleDb>>().cloned();
        Ok(Self(module_db))
    }
}

impl Phase for GraphqlDbService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type for GraphQL lint rules that can optionally inspect module-graph data.
#[derive(Clone)]
pub struct GraphqlModuleGraph<N>(pub N);

impl<N> Queryable for GraphqlModuleGraph<N>
where
    N: AstNode<Language = GraphqlLanguage> + 'static,
{
    type Input = GraphqlSyntaxNode;
    type Output = N;
    type Language = GraphqlLanguage;
    type Services = GraphqlDbService;

    fn build_visitor(analyzer: &mut impl AddVisitor<GraphqlLanguage>, _root: &GraphqlRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
