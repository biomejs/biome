//! Query types that hand a lint rule the Vue components of a file, or their
//! declarations, as described by the Vue semantic model.
//!
//! A rule picks the unit it judges as its `Query`:
//!
//! - [`VueComponent`] runs the rule once per component. Use it when the rule
//!   compares declarations with each other.
//! - [`VueProp`] runs the rule once per prop. Use it when the rule judges
//!   one prop on its own.
//! - [`VueModel`] runs the rule once per model declared with
//!   `defineModel()`. Use it when the rule judges one model on its own.
//! - [`VueEmit`] runs the rule once per declared event. Use it when the rule
//!   judges one event declaration on its own.
//! - [`VueComponentRegistration`] runs the rule once per entry of the
//!   `components` option. Use it when the rule judges one locally registered
//!   component on its own.
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
//! - [`VueElseIf`] runs the rule once on the condition of a `v-else-if`,
//!   with the conditions of the branches before it. Use it when the rule
//!   compares the conditions of a `v-if` / `v-else-if` chain.
//!
//! The model already folds the different ways Vue lets a declaration be
//! written into one shape, so a rule never matches on syntax to find them.
//! When a rule needs the syntax anyway, to build a fix or to read what the
//! model does not describe, [`AnalyzedVueModel::node`] returns the node a row
//! of the model stands for in constant time.
//!
//! A query for another kind of declaration is one `define_vue_query!` call
//! and one loop in [`VueSemanticVisitor`]. References have their own visitors,
//! [`VueReferenceVisitor`] and [`VueEventVisitor`], so that files analyzed
//! without a rule that asks for them don't pay for matching every reference.
//! [`VueElseIfVisitor`] reads the trees of other snippets, so it has its own
//! visitor for the same reason.

use crate::services::semantic::SemanticModelBuilderVisitor;
use crate::utils::condition::{is_same_condition, split_by_logical_operator};
use biome_analyze::{
    AddVisitor, AnalyzedTree, FromServices, Phase, Phases, QueryKey, QueryMatch, Queryable,
    RuleKey, RuleMetadata, ServiceBag, ServicesDiagnostic, SyntaxVisitor, Visitor, VisitorContext,
    VisitorStartContext,
};
use biome_js_semantic::SemanticModel;
use biome_js_syntax::{
    AnyJsExpression, AnyJsRoot, JsLanguage, JsLogicalOperator, JsSyntaxNode, TextRange,
};
use biome_languages::{JsFileSource, LanguageDb};
use biome_parser::db::{AnalyzedSource, SourceExt};
use biome_rowan::{AstNode, WalkEvent};
use biome_vue_semantic::{
    AnalyzedVueModel, Component, DirectiveKind, Emit, Prop, RefSite, Reference, Snippet, SnippetId,
    Symbol, VueSemanticModel, component_model, vue_model_from_source,
};
use std::rc::Rc;

