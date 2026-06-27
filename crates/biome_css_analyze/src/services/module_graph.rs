use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssSyntaxNode};
use biome_module_graph::ModuleDb;
use biome_rowan::AstNode;
use std::rc::Rc;

/// Service providing access to the module database for CSS lint rules.
///
/// Only available for rules in the [`RuleDomain::Project`] domain.
#[derive(Clone)]
pub struct CssDbService(Rc<dyn ModuleDb>);

impl std::fmt::Debug for CssDbService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CssDbService").finish_non_exhaustive()
    }
}

impl CssDbService {
    pub fn db(&self) -> &dyn ModuleDb {
        self.0.as_ref()
    }
}

impl FromServices for CssDbService {
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
                    "The rule {rule_key} uses CssDbService, but it is not in the Project domain."
                );
            }
        }

        let module_db: &Rc<dyn ModuleDb> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ModuleDb"]))?;

        Ok(Self(module_db.clone()))
    }
}

impl Phase for CssDbService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type for CSS lint rules that require access to the [`CssDbService`].
///
/// Use `type Query = CssModuleGraph<CssClassSelector>` to query AST nodes while
/// also having access to the module database via `ctx.db()`.
#[derive(Clone)]
pub struct CssModuleGraph<N>(pub N);

impl<N> Queryable for CssModuleGraph<N>
where
    N: AstNode<Language = CssLanguage> + 'static,
{
    type Input = CssSyntaxNode;
    type Output = N;
    type Language = CssLanguage;
    type Services = CssDbService;

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
