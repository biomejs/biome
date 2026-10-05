use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorStartContext,
};
use biome_db::AnyParsedSource;
use biome_js_semantic::{SemanticModel, SemanticModelOptions, js_semantic_model, semantic_model};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, TextRange, WalkEvent};
use biome_languages::{JsFileSource, LanguageDb};
use biome_rowan::AstNode;
use std::rc::Rc;

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

    type Language = JsLanguage;
    type Services = Self;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
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

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

pub(crate) struct SemanticModelBuilderVisitor;

impl Visitor for SemanticModelBuilderVisitor {
    type Language = JsLanguage;

    fn start(&mut self, ctx: VisitorStartContext<JsLanguage>) {
        if ctx.services.get_service::<SemanticModel>().is_some() {
            return;
        }
        let source_type = ctx
            .services
            .get_service::<JsFileSource>()
            .copied()
            .unwrap_or_default();
        if let Some(db) = ctx.services.get_service::<Rc<dyn LanguageDb>>()
            && let Some(source) = ctx.services.get_service::<AnyParsedSource>()
        {
            let model = js_semantic_model(db.as_ref(), source);
            // Semantic equality excludes locations, but analyzer consumers need the
            // current syntax and the embedding flavor of the analyzed source.
            if model.root().syntax().as_send() == ctx.root.syntax().as_send()
                && model.flavor() == (&source_type).into()
            {
                ctx.services.insert_service(model.clone());
                return;
            }
        }
        ctx.services.insert_service(semantic_model(
            ctx.root,
            SemanticModelOptions::from(&source_type),
        ));
    }

    fn visit(&mut self, _: &WalkEvent<JsSyntaxNode>, _: VisitorContext<JsLanguage>) {}
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

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<Self::Language>) {
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
