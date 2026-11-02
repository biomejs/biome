//! Query types that hand a lint rule the Vue components of a file, or their
//! declarations, as described by the Vue semantic model.
//!
//! A rule picks the unit it judges as its `Query`:
//!
//! - [`VueComponent`] runs the rule once per component. Use it when the rule
//!   compares declarations with each other.
//! - [`VueProp`] runs the rule once per prop. Use it when the rule judges
//!   one prop on its own.
//! - [`VueEmit`] runs the rule once per declared event. Use it when the rule
//!   judges one event declaration on its own.
//! - [`VueSnippet`] runs the rule once on an embedded snippet, such as a
//!   directive value. Use it when the rule judges a template expression
//!   by where it is written.
//! - [`VueReference`] runs the rule once per use of a name. Use it when the
//!   rule judges what a use does, wherever it is written: the diagnostic can
//!   then point at a template expression.
//! - [`VueSemantic`] runs the rule once per syntax node, like
//!   [`Semantic`](crate::services::semantic::Semantic). Use it when the rule
//!   judges syntax, such as a template expression, with the help of the
//!   model.
//! - [`VueEvent`] runs the rule once per event name written in an emit
//!   declaration or an emit call. Use it when the rule judges how one event
//!   is declared or emitted.
//!
//! The model already folds the different ways Vue lets a declaration be
//! written into one shape, so a rule never matches on syntax to find them.
//!
//! A query for another kind of declaration is one `define_vue_query!` call
//! and one loop in [`VueSemanticVisitor`]. References have their own visitors,
//! [`VueReferenceVisitor`] and [`VueEventVisitor`], so that files analyzed
//! without a rule that asks for them don't pay for matching every reference.

use crate::services::semantic::{SemanticModelBuilderVisitor, SemanticServices};
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable, RuleKey,
    RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorStartContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxNode, TextRange};
