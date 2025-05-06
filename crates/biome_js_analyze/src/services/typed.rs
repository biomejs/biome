use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleDomain, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorFinishContext,
};
use biome_js_syntax::{
    AnyJsExpression, AnyJsRoot, JsExpressionStatement, JsLanguage, JsSyntaxNode,
};
use biome_js_type_info::Type;
use biome_module_graph::{ModuleGraph, ScopedResolver};
use biome_rowan::{AstNode, TextRange, WalkEvent};
use camino::Utf8PathBuf;
use std::sync::Arc;

/// Service for use with type inference rules.
#[derive(Clone, Debug)]
pub struct TypedService {
    resolver: Option<Arc<ScopedResolver>>,
}

impl TypedService {
    pub fn type_for_expression(&self, expr: &AnyJsExpression) -> Type {
        self.resolver
            .as_ref()
            .map(|resolver| resolver.resolved_type_for_expression(expr))
            .unwrap_or_default()
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

        let resolver: Option<&Option<Arc<ScopedResolver>>> = services.get_service();
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
        analyzer.add_visitor(Phases::Syntax, ScopedResolverBuilderVisitor::default);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

#[derive(Default)]
struct ScopedResolverBuilderVisitor {
    resolver: Option<Option<ScopedResolver>>,
}

impl Visitor for ScopedResolverBuilderVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, ctx: VisitorContext<JsLanguage>) {
        let resolver = self.resolver.get_or_insert_with(|| {
            let file_path: &Arc<Utf8PathBuf> = ctx.services.get_service()?;
            let module_graph: &Arc<ModuleGraph> = ctx.services.get_service()?;
            module_graph
                .module_info_for_path(file_path.as_ref())
                .map(|module_info| {
                    ScopedResolver::from_global_scope(module_info, module_graph.clone())
                })
        });

        let Some(resolver) = resolver else {
            return;
        };

        match event {
            WalkEvent::Enter(node) => {
                if let Some(expr) =
                    JsExpressionStatement::cast_ref(node).and_then(|node| node.expression().ok())
                {
                    resolver.register_types_for_expression(&expr);
                }
            }
            WalkEvent::Leave(_node) => {}
        }
    }

    fn finish(mut self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let resolver = self.resolver.take().flatten().map(Arc::new);
        ctx.services.insert_service(resolver);
    }
}
