use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch,
    Queryable, RuleKey, ServiceBag, SyntaxVisitor,
};
use biome_dependency_graph::{DependencyGraph, ModuleImports};
use biome_js_syntax::{AnyJsImportLike, AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_rowan::{AstNode, TextRange};
use camino::Utf8Path;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DependencyGraphService(Arc<DependencyGraph>);

impl DependencyGraphService {
    pub fn imports_for_path(&self, path: &Utf8Path) -> Option<ModuleImports> {
        self.0.imports_for_path(path)
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
pub struct ResolvedImports(AnyJsImportLike);

impl QueryMatch for ResolvedImports {
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl Queryable for ResolvedImports {
    type Input = JsSyntaxNode;
    type Output = AnyJsImportLike;

    type Language = JsLanguage;
    type Services = DependencyGraphService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(AnyJsImportLike::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        AnyJsImportLike::unwrap_cast(node.clone())
    }
}
