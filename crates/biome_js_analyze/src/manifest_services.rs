use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, Queryable,
    RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_project::PackageJson;
use biome_rowan::AstNode;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ManifestServices {
    pub(crate) manifest: Arc<PackageJson>,
}

impl ManifestServices {
    pub(crate) fn is_dependency(&self, specifier: &str) -> bool {
        self.manifest.dependencies.contains(specifier)
    }

    pub(crate) fn is_dev_dependency(&self, specifier: &str) -> bool {
        self.manifest.dev_dependencies.contains(specifier)
    }

    #[allow(dead_code)]
    pub(crate) fn is_optional_dependency(&self, specifier: &str) -> bool {
        self.manifest.optional_dependencies.contains(specifier)
    }
}

impl FromServices for ManifestServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, MissingServicesDiagnostic> {
        let manifest: &Arc<PackageJson> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["PackageJson"])
        })?;

        Ok(Self {
            manifest: manifest.clone(),
        })
    }
}

impl Phase for ManifestServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Manifest<N>(pub N);

impl<N> Queryable for Manifest<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = ManifestServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
