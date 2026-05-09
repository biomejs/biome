use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_module_graph::{JsModuleInfo, ModuleDb};
use biome_project_layout::ProjectLayout;
use biome_rowan::{AstNode, Language, SyntaxNode, TextRange};
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Clone)]
pub struct DbService(Arc<dyn ModuleDb>, Arc<ProjectLayout>);

impl std::fmt::Debug for DbService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DbService").finish_non_exhaustive()
    }
}

impl DbService {
    pub fn db(&self) -> &dyn ModuleDb {
        self.0.as_ref()
    }

    pub fn module_info_for_path(&self, path: &Utf8Path) -> Option<JsModuleInfo> {
        self.0.js_module_info_for_path(path)
    }

    pub fn project_layout(&self) -> &ProjectLayout {
        self.1.as_ref()
    }
}

impl FromServices for DbService {
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
                panic!("The rule {rule_key} uses DbService, but it is not in the Project domain.");
            }
        }
        let module_db: &Arc<dyn ModuleDb> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleDb"]))?;

        let project_layout: &Arc<ProjectLayout> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"]))?;

        Ok(Self(module_db.clone(), project_layout.clone()))
    }
}

impl Phase for DbService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules that matches import statements and uses the
/// database to resolve their specifiers.
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
    type Services = DbService;

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
