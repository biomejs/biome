use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorFinishContext,
};
use biome_css_semantic::SemanticEventExtractor;
use biome_css_semantic::builder::SemanticModelBuilder;
use biome_css_semantic::model::SemanticModel;
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssSyntaxNode, TextRange};
use biome_rowan::{AstNode, WalkEvent};

/// ## Warning
///
/// Using this type as a [biome_analyze::Rule] `Query` is discouraged, because it enforces the inspections of an entire
/// document, even when the document doesn't contain the nodes that needs to be inspected.
///
/// Prefer the use of `Semantic<Node>` to trigger the rule only for those nodes that might trigger the rule.
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

    type Language = CssLanguage;
    type Services = Self;

    fn build_visitor(analyzer: &mut impl AddVisitor<Self::Language>, root: &AnyCssRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor::new(root));
        analyzer.add_visitor(Phases::Semantic, || SemanticModelVisitor);
    }

    fn unwrap_match(services: &ServiceBag, _: &SemanticModelEvent) -> Self::Output {
        // SAFETY: `build_visitor` registers the model builder before the semantic visitor emits
        // `SemanticModelEvent`, and the builder inserts the service when syntax traversal finishes.
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
    pub(crate) fn new(root: &AnyCssRoot) -> Self {
        Self {
            extractor: SemanticEventExtractor::default(),
            builder: SemanticModelBuilder::new(root.clone()),
        }
    }
}

impl Visitor for SemanticModelBuilderVisitor {
    type Language = CssLanguage;

    fn visit(&mut self, event: &WalkEvent<CssSyntaxNode>, _ctx: VisitorContext<Self::Language>) {
        match event {
            WalkEvent::Enter(node) => self.extractor.enter(node),
            WalkEvent::Leave(node) => self.extractor.leave(node),
        }

        while let Some(event) = self.extractor.pop() {
            self.builder.push_event(event);
        }
    }

    fn finish(self: Box<Self>, ctx: VisitorFinishContext<Self::Language>) {
        // If a pre-built SemanticModel was already inserted (e.g. by the workspace
        // open_file/change_file cycle), skip building a new one.
        if ctx.services.get_service::<SemanticModel>().is_some() {
            return;
        }
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
    type Language = CssLanguage;

    fn visit(&mut self, event: &WalkEvent<CssSyntaxNode>, mut ctx: VisitorContext<Self::Language>) {
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

    fn build_visitor(analyzer: &mut impl AddVisitor<CssLanguage>, root: &AnyCssRoot) {
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