/// The Vue semantic model of the analyzed file, and which of its embedded
/// snippets the analyzed tree is.
#[derive(Clone, Debug)]
pub struct VueSemanticService {
    model: AnalyzedVueModel<JsLanguage>,
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
                        vue_model_from_source(db, *file).clone(),
                        None,
                        tree,
                    )),
                    AnalyzedSource::Snippet(snippet) => {
                        // Use the host document of the snippet: the document
                        // of its path can be the one of another client.
                        let model = vue_model_from_source(db, snippet.source()).clone();
                        let snippet = model.snippet_at_offset(snippet.content_offset())?.id();
                        Some(Self::new(model, Some(snippet), tree))
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
            Self::new(model, None, tree)
        })
    }

    fn new(
        model: VueSemanticModel,
        snippet: Option<SnippetId>,
        tree: AnalyzedTree<'_, JsLanguage>,
    ) -> Self {
        Self {
            model: AnalyzedVueModel::<JsLanguage>::new(model, tree, snippet),
            snippet,
        }
    }

    /// The model of the whole file, including the script blocks of a
    /// single-file component other than the analyzed one.
    pub(crate) fn model(&self) -> &AnalyzedVueModel<JsLanguage> {
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
            let service = Self::from_services(ctx.tree(), ctx.services);
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
            for model in component.models().filter(|model| service.declares(model)) {
                ctx.match_query(VueModelMatch(model));
            }
            for emit in component.emits().filter(|emit| service.declares(emit)) {
                ctx.match_query(VueEmitMatch(emit));
            }
            for registration in component
                .registered_components()
                .filter(|registration| service.declares(registration))
            {
                ctx.match_query(VueComponentRegistrationMatch(registration));
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

/// Emits a query match when the analyzed tree is the condition of a
/// `v-else-if`, with the conditions of the branches before it in its chain.
pub(crate) struct VueElseIfVisitor;

impl Visitor for VueElseIfVisitor {
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
        let Some(condition) = VueElseIfCondition::from_services(node, ctx.services) else {
            return;
        };
        ctx.match_query(VueElseIfMatch(condition));
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
            type Services = VueSemanticServices;

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
    /// Matches each model of each Vue component, declared with
    /// `defineModel()`. A model also declares a prop, so its output is the
    /// [`Prop`] it declares.
    VueModel,
    VueModelMatch,
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
    /// Matches each entry of the `components` option of each Vue component.
    VueComponentRegistration,
    VueComponentRegistrationMatch,
    Symbol
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

/// The services of the rules that query the Vue semantic model: the
/// JavaScript semantic model of the analyzed tree, and the Vue semantic model
/// of the file it belongs to.
pub struct VueSemanticServices {
    model: SemanticModel,
    vue: VueSemanticService,
}

impl VueSemanticServices {
    /// The JavaScript semantic model of the analyzed tree.
    pub fn model(&self) -> &SemanticModel {
        &self.model
    }

    /// The Vue semantic model of the file the analyzed tree belongs to, with
    /// the nodes of the analyzed tree its rows stand for.
    ///
    /// When the analyzed tree is not the one stored for the file, as in a fix
    /// pass, the model covers the analyzed tree only.
    pub fn vue_model(&self) -> &AnalyzedVueModel<JsLanguage> {
        self.vue.model()
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

define_vue_query!(
    /// Matches the analyzed tree when it is the condition of a `v-else-if`.
    VueElseIf,
    VueElseIfMatch,
    VueElseIfCondition,
    VueElseIfVisitor
);

/// The condition of a `v-else-if`, written in the analyzed tree, and the
/// conditions of the branches before it in its chain.
#[derive(Clone)]
pub struct VueElseIfCondition {
    condition: AnyJsExpression,
    earlier: Rc<[EarlierCondition]>,
}

impl VueElseIfCondition {
    /// Returns the condition written in `root`, the root of the analyzed
    /// tree, when the tree is the condition of a `v-else-if` and parsed
    /// without errors.
    fn from_services(root: &JsSyntaxNode, services: &ServiceBag) -> Option<Self> {
        let service = services.get_service::<VueSemanticService>()?;
        let snippet = service.model.snippet(service.snippet?);
        let attr = snippet.attr()?;
        if attr.directive_kind() != Some(DirectiveKind::ElseIf) {
            return None;
        }
        let (chain, index) = attr.element().chain()?;
        let db = services.get_service::<Rc<dyn LanguageDb>>()?.as_ref();
        let source = services.get_service::<AnalyzedSource>()?;
        if let AnalyzedSource::Snippet(analyzed) = source
            && analyzed.has_errors()
        {
            return None;
        }
        let condition = AnyJsRoot::cast_ref(root)?
            .as_js_expression_template_root()?
            .expression()?;

        // The model tells which snippets hold the earlier conditions, and the
        // database holds their trees.
        let stored = source.document().snippets(db);
        let branches: Vec<_> = chain.branches().take(index).collect();
        let earlier = branches
            .into_iter()
            .rev()
            .filter_map(|branch| {
                let offset = branch.condition?.offset();
                // Only a JavaScript snippet can be cast to a JavaScript root.
                let stored = stored.iter().find(|stored| {
                    stored.content_offset(db) == offset
                        && db
                            .source_from_index(stored.document_source_index(db))
                            .and_then(|source| source.to_js_file_source())
                            .is_some()
                })?;
                if stored.has_errors(db) {
                    return None;
                }
                let root: AnyJsRoot = stored.parsed(db).tree();
                let expression = root.as_js_expression_template_root()?.expression()?;
                Some(EarlierCondition(expression))
            })
            .collect();
        Some(Self { condition, earlier })
    }

    /// The condition of the `v-else-if`, in the analyzed tree.
    pub fn condition(&self) -> &AnyJsExpression {
        &self.condition
    }

    /// The conditions of the `v-if` and `v-else-if` branches before this one
    /// in its chain, nearest first. A branch without a condition, or whose
    /// condition doesn't parse, has no entry.
    pub fn earlier_conditions(&self) -> &[EarlierCondition] {
        &self.earlier
    }

    /// The range of the condition, in the coordinates of the analyzed tree.
    pub fn range(&self) -> TextRange {
        self.condition.range()
    }
}

/// A condition written in another snippet of the file than the analyzed one.
///
/// Its nodes belong to the tree of that snippet, whose positions don't match
/// the ones of the analyzed tree, so a diagnostic must never point at them.
/// This type only lets a rule split the condition and compare its parts with
/// expressions of the analyzed tree.
#[derive(Clone)]
pub struct EarlierCondition(AnyJsExpression);

impl EarlierCondition {
    /// Splits the condition into the operands that, joined by `operator`,
    /// make it. See [`split_by_logical_operator`].
    pub fn split(&self, operator: JsLogicalOperator) -> impl Iterator<Item = Self> {
        split_by_logical_operator(operator, self.0.clone()).map(Self)
    }

    /// Returns whether `expression` is the same condition as this one. See
    /// [`is_same_condition`].
    pub fn is_same_condition(&self, expression: &AnyJsExpression) -> bool {
        is_same_condition(&self.0, expression)
    }
}
