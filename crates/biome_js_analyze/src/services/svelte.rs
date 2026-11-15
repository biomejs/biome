//! A query type that hands a lint rule the Svelte semantic model of the file
//! the analyzed tree belongs to.
//!
//! [`SvelteSemantic`] runs the rule once per syntax node, like
//! [`Semantic`](crate::services::semantic::Semantic), and gives it both the
//! JavaScript semantic model of the analyzed tree and the Svelte semantic
//! model of its file. For a script or a template expression of a component,
//! that model covers the whole component, so the rule can see how the
//! template uses a variable the script declares. For a `.svelte.js` module it
//! covers the module.
//!
//! When a rule needs the syntax of a row of the model, to build a fix or to
//! read what the model does not describe, [`AnalyzedSvelteModel::node`]
//! returns the node in constant time.

use crate::services::semantic::SemanticModelBuilderVisitor;
use biome_analyze::{
    AddVisitor, AnalyzedTree, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorStartContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode};
use biome_languages::{JsFileSource, LanguageDb};
use biome_parser::db::AnalyzedSource;
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_svelte_semantic::{
    AnalyzedSvelteModel, Snippet, SnippetId, SvelteSemanticModel, module_model,
    svelte_model_from_source,
};
use camino::Utf8PathBuf;
use std::rc::Rc;

/// The Svelte semantic model of the analyzed file, and which of its embedded
/// snippets the analyzed tree is.
#[derive(Clone, Debug)]
pub struct SvelteSemanticService {
    model: AnalyzedSvelteModel<JsLanguage>,
    /// `None` when the analyzed tree is a whole module.
    snippet: Option<SnippetId>,
}

impl SvelteSemanticService {
    /// Builds the service for the tree being analyzed.
    ///
    /// The model of the stored file is used when the analyzed tree is the
    /// stored one. A fix pass analyzes a tree the database does not hold; its
    /// ranges would not match a stored model, so the model is then built from
    /// the tree alone for a module, and is empty for a snippet of a
    /// component.
    fn from_services(tree: AnalyzedTree<'_, JsLanguage>, services: &ServiceBag) -> Self {
        let root = tree.root();
        let stored = services
            .get_service::<Rc<dyn LanguageDb>>()
            .zip(services.get_service::<AnalyzedSource>())
            .and_then(|(db, source)| {
                let db = db.as_ref();
                let stored_root: AnyJsRoot = source.tree(db);
                if stored_root.syntax().as_send() != root.syntax().as_send() {
                    return None;
                }
                match source {
                    AnalyzedSource::Source(file) => Some(Self::new(
                        svelte_model_from_source(db, *file).clone(),
                        None,
                        tree,
                    )),
                    AnalyzedSource::Snippet(snippet) => {
                        // Use the host document of the snippet: the document
                        // of its path can be the one of another client.
                        let model = svelte_model_from_source(db, snippet.source()).clone();
                        let snippet = model.snippet_at_offset(snippet.content_offset())?.id();
                        Some(Self::new(model, Some(snippet), tree))
                    }
                }
            });
        stored.unwrap_or_else(|| {
            let is_module = services
                .get_service::<JsFileSource>()
                .is_some_and(JsFileSource::is_svelte_source_module);
            let model = match (is_module, services.get_service::<SemanticModel>()) {
                (true, Some(js)) => {
                    let path = services
                        .get_service::<Utf8PathBuf>()
                        .cloned()
                        .unwrap_or_default();
                    module_model(root, js, &path)
                }
                _ => SvelteSemanticModel::default(),
            };
            Self::new(model, None, tree)
        })
    }

    fn new(
        model: SvelteSemanticModel,
        snippet: Option<SnippetId>,
        tree: AnalyzedTree<'_, JsLanguage>,
    ) -> Self {
        Self {
            model: AnalyzedSvelteModel::<JsLanguage>::new(model, tree, snippet),
            snippet,
        }
    }
}

/// Inserts the [`SvelteSemanticService`] of the analyzed tree into the
/// service bag, so the model is only built when an enabled rule queries it.
struct SvelteSemanticVisitor;

impl Visitor for SvelteSemanticVisitor {
    type Language = JsLanguage;

    fn start(&mut self, ctx: VisitorStartContext<JsLanguage>) {
        if ctx
            .services
            .get_service::<SvelteSemanticService>()
            .is_none()
        {
            let service = SvelteSemanticService::from_services(ctx.tree(), ctx.services);
            ctx.services.insert_service(service);
        }
    }

    fn visit(&mut self, _: &WalkEvent<JsSyntaxNode>, _: VisitorContext<JsLanguage>) {}

    fn visited_kinds(
        &self,
        _: Option<SyntaxKindSet<JsLanguage>>,
    ) -> Option<SyntaxKindSet<JsLanguage>> {
        Some(SyntaxKindSet::EMPTY)
    }
}

/// The services of a rule that queries [`SvelteSemantic`]: the JavaScript
/// semantic model of the analyzed tree, and the Svelte semantic model of the
/// file it belongs to.
pub struct SvelteSemanticServices {
    model: SemanticModel,
    svelte: SvelteSemanticService,
}

impl SvelteSemanticServices {
    /// The JavaScript semantic model of the analyzed tree.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }

    /// The Svelte semantic model of the file the analyzed tree belongs to,
    /// with the nodes of the analyzed tree its rows stand for.
    pub fn svelte_model(&self) -> &AnalyzedSvelteModel<JsLanguage> {
        &self.svelte.model
    }

    /// The snippet of a component that the analyzed tree is.
    ///
    /// Returns `None` when the analyzed tree is a whole module, or when the
    /// model of its file is not available.
    pub fn snippet(&self) -> Option<Snippet> {
        self.svelte.snippet.map(|id| self.svelte.model.snippet(id))
    }
}

impl FromServices for SvelteSemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;
        let svelte: &SvelteSemanticService = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["SvelteSemanticService"])
        })?;
        Ok(Self {
            model: model.clone(),
            svelte: svelte.clone(),
        })
    }
}

impl Phase for SvelteSemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Matches each node of type `N`, and gives the rule both the JavaScript and
/// the Svelte semantic models.
#[derive(Clone)]
pub struct SvelteSemantic<N>(pub N);

impl<N> Queryable for SvelteSemantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;
    type Language = JsLanguage;
    type Services = SvelteSemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, || SvelteSemanticVisitor);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}
