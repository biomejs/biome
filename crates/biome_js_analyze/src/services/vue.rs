//! Query types that hand a lint rule the Vue components of a file, or their
//! declarations, as described by the Vue semantic model.
//!
//! A rule picks the unit it judges as its `Query`:
//!
//! - [`VueComponent`] runs the rule once per component. Use it when the rule
//!   compares declarations with each other.
//! - [`VueProp`] runs the rule once per prop. Use it when the rule judges
//!   one prop on its own.
//!
//! The model already folds the different ways Vue lets a declaration be
//! written into one shape, so a rule never matches on syntax to find them.
//!
//! A query for another kind of declaration is one `define_vue_query!` call
//! and one loop in [`VueSemanticVisitor`].

use crate::services::semantic::{SemanticModelBuilderVisitor, SemanticServices};
use biome_analyze::{
    AddVisitor, Phases, QueryMatch, Queryable, ServiceBag, Visitor, VisitorContext,
    VisitorStartContext,
};
use biome_db::AnyParsedSource;
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, TextRange};
use biome_languages::{JsFileSource, LanguageDb};
use biome_rowan::{AstNode, WalkEvent};
use biome_vue_semantic::{
    Component, Prop, SemanticModel as VueSemanticModel, SnippetId, Symbol, component_model,
    vue_model_from_source,
};
use camino::Utf8PathBuf;
use std::rc::Rc;

/// The Vue semantic model of the analyzed file, and which of its embedded
/// snippets the analyzed tree is.
#[derive(Clone, Debug)]
pub struct VueSemanticService {
    model: VueSemanticModel,
    /// `None` when the analyzed tree is a whole JavaScript file.
    snippet: Option<SnippetId>,
}

impl VueSemanticService {
    /// Builds the service for the tree being analyzed.
    ///
    /// The model of the stored file is used when the analyzed tree is the
    /// stored one. A fix pass analyzes a tree the database does not hold; its
    /// ranges would not match a stored model, so the model is then built from
    /// the tree alone and describes its components without their template.
    fn from_services(root: &AnyJsRoot, services: &ServiceBag) -> Self {
        let stored = services
            .get_service::<Rc<dyn LanguageDb>>()
            .zip(services.get_service::<AnyParsedSource>())
            .and_then(|(db, source)| {
                let db = db.as_ref();
                let stored_root: AnyJsRoot = source.tree(db);
                if stored_root.syntax().as_send() != root.syntax().as_send() {
                    return None;
                }
                match source {
                    AnyParsedSource::ParsedSource(file) => Some(Self {
                        model: vue_model_from_source(db, *file).clone(),
                        snippet: None,
                    }),
                    AnyParsedSource::ParsedSnippet(snippet) => {
                        let path = services.get_service::<Utf8PathBuf>()?;
                        let host = db.parsed_source_for_path(path)?;
                        let model = vue_model_from_source(db, host).clone();
                        let snippet = model.snippet_at_offset(snippet.content_offset(db))?.id();
                        Some(Self {
                            model,
                            snippet: Some(snippet),
                        })
                    }
                }
            });
        stored.unwrap_or_else(|| {
            let source_type = services
                .get_service::<JsFileSource>()
                .copied()
                .unwrap_or_default();
            let model = match services.get_service::<SemanticModel>() {
                Some(js) => component_model(root, js, source_type),
                None => VueSemanticModel::default(),
            };
            Self {
                model,
                snippet: None,
            }
        })
    }

    /// Returns `true` when `symbol` is declared in the analyzed tree.
    fn declares(&self, symbol: &Symbol) -> bool {
        symbol.snippet().map(|snippet| snippet.id()) == self.snippet
    }
}

/// Builds the Vue semantic model and emits a query match for each component
/// and each declaration written in the analyzed tree.
pub(crate) struct VueSemanticVisitor;

impl Visitor for VueSemanticVisitor {
    type Language = JsLanguage;

    fn start(&mut self, ctx: VisitorStartContext<JsLanguage>) {
        if ctx.services.get_service::<VueSemanticService>().is_none() {
            let service = VueSemanticService::from_services(ctx.root, ctx.services);
            ctx.services.insert_service(service);
        }
    }

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let WalkEvent::Enter(node) = event else {
            return;
        };
        if node.parent().is_some() {
            return;
        }
        let Some(service) = ctx.services.get_service::<VueSemanticService>() else {
            return;
        };
        for component in service.model.components() {
            if component.snippet().map(|snippet| snippet.id()) == service.snippet {
                ctx.match_query(VueComponentMatch(component.clone()));
            }
            for prop in component.props().filter(|prop| service.declares(prop)) {
                ctx.match_query(VuePropMatch(prop));
            }
        }
    }
}

macro_rules! define_vue_query {
    ($(#[$meta:meta])* $query:ident, $input:ident, $output:ty) => {
        $(#[$meta])*
        pub struct $query;

        pub struct $input($output);

        impl QueryMatch for $input {
            fn text_range(&self) -> TextRange {
                self.0.range()
            }
        }

        impl Queryable for $query {
            type Input = $input;
            type Output = $output;
            type Language = JsLanguage;
            type Services = SemanticServices;

            fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
                analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
                analyzer.add_visitor(Phases::Semantic, || VueSemanticVisitor);
            }

            fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
                query.0.clone()
            }
        }
    };
}

define_vue_query!(
    /// Matches each Vue component the analyzed tree defines.
    VueComponent,
    VueComponentMatch,
    Component
);
define_vue_query!(
    /// Matches each prop of each Vue component, whatever syntax declares it.
    VueProp,
    VuePropMatch,
    Prop
);
