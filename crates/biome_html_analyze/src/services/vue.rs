//! Query types that give HTML lint rules the Vue semantic model of the
//! analyzed `.vue` file.
//!
//! The model covers the template, the script blocks, and how names in the
//! template resolve to declarations, so a rule that needs more than the HTML
//! syntax of the template asks the model instead of parsing attribute values.
//!
//! The model is built from the file stored in the language database, because
//! the database also holds the parsed scripts and template expressions. It is
//! empty when no stored tree matches the analyzed one: in a file that isn't a
//! `.vue` file, when the analyzer runs without a database, and in a fix pass,
//! which analyzes a tree the database doesn't hold.

use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, Queryable, RuleKey, RuleMetadata,
    ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext, VisitorStartContext,
};
use biome_db::Source;
use biome_html_syntax::{HtmlLanguage, HtmlRoot, HtmlSyntaxNode};
use biome_languages::{HtmlFileSource, LanguageDb};
use biome_parser::db::SourceExt;
use biome_rowan::{AstNode, SyntaxKindSet, WalkEvent};
use biome_vue_semantic::{VueSemanticModel, vue_model_from_source};
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct VueSemanticServices {
    model: VueSemanticModel,
}

impl VueSemanticServices {
    pub fn model(&self) -> &VueSemanticModel {
        &self.model
    }
}

impl FromServices for VueSemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &VueSemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["VueSemanticModel"]))?;
        Ok(Self {
            model: model.clone(),
        })
    }
}

impl Phase for VueSemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Query type for HTML lint rules that match AST nodes and read the Vue
/// semantic model through `ctx.model()`.
#[derive(Clone)]
pub struct VueSemantic<N>(pub N);

impl<N> Queryable for VueSemantic<N>
where
    N: AstNode<Language = HtmlLanguage> + 'static,
{
    type Input = HtmlSyntaxNode;
    type Output = N;
    type Language = HtmlLanguage;
    type Services = VueSemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<HtmlLanguage>, _: &HtmlRoot) {
        analyzer.add_visitor(Phases::Syntax, || VueSemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

/// Inserts the [`VueSemanticModel`] of the analyzed file into the service
/// bag, so the model is only built when an enabled rule queries it.
struct VueSemanticModelBuilderVisitor;

impl Visitor for VueSemanticModelBuilderVisitor {
    type Language = HtmlLanguage;

    fn start(&mut self, ctx: VisitorStartContext<HtmlLanguage>) {
        if ctx.services.get_service::<VueSemanticModel>().is_none() {
            let model = model_from_services(ctx.root, ctx.services).unwrap_or_default();
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

/// Returns the model of the analyzed `.vue` document.
///
/// The document comes from the services rather than from its path, because
/// each client of the workspace can have its own document for a path.
///
/// Returns `None` when the stored tree isn't `root`, because the ranges of the
/// model would not match the analyzed tree.
fn model_from_services(root: &HtmlRoot, services: &ServiceBag) -> Option<VueSemanticModel> {
    if !services.get_service::<HtmlFileSource>()?.is_vue() {
        return None;
    }
    let db = services.get_service::<Rc<dyn LanguageDb>>()?.as_ref();
    let file = *services.get_service::<Source>()?;
    let stored: HtmlRoot = file.parsed(db).tree();
    if stored.syntax().as_send() != root.syntax().as_send() {
        return None;
    }
    Some(vue_model_from_source(db, file).clone())
}

#[cfg(test)]
mod tests {
    use super::model_from_services;
    use biome_analyze::ServiceBag;
    use biome_db::Source;
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_html_syntax::HtmlRoot;
    use biome_languages::{DocumentFileSource, HtmlFileSource, LanguageDb};
    use biome_parser::db::{ParseDb, SourceExt, SourceOutput};
    use camino::{Utf8Path, Utf8PathBuf};
    use std::rc::Rc;
    use std::sync::OnceLock;

    const SOURCE: &str = "<template><div v-if=\"ok\"><span /></div></template>\n";

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

    /// Parses every text as a Vue file.
    #[salsa::db]
    impl ParseDb for TestDb {
        fn parse_source(&self, source: Source) -> SourceOutput {
            let parsed = parse_html(source.text(self), HtmlParserOptions::default().with_vue());
            SourceOutput::new(parsed.into(), 0)
        }
    }

    #[salsa::db]
    impl LanguageDb for TestDb {
        fn source_from_index(&self, _: usize) -> Option<DocumentFileSource> {
            Some(DocumentFileSource::Html(HtmlFileSource::vue()))
        }
    }

    /// Stores `SOURCE` at `App.vue` and returns the service bag the analyzer
    /// would build for that document, with the stored tree.
    fn services(source_type: HtmlFileSource) -> (ServiceBag, HtmlRoot) {
        let db = TestDb::default();
        let file = Source::new(&db, Utf8PathBuf::from("App.vue"), SOURCE.into(), 0);
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
        let (services, root) = services(HtmlFileSource::vue());
        let model = model_from_services(&root, &services).expect("a model");
        let tags: Vec<_> = model
            .elements()
            .map(|element| element.tag().to_string())
            .collect();
        assert_eq!(tags, ["template", "div", "span"]);
    }

    #[test]
    fn no_model_for_a_tree_the_database_does_not_hold() {
        let (services, _) = services(HtmlFileSource::vue());
        let fixed = parse_html(
            "<template><div v-if=\"ok\"><span></span></div></template>\n",
            HtmlParserOptions::default().with_vue(),
        );
        assert!(model_from_services(&fixed.tree(), &services).is_none());
    }

    #[test]
    fn no_model_outside_vue_files() {
        let (services, root) = services(HtmlFileSource::html());
        assert!(model_from_services(&root, &services).is_none());
    }
}
