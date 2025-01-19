use biome_analyze::{
    AddVisitor, FromServices, MissingServicesDiagnostic, Phase, Phases, QueryKey, QueryMatch,
    Queryable, RuleKey, ServiceBag, SyntaxVisitor, Visitor, VisitorContext, VisitorFinishContext,
};
use biome_css_semantic::builder::SemanticModelBuilder;
use biome_css_semantic::{model::SemanticModel, SemanticEventExtractor};
use biome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
use biome_rowan::{AstNode, TextRange, WalkEvent};

/// The [SemanticServices] types can be used as a queryable to get an instance
/// of the whole [SemanticModel] without matching on a specific AST node
///
/// ```ignore
/// impl Rule for SampleCssLintRule {
///    type Query = SemanticServices;
///    type State = ();
///    type Signals = Option<Self::State>;
///    type Options = ();
///    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
///     let node = ctx.query();
///     for n in node.rules() {
///       // Do something with the rules
///     }
///     //.....//
///    }
/// }
/// ```
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

impl Queryable for SemanticServices {
    type Input = SemanticModelEvent;
    type Output = SemanticModel;

    type Language = CssLanguage;
    type Services = Self;

    fn build_visitor(
        analyzer: &mut impl biome_analyze::AddVisitor<Self::Language>,
        root: &<Self::Language as biome_rowan::Language>::Root,
    ) {
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

pub struct SemanticModelBuilderVisitor {
    extractor: SemanticEventExtractor,
    builder: SemanticModelBuilder,
}

impl SemanticModelBuilderVisitor {
    pub(crate) fn new(root: &CssRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
    type Language = CssLanguage;

    fn visit(&mut self, event: &WalkEvent<CssSyntaxNode>, _ctx: VisitorContext<CssLanguage>) {
        match event {
            WalkEvent::Enter(node) => {
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

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<CssLanguage>) {
        let model = self.builder.build();
        ctx.services.insert_service(model);
    }
}

pub struct SemanticModelVisitor;

impl Visitor for SemanticModelVisitor {
    type Language = CssLanguage;

    fn visit(&mut self, event: &WalkEvent<CssSyntaxNode>, mut ctx: VisitorContext<CssLanguage>) {
        let root = match event {
            WalkEvent::Enter(node) => {
                if node.parent().is_some() {
                    return;
                }
                node.clone()
            }
            WalkEvent::Leave(_) => return,
        };

        let text_range = root.text_range_with_trivia();
        ctx.match_query(SemanticModelEvent(text_range));
    }
}

pub struct SemanticModelEvent(TextRange);

impl QueryMatch for SemanticModelEvent {
    fn text_range(&self) -> TextRange {
        self.0
    }
}

/// The [Semantic] type usable by lint rules **that uses the semantic model** to match on specific [AstNode] types
///
/// ```ignore
/// impl Rule for SampleCssLintRule {
///    type Query = Semantic<CssGenericProperty>;
///    type State = ();
///    type Signals = Option<Self::State>;
///    type Options = ();
///    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
///     let node = ctx.query();
///     // The model holds all information about the semantic.
///     let model = ctx.model();
///     for n in model.rules() {
///       // Do something with the rules
///     }
///     //.....//
///    }
/// }
/// ```
#[derive(Clone)]
pub struct Semantic<N>(pub N);

impl<N> Queryable for Semantic<N>
where
    N: AstNode<Language = CssLanguage> + 'static,
{
    type Input = CssSyntaxNode;
    type Output = N;

    type Language = CssLanguage;
    type Services = SemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<CssLanguage>, root: &CssRoot) {
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