use biome_languages::{JsFileSource, LanguageDb};
use biome_parser::db::AnalyzedSource;
use biome_rowan::{AstNode, WalkEvent};
use biome_vue_semantic::{
    Component, Emit, Prop, RefSite, Reference, Snippet, SnippetId, Symbol, VueSemanticModel,
    component_model, vue_model_from_source,
};
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
            .zip(services.get_service::<AnalyzedSource>())
            .and_then(|(db, source)| {
                let db = db.as_ref();
                let stored_root: AnyJsRoot = source.tree(db);
                if stored_root.syntax().as_send() != root.syntax().as_send() {
                    return None;
                }
                match source {
                    AnalyzedSource::Source(file) => Some(Self {
                        model: vue_model_from_source(db, *file).clone(),
                        snippet: None,
                    }),
                    AnalyzedSource::Snippet(snippet) => {
                        // Use the host document of the snippet: the document
                        // of its path can be the one of another client.
                        let model = vue_model_from_source(db, snippet.source()).clone();
                        let snippet = model.snippet_at_offset(snippet.content_offset())?.id();
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

    /// The model of the whole file the analyzed tree belongs to.
    pub(crate) fn model(&self) -> &VueSemanticModel {
        &self.model
    }

    /// Returns `true` when `symbol` is declared in the analyzed tree.
    fn declares(&self, symbol: &Symbol) -> bool {
        symbol.snippet().map(|snippet| snippet.id()) == self.snippet
    }

    /// Inserts the service for the tree being analyzed, unless another visitor
    /// already did.
    fn insert_into(ctx: &mut VisitorStartContext<JsLanguage>) {
        if ctx.services.get_service::<Self>().is_none() {
            let service = Self::from_services(ctx.root, ctx.services);
            ctx.services.insert_service(service);
        }
    }
}

/// Builds the Vue semantic model and emits a query match for each component,
/// each declaration and each reference written in the analyzed tree.
pub(crate) struct VueSemanticVisitor;

impl Visitor for VueSemanticVisitor {
    type Language = JsLanguage;

    fn start(&mut self, mut ctx: VisitorStartContext<JsLanguage>) {
        VueSemanticService::insert_into(&mut ctx);
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
            for emit in component.emits().filter(|emit| service.declares(emit)) {
                ctx.match_query(VueEmitMatch(emit));
            }
        }
        if let Some(snippet) = service.snippet {
            ctx.match_query(VueSnippetMatch(service.model.snippet(snippet)));
        }
    }
}

/// Builds the Vue semantic model and emits a query match for each reference
/// written in the analyzed tree.
pub(crate) struct VueReferenceVisitor;

impl Visitor for VueReferenceVisitor {
    type Language = JsLanguage;

    fn start(&mut self, mut ctx: VisitorStartContext<JsLanguage>) {
        VueSemanticService::insert_into(&mut ctx);
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
        for reference in service.model.references_in(service.snippet) {
            ctx.match_query(VueReferenceMatch(reference));
        }
    }
}

/// Builds the Vue semantic model and emits a query match for each event name
/// written in the analyzed tree, in an emit declaration or an emit call.
pub(crate) struct VueEventVisitor;

impl Visitor for VueEventVisitor {
    type Language = JsLanguage;

    fn start(&mut self, mut ctx: VisitorStartContext<JsLanguage>) {
        VueSemanticService::insert_into(&mut ctx);
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
            for emit in component.emits().filter(|emit| service.declares(emit)) {
                ctx.match_query(VueEventMatch(VueEventSite::Declaration(emit)));
            }
        }
        for reference in service
            .model
            .references_in(service.snippet)
            .filter(|reference| reference.site() == RefSite::EventName)
        {
            ctx.match_query(VueEventMatch(VueEventSite::Call(reference)));
        }
    }
}

macro_rules! define_vue_query {
    ($(#[$meta:meta])* $query:ident, $input:ident, $output:ty) => {
        define_vue_query!($(#[$meta])* $query, $input, $output, VueSemanticVisitor);
    };
    ($(#[$meta:meta])* $query:ident, $input:ident, $output:ty, $visitor:ident) => {
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
                analyzer.add_visitor(Phases::Semantic, || $visitor);
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
define_vue_query!(
    /// Matches each event each Vue component declares, whatever syntax
    /// declares it.
    VueEmit,
    VueEmitMatch,
    Emit
);
define_vue_query!(
    /// Matches the analyzed tree when it is a snippet embedded in a `.vue`
    /// file.
    VueSnippet,
    VueSnippetMatch,
    Snippet
);
define_vue_query!(
    /// Matches each reference written in the analyzed tree, whatever it
    /// resolves to.
    VueReference,
    VueReferenceMatch,
    Reference,
    VueReferenceVisitor
);

/// The services of a rule that queries [`VueSemantic`]: the JavaScript
/// semantic model of the analyzed tree, and the Vue semantic model of the
/// file it belongs to.
pub struct VueSemanticServices {
    model: SemanticModel,
    vue: VueSemanticService,
}

impl VueSemanticServices {
    /// The JavaScript semantic model of the analyzed tree.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }

    /// The snippet of a single-file component that the analyzed tree is.
    ///
    /// Returns `None` when the analyzed tree is a whole JavaScript file, or
    /// when the model of its file is not available.
    pub fn snippet(&self) -> Option<Snippet> {
        self.vue.snippet.map(|id| self.vue.model.snippet(id))
    }
}

impl FromServices for VueSemanticServices {
    fn from_services(
        rule_key: &RuleKey,
        _rule_metadata: &RuleMetadata,
        services: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        let model: &SemanticModel = services
            .get_service()
            .ok_or_else(|| ServicesDiagnostic::new(rule_key.rule_name(), &["SemanticModel"]))?;
        let vue: &VueSemanticService = services.get_service().ok_or_else(|| {
            ServicesDiagnostic::new(rule_key.rule_name(), &["VueSemanticService"])
        })?;
        Ok(Self {
            model: model.clone(),
            vue: vue.clone(),
        })
    }
}

impl Phase for VueSemanticServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

/// Matches each node of type `N`, and gives the rule both the JavaScript and
/// the Vue semantic models.
#[derive(Clone)]
pub struct VueSemantic<N>(pub N);

impl<N> Queryable for VueSemantic<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = JsSyntaxNode;
    type Output = N;
    type Language = JsLanguage;
    type Services = VueSemanticServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, || VueSemanticVisitor);
        analyzer.add_visitor(Phases::Semantic, SyntaxVisitor::default);
    }

    fn key() -> QueryKey<Self::Language> {
        QueryKey::Syntax(N::KIND_SET)
    }

    fn unwrap_match(_: &ServiceBag, node: &Self::Input) -> Self::Output {
        N::unwrap_cast(node.clone())
    }
}

define_vue_query!(
    /// Matches each event name written in an emit declaration or an emit call,
    /// whatever syntax declares or emits the event.
    VueEvent,
    VueEventMatch,
    VueEventSite,
    VueEventVisitor
);

/// An event name written in the analyzed tree.
#[derive(Clone, Debug)]
pub enum VueEventSite {
    /// An event a component declares, as in `defineEmits(['change'])`.
    Declaration(Emit),
    /// The event name passed to a call that emits the event, as in
    /// `emit('change', value)`. Only names written as a string are matched.
    Call(Reference),
}

impl VueEventSite {
    pub fn name(&self) -> &str {
        match self {
            Self::Declaration(emit) => emit.name(),
            Self::Call(reference) => reference.name(),
        }
    }

    /// The range of the event name, in the coordinates of the analyzed tree.
    pub fn range(&self) -> TextRange {
        match self {
            Self::Declaration(emit) => emit.range(),
            Self::Call(reference) => reference.range(),
        }
    }
}
