use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_js_syntax::{
    AnyJsBinding, AnyJsExpression, AnyJsFunction, AnyJsRoot, JsLanguage, JsSyntaxNode,
};
use biome_js_type_info::Type;
use biome_module_graph::ModuleResolver;
use biome_rowan::{AstNode, TextRange};
use std::sync::Arc;

/// Service for use with type inference rules.
///
/// This service is used for retrieving [`Type`] instances for arbitrary
/// expressions or function definitions from the module graph.
#[derive(Clone)]
pub struct TypedService {
    resolver: Option<Arc<ModuleResolver>>,
}

impl TypedService {
    /// Returns the [`Type`] for the given `expression`.
    pub fn type_of_expression(&self, expression: &AnyJsExpression) -> Type {
        self.resolver
            .as_ref()
            .map(|resolver| resolver.resolved_type_of_expression(expression))
            .unwrap_or_default()
    }

    /// Returns the [`Type`] for the given `function`.
    pub fn type_of_function(&self, function: &AnyJsFunction) -> Type {
        match function {
            AnyJsFunction::JsArrowFunctionExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsArrowFunctionExpression(expr.clone()))
            }
            AnyJsFunction::JsFunctionDeclaration(decl) => decl
                .id()
                .ok()
                .as_ref()
                .and_then(AnyJsBinding::as_js_identifier_binding)
                .and_then(|identifier| identifier.name_token().ok())
                .and_then(|name| {
                    self.resolver.as_ref().map(|resolver| {
                        resolver.resolved_type_of_named_value(function.range(), name.text())
                    })
                })
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExportDefaultDeclaration(_decl) => self
                .resolver
                .as_ref()
                .and_then(|resolver| resolver.resolved_type_of_default_export())
                .unwrap_or_default(),
            AnyJsFunction::JsFunctionExpression(expr) => {
                self.type_of_expression(&AnyJsExpression::JsFunctionExpression(expr.clone()))
            }
        }
    }
}

impl FromServices for TypedService {
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
                    "The rule {rule_key} uses TypedService, but it is not in the project domain."
                );
            }
        }

        let resolver: Option<&Option<Arc<ModuleResolver>>> = services.get_service();
        let resolver = resolver.and_then(|resolver| resolver.as_ref().map(Arc::clone));
        Ok(Self { resolver })
    }
}

impl Phase for TypedService {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type usable by lint rules that wish to perform type inference on
/// nodes.
#[derive(Clone)]
pub struct Typed<N>(N);

impl<N> QueryMatch for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.0.range()
    }
}

impl<N> Queryable for Typed<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = TypedService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
