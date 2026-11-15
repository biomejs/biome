//! Query types that give HTML lint rules the Svelte semantic model of the
//! analyzed `.svelte` file.
//!
//! The model covers the template, the scripts, and how names in the template
//! resolve to declarations, so a rule that needs more than the HTML syntax of
//! the template asks the model instead of reading the embedded expressions.
//!
//! When a rule needs the syntax of an element, an attribute or a block the
//! model describes, [`AnalyzedSvelteModel::node`] returns its node in
//! constant time.
//!
//! The model is built from the file stored in the language database, because
//! the database also holds the parsed scripts and template expressions. It is
//! empty when no stored tree matches the analyzed one: in a file that isn't a
//! `.svelte` file, when the analyzer runs without a database, and in a fix
//! pass, which analyzes a tree the database doesn't hold.

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext, VisitorStartContext,
};
use biome_db::Source;
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_languages::{HtmlFileSource, LanguageDb};
use biome_parser::db::SourceExt;
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_svelte_semantic::{AnalyzedSvelteModel, SvelteSemanticModel, svelte_model_from_source};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct SvelteSemanticServices {
    model: AnalyzedSvelteModel<HtmlLanguage>,
}

impl SvelteSemanticServices {
    /// The Svelte semantic model of the analyzed document, with the nodes of
    /// the document its rows stand for.
    pub fn model(&self) -> &AnalyzedSvelteModel<HtmlLanguage> {
        &self.model
    }
}

impl FromServices for SvelteSemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &AnalyzedSvelteModel<HtmlLanguage> =
            services.get_service().ok_or_else(|| {
                ServicesDiagnostic::new(rule_key.rule_name(), &["AnalyzedSvelteModel"])
            })?;
        Ok(Self {
            model: model.clone(),
        })
    }
}

impl Phase for SvelteSemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type for HTML lint rules that match AST nodes and read the Svelte
/// semantic model through `ctx.model()`.
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

/// Inserts the [`AnalyzedSvelteModel`] of the analyzed file into the service
/// bag, so the model is only built when an enabled rule queries it.
struct SvelteSemanticModelBuilderVisitor;

impl Visitor for SvelteSemanticModelBuilderVisitor {
    type Language = HtmlLanguage;

    fn start(&mut self, ctx: VisitorStartContext<HtmlLanguage>) {
        if ctx
            .services
            .get_service::<AnalyzedSvelteModel<HtmlLanguage>>()
            .is_none()
        {
            let model = model_from_services(ctx.root, ctx.services).unwrap_or_default();
            let model = AnalyzedSvelteModel::<HtmlLanguage>::new(model, ctx.tree());
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

/// Returns the model of the analyzed `.svelte` document.
///
/// The document comes from the services rather than from its path, because
/// each client of the workspace can have its own document for a path.
///
/// Returns `None` when the stored tree isn't `root`, because the ranges of the
/// model would not match the analyzed tree.
fn model_from_services(root: &HtmlRoot, services: &ServiceBag) -> Option<SvelteSemanticModel> {
    if !services.get_service::<HtmlFileSource>()?.is_svelte() {
        return None;
    }
    let db = services.get_service::<Rc<dyn LanguageDb>>()?.as_ref();
    let file = *services.get_service::<Source>()?;
    let stored: HtmlRoot = file.parsed(db).tree();
    if stored.syntax().as_send() != root.syntax().as_send() {
        return None;
    }
    Some(svelte_model_from_source(db, file).clone())
}

#[cfg(test)]
mod tests {
    use super::{SvelteSemanticModelBuilderVisitor, model_from_services};
    use biome_analyze::{
        AnalyzerContext, AnalyzerOptions, AnalyzerSignal, MatchQueryParams, Phases, QueryMatcher,
        ServiceBag, Visitor, VisitorContext, VisitorStartContext,
    };
    use biome_db::Source;
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
    use biome_languages::{DocumentFileSource, HtmlFileSource, LanguageDb};
    use biome_parser::db::{ParseDb, SourceExt, SourceOutput};
    use biome_rowan::{AstNode, WalkEvent};
    use biome_svelte_semantic::AnalyzedSvelteModel;
    use camino::{Utf8Path, Utf8PathBuf};
    use std::cell::RefCell;
    use std::ops::{ControlFlow, Deref};
    use std::rc::Rc;
    use std::sync::OnceLock;

    const SOURCE: &str = "{#if ok}<span class=\"a\" />{/if}\n";

    #[salsa::db]
    #[derive(Default)]
    struct TestDb {
        file: OnceLock<Source>,
        storage: salsa::Storage<Self>,
    }

    #[salsa::db]
    impl salsa::Database for TestDb {}

    #[salsa::db]
    impl biome_db::Db for TestDb {
        fn parsed_source_for_path(&self, path: &Utf8Path) -> Option<Source> {
            self.file
                .get()
                .copied()
                .filter(|file| file.path(self) == path)
        }
    }

    /// Parses every text as a Svelte file.
    #[salsa::db]
    impl ParseDb for TestDb {
        fn parse_source(&self, source: Source) -> SourceOutput {
            let parsed = parse_html(
                source.text(self),
                HtmlParserOptions::from(&HtmlFileSource::svelte()),
            );
            SourceOutput::new(parsed.into(), 0)
        }
    }

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _: usize) -> Option<DocumentFileSource> {
            Some(DocumentFileSource::Html(HtmlFileSource::svelte()))
        }
    }

    /// Stores `SOURCE` at `App.svelte` and returns the service bag the
    /// analyzer would build for that document, with the stored tree.
    fn services(source_type: HtmlFileSource) -> (ServiceBag, HtmlRoot) {
        let db = TestDb::default();
        let file = Source::new(&db, Utf8PathBuf::from("App.svelte"), SOURCE.into(), 0);
        db.file.set(file).unwrap();
        let root: HtmlRoot = file.parsed(&db).tree();
        let mut services = ServiceBag::default();
        services.insert_service(Rc::new(db) as Rc<dyn LanguageDb>);
        services.insert_service(file);
        services.insert_service(source_type);
        (services, root)
    }

    #[test]
    fn builds_the_model_of_the_stored_file() {
        let (services, root) = services(HtmlFileSource::svelte());
        let model = model_from_services(&root, &services).expect("a model");
        let tags: Vec<_> = model
            .elements()
            .map(|element| element.tag().to_string())
            .collect();
        assert_eq!(tags, ["span"]);
        assert_eq!(model.blocks().count(), 1);
    }

    #[test]
    fn no_model_for_a_tree_the_database_does_not_hold() {
        let (services, _) = services(HtmlFileSource::svelte());
        let fixed = parse_html(
            "{#if ok}<span></span>{/if}\n",
            HtmlParserOptions::from(&HtmlFileSource::svelte()),
        );
        assert!(model_from_services(&fixed.tree(), &services).is_none());
    }

    #[test]
    fn no_model_outside_svelte_files() {
        let (services, root) = services(HtmlFileSource::html());
        assert!(model_from_services(&root, &services).is_none());
    }

    /// Reads the node of each block, element and attribute of the model
    /// through [`AnalyzedSvelteModel`], as a rule does.
    struct NodeProbe<'a> {
        nodes: &'a RefCell<Vec<String>>,
    }

