use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
};
use biome_css_semantic::model::SemanticModel;
use biome_css_semantic::{db::css_semantic_model, semantic_model};
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssSyntaxNode, TextRange};
use biome_db::AnyParsedSource;
use biome_languages::LanguageDb;
use biome_rowan::{AstNode, WalkEvent};
use camino::Utf8Path;

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

    fn build_visitor(analyzer: &mut impl AddVisitor<Self::Language>, _: &AnyCssRoot) {
        analyzer.add_visitor(Phases::Semantic, || SemanticModelVisitor);
    }

    fn unwrap_match(services: &ServiceBag, _: &SemanticModelEvent) -> Self::Output {
        services
            .get_service::<SemanticModel>()
            .expect("SemanticModel service is not registered")
            .clone()
    }
}

pub(crate) fn model_for_root(
    db: Option<&dyn LanguageDb>,
    root: &AnyCssRoot,
    path: &Utf8Path,
) -> SemanticModel {
    if let Some(db) = db
        && let Some(syntax) = root.syntax().as_send()
        && let Some(source) = db.parsed_source_for_path(path)
    {
        let source = if source.parsed(db).as_send_node().as_ref() == Some(&syntax) {
            Some(AnyParsedSource::from(source))
        } else {
            source
                .snippets(db)
                .iter()
                .find(|snippet| {
                    let parsed = snippet.parsed(db);
                    parsed.as_send_node().as_ref() == Some(&syntax)
                        || (parsed.is_embedded_node_parse()
                            && db
                                .source_from_index(snippet.document_source_index(db))
                                .and_then(|source| source.to_css_file_source())
                                .is_some()
                            && parsed.tree::<AnyCssRoot>().syntax().as_send().as_ref()
                                == Some(&syntax))
                })
                .map(AnyParsedSource::from)
        };
        if let Some(source) = source {
            let model = css_semantic_model(db, &source);
            // Semantic equality ignores trivia and locations; diagnostics must
            // still use nodes from the syntax tree being analyzed.
            if model.root().syntax().as_send().as_ref() == Some(&syntax) {
                return model.clone();
            }
        }
    }
    semantic_model(root)
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

    fn build_visitor(analyzer: &mut impl AddVisitor<CssLanguage>, _: &AnyCssRoot) {
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
