use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_json_syntax::{JsonLanguage, JsonRoot, JsonSyntaxNode};
use biome_project_layout::ProjectLayout;
use biome_rowan::AstNode;
use std::sync::Arc;

/// Service that provides access to the project layout for JSON rules.
///
/// This allows rules to look up `package.json` data from dependencies
/// in `node_modules` via [`ProjectLayout`].
#[derive(Debug)]
pub struct ProjectLayoutService(Option<Arc<ProjectLayout>>);

impl ProjectLayoutService {
    /// Returns a reference to the project layout, if available.
    pub fn project_layout(&self) -> Option<&ProjectLayout> {
        self.0.as_deref()
    }
}

impl FromServices for ProjectLayoutService {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let layout: &Option<Arc<ProjectLayout>> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"]))?;

        Ok(Self(layout.clone()))
    }
}

impl Phase for ProjectLayoutService {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that need the project layout** and matches
/// on specific [AstNode] types.
#[derive(Clone)]
pub struct PackageJsonFile<N>(pub N);

impl<N> Queryable for PackageJsonFile<N>
where
    N: AstNode<Language = JsonLanguage> + 'static,
{
    type Input = JsonSyntaxNode;
    type Output = N;

    type Language = JsonLanguage;
    type Services = ProjectLayoutService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsonLanguage>, _: &JsonRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
