use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor,
};
use biome_css_semantic::model::SemanticModel;
use biome_css_syntax::{AnyCssRoot, CssLanguage, CssSyntaxNode};
use biome_rowan::AstNode;
use std::sync::Arc;

pub struct SemanticServices {
    model: Arc<SemanticModel>,
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
        let model: &Arc<SemanticModel> = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;

        Ok(Self {
            model: model.clone(),
        })
    }
}

impl Phase for SemanticServices {
    fn phase() -> Phases {
        Phases::Syntax
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

    fn build_visitor(analyzer: &mut impl AddVisitor<CssLanguage>, _root: &AnyCssRoot) {
        analyzer.add_visitor(Phases::Syntax, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
