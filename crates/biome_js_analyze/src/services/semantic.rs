use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, TextRange, WalkEvent};
use biome_rowan::AstNode;

pub struct SemanticServices {
    model: SemanticModel,
}

impl SemanticServices {
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }
}

impl FromServices for SemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,

        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;
        Ok(Self {
            model: model.clone(),
        })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// The [SemanticServices] types can be used as a queryable to get an instance
/// of the whole [SemanticModel] without matching on a specific AST node
impl Queryable for SemanticServices {
    type Input = SemanticModelEvent;
    type Output = SemanticModel;

    type Language = JsLanguage;
    type Services = Self;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Semantic, || SemanticModelVisitor);
    }

    fn unwrap_match(services: &ServiceBag, _: &SemanticModelEvent) -> Self::Output {
        services
            .get_service::<SemanticModel>()
            .expect("SemanticModel service is not registered")
            .clone()
    }
}

/// Query type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
#[derive(Clone)]
pub struct Semantic<N>(pub N);

impl<N> Queryable for Semantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;

    type Language = JsLanguage;
    type Services = SemanticServices;

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
pub struct SemanticModelVisitor;

pub struct SemanticModelEvent(AnyJsRoot);

impl QueryMatch for SemanticModelEvent {
    fn text_range(&self) -> TextRange {
        self.0.syntax().text_range_with_trivia()
    }
}

impl Visitor for SemanticModelVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let root = match event {
            WalkEvent::Enter(node) => {
                let Some(node) = AnyJsRoot::cast_ref(node) else {
                    return;
                };

                node
            }
            WalkEvent::Leave(_) => return,
        };

        ctx.match_query(SemanticModelEvent(root.clone()));
    }
}
