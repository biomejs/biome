use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch,
    Queryable, RuleKey, ServiceBag, SyntaxVisitor, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_js_semantic::{SemanticEventExtractor, SemanticModel, SemanticModelBuilder};
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
        services: &ServiceBag,
    ) -> Result<Self, MissingServicesDiagnostic> {
        let model: &SemanticModel = services.get_service().ok_or_else(|| {
            MissingServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"])
        })?;
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

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
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

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, root: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

pub struct SemanticModelBuilderVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticModelBuilderVisitor {
    pub(crate) fn new(root: &AnyJsRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, _ctx: VisitorContext<JsLanguage>) {
        match event {
            WalkEvent::Enter(node) => {
                self.builder.push_node(node);
                self.extractor.enter(node);
            }
            WalkEvent::Leave(node) => {
                self.extractor.leave(node);
            }
        }

        while let Some(e) = self.extractor.pop() {
            self.builder.push_event(e);
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<JsLanguage>) {
        let model = self.builder.build();
        ctx.services.insert_service(model);
    }
}

pub struct SemanticModelVisitor;

pub struct SemanticModelEvent(TextRange);

impl QueryMatch for SemanticModelEvent {
    fn text_range(&self) -> TextRange {
        self.0
    }
}

impl Visitor for SemanticModelVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }

                node
            }
            WalkEvent::Leave(_) => return,
        };

        let text_range = root.text_range_with_trivia();
        ctx.match_query(SemanticModelEvent(text_range));
    }
}
