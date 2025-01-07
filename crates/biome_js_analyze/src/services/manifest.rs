use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, Queryable,
    RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_package::PackageJson;
use biome_project_layout::ProjectLayout;
use biome_rowan::AstNode;
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct ManifestServices {
    pub(crate) manifest: Option<PackageJson>,
}

impl ManifestServices {
    pub(crate) fn name(&self) -> Option<&str> {
        self.manifest
            .as_ref()
            .as_ref()
            .and_then(|pkg| pkg.name.as_deref())
    }

    pub(crate) fn is_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_ref()
            .as_ref()
            .is_some_and(|pkg| pkg.dependencies.contains(specifier))
    }

    pub(crate) fn is_dev_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_ref()
            .as_ref()
            .is_some_and(|pkg| pkg.dev_dependencies.contains(specifier))
    }

    pub(crate) fn is_peer_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_ref()
            .as_ref()
            .is_some_and(|pkg| pkg.peer_dependencies.contains(specifier))
    }

    pub(crate) fn is_optional_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_ref()
            .as_ref()
            .is_some_and(|pkg| pkg.optional_dependencies.contains(specifier))
    }
}

impl FromServices for ManifestServices {
    fn from_services(
        rule_key: &RuleKey,
        services: &ServiceBag,
        file_path: &Utf8Path,
    ) -> biome_diagnostics::Result<Self, MissingServicesDiagnostic> {
        let project_layout: &Arc<ProjectLayout> = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["ProjectLayout"])
        })?;

        Ok(Self {
            manifest: project_layout.get_node_manifest_for_path(file_path),
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
