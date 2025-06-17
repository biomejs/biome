use std::sync::Arc;

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_package::PackageJson;
use biome_rowan::AstNode;
use camino::Utf8PathBuf;

#[derive(Debug, Clone)]
pub struct ManifestServices {
    /// Path to the directory of the package in which the `package.json` was
    /// found.
    pub(crate) package_path: Option<Utf8PathBuf>,

    /// Contents of the `package.json` in the package.
    pub(crate) manifest: Option<Arc<PackageJson>>,
}

impl ManifestServices {
    pub(crate) fn name(&self) -> Option<&str> {
        self.manifest.as_deref().and_then(|pkg| pkg.name.as_deref())
    }

    pub(crate) fn is_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_deref()
            .is_some_and(|pkg| pkg.dependencies.contains(specifier))
    }

    pub(crate) fn is_dev_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_deref()
            .is_some_and(|pkg| pkg.dev_dependencies.contains(specifier))
    }

    pub(crate) fn is_peer_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_deref()
            .is_some_and(|pkg| pkg.peer_dependencies.contains(specifier))
    }

    pub(crate) fn is_optional_dependency(&self, specifier: &str) -> bool {
        self.manifest
            .as_deref()
            .is_some_and(|pkg| pkg.optional_dependencies.contains(specifier))
    }
}

impl FromServices for ManifestServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> biome_diagnostics::Result<Self, ServicesDiagnostic> {
        let manifest_info: &Option<(Utf8PathBuf, Arc<PackageJson>)> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["PackageJson"]))?;

        let (package_path, manifest) = match manifest_info {
            Some((package_path, manifest)) => (Some(package_path.clone()), Some(manifest.clone())),
            None => (None, None),
        };

        Ok(Self {
            package_path,
            manifest,
        })
    }
}

impl Phase for ManifestServices {
    fn phase() -> Phases {
        Phases::Syntax
    }
}

/// Query type usable by lint rules **that uses the package manifest** and matches on specific [AstNode] types.
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
