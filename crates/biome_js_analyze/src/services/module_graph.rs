use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch,
    Queryable, RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_module_graph::{JsModuleInfo, ModuleGraph};
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange};
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ModuleGraphService(Arc<ModuleGraph>);

impl ModuleGraphService {
    pub fn module_graph(&self) -> &ModuleGraph {
        self.0.as_ref()
    }

    pub fn module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.0.module_info_for_path(path)
    }
}

impl FromServices for ModuleGraphService {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let module_graph: &Arc<ModuleGraph> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["ModuleGraph"])
        })?;
        Ok(Self(module_graph.clone()))
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
