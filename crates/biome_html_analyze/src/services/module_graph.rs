use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_module_graph::ModuleDb;
use biome_project_layout::ProjectLayout;
use biome_rowan::AstNode;
use std::rc::Rc;
use std::sync::Arc;

/// Service providing access to the module database for HTML lint rules.
#[derive(Clone)]
pub struct HtmlDbService(Rc<dyn ModuleDb>, Arc<ProjectLayout>);

impl std::fmt::Debug for HtmlDbService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HtmlDbService").finish_non_exhaustive()
    }
}

impl HtmlDbService {
    pub fn db(&self) -> &dyn ModuleDb {
        self.0.as_ref()
    }

    pub fn project_layout(&self) -> &ProjectLayout {
        self.1.as_ref()
    }
}

impl FromServices for HtmlDbService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let module_db: &Rc<dyn ModuleDb> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleDb"]))?;

        let project_layout: &Arc<ProjectLayout> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"]))?;

        Ok(Self(module_db.clone(), project_layout.clone()))
    }
}

impl Phase for HtmlDbService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type for HTML lint rules that require access to the [`HtmlDbService`].
///
/// Use `type Query = HtmlModuleGraph<HtmlAttribute>` to query AST nodes while
/// also having access to the module database via `ctx.db()`.
#[derive(Clone)]
pub struct HtmlModuleGraph<N>(pub N);

impl<N> Queryable for HtmlModuleGraph<N>
where
    N: AstNode<Language = HtmlLanguage> + 'static,
{
    type Input = HtmlSyntaxNode;
    type Output = N;
    type Language = HtmlLanguage;
    type Services = HtmlDbService;

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