    impl Visitor for NodeProbe<'_> {
        type Language = HtmlLanguage;

        fn start(&mut self, ctx: VisitorStartContext<HtmlLanguage>) {
            let model: &AnalyzedSvelteModel<HtmlLanguage> =
                ctx.services.get_service().expect("a model");
            let mut nodes = self.nodes.borrow_mut();
            for block in model.blocks() {
                let node = model.node(&block).expect("a block node");
                assert_eq!(node.range(), block.range());
                nodes.push(node.syntax().text_trimmed().to_string());
            }
            for element in model.elements() {
                let node = model.node(&element).expect("an element node");
                assert_eq!(node.range(), element.range());
                nodes.push(node.syntax().text_trimmed().to_string());
                for attr in element.attrs() {
                    let node = model.node(&attr).expect("an attribute node");
                    nodes.push(node.syntax().text_trimmed().to_string());
                }
            }
        }

        fn visit(&mut self, _: &WalkEvent<HtmlSyntaxNode>, _: VisitorContext<HtmlLanguage>) {}
    }

    struct NoMatcher;

    impl QueryMatcher<HtmlLanguage> for NoMatcher {
        fn match_query(&mut self, _: MatchQueryParams<HtmlLanguage>) {}
    }

    #[test]
    fn rows_resolve_to_nodes_of_the_analyzed_document() {
        let (services, root) = services(HtmlFileSource::svelte());
        let found = RefCell::new(Vec::new());
        let mut emit = |_: &dyn AnalyzerSignal<HtmlLanguage>| ControlFlow::<()>::Continue(());
        let mut analyzer = biome_analyze::Analyzer::new(
            crate::METADATA.deref(),
            NoMatcher,
            Box::new(crate::HtmlSuppression),
            Box::new(crate::suppression_action::HtmlSuppressionAction),
            &mut emit,
        );
        analyzer.add_visitor(Phases::Syntax, Box::new(SvelteSemanticModelBuilderVisitor));
        analyzer.add_visitor(Phases::Semantic, Box::new(NodeProbe { nodes: &found }));
        analyzer.run(AnalyzerContext {
            root,
            services,
            range: None,
            options: &AnalyzerOptions::default(),
        });
        assert_eq!(
            found.into_inner(),
            [SOURCE.trim_end(), r#"<span class="a" />"#, r#"class="a""#].map(String::from)
        );
    }
}
