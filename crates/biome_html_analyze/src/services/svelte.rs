//! Query types that give HTML lint rules the Svelte semantic model of the
//! analyzed `.svelte` file.

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext, VisitorStartContext,
};
use biome_db::Source;
use biome_embeds::svelte_model_from_source;
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_languages::{HtmlFileSource, LanguageDb};
use biome_parser::db::SourceExt;
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_svelte_semantic::SvelteSemanticModel;
use std::rc::Rc;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct SvelteSemanticServices {
    model: Arc<SvelteSemanticModel>,
}

impl SvelteSemanticServices {
    /// Returns the semantic model of the analyzed Svelte component.
    pub fn model(&self) -> &SvelteSemanticModel {
        &self.model
    }
}

impl FromServices for SvelteSemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &Arc<SvelteSemanticModel> = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["SvelteSemanticModel"])
        })?;
        Ok(Self {
            model: Arc::clone(model),
        })
    }
}

impl Phase for SvelteSemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Matches an HTML node and provides the Svelte semantic model through
/// `RuleContext::model`.
#[derive(Clone)]
pub struct SvelteSemantic<N>(pub N);

impl<N> Queryable for SvelteSemantic<N>
where
    N: AstNode<Language = HtmlLanguage> + 'static,
{
    type Input = HtmlSyntaxNode;
    type Output = N;
    type Language = HtmlLanguage;
    type Services = SvelteSemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<HtmlLanguage>, _: &HtmlRoot) {
        analyzer.add_visitor(Phases::Syntax, || SvelteSemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

struct SvelteSemanticModelBuilderVisitor;

impl Visitor for SvelteSemanticModelBuilderVisitor {
    type Language = HtmlLanguage;

    fn start(&mut self, ctx: VisitorStartContext<HtmlLanguage>) {
        if ctx
            .services
            .get_service::<Arc<SvelteSemanticModel>>()
            .is_none()
            && let Some(model) = model_from_services(ctx.root, ctx.services)
        {
            ctx.services.insert_service(model);
        }
    }

    fn visit(&mut self, _: &WalkEvent<HtmlSyntaxNode>, _: VisitorContext<HtmlLanguage>) {}

    fn visited_kinds(
        &self,
        _: Option<SyntaxKindSet<HtmlLanguage>>,
    ) -> Option<SyntaxKindSet<HtmlLanguage>> {
        Some(SyntaxKindSet::EMPTY)
    }
}

fn model_from_services(root: &HtmlRoot, services: &ServiceBag) -> Option<Arc<SvelteSemanticModel>> {
    if !services.get_service::<HtmlFileSource>()?.is_svelte() {
        return None;
    }
    let db = services.get_service::<Rc<dyn LanguageDb>>()?.as_ref();
    let file = *services.get_service::<Source>()?;
    let stored: HtmlRoot = file.parsed(db).tree();
    if stored.syntax().as_send() != root.syntax().as_send() {
        return None;
    }
    svelte_model_from_source(db, file).clone()
}
