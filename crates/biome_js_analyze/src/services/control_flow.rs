use biome_analyze::QueryMatch;
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, Queryable, RuleKey, RuleMetadata, ServiceBag,
    ServicesDiagnostic, Visitor, VisitorContext,
};
use biome_control_flow::{ExceptionHandlerKind, InstructionKind, builder::ROOT_BLOCK_ID};
use biome_db::AnyParsedSource;
use biome_js_control_flow::{ControlFlowModel, control_flow_model, js_control_flow_model};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsReturnStatement, JsSyntaxNode, TextRange};
use biome_languages::LanguageDb;
use biome_rowan::{AstNode, NodeOrToken, WalkEvent};
use roaring::RoaringBitmap;
use std::{marker::PhantomData, ops::Deref, rc::Rc};

use super::{semantic::SemanticModelBuilderVisitor, typed::TypedService};

pub(crate) use biome_js_control_flow::AnyJsControlFlowRoot;
pub use biome_js_control_flow::JsControlFlowGraph;

#[derive(Clone)]
pub struct ControlFlowGraph {
    pub graph: JsControlFlowGraph,
}

impl Deref for ControlFlowGraph {
    type Target = JsControlFlowGraph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

/// Schedules CFG consumers in the semantic phase without requiring a binding model.
pub struct ControlFlowServices;

impl FromServices for ControlFlowServices {
    fn from_services(
        _: &RuleKey,
        _: &RuleMetadata,
        _: &ServiceBag,
    ) -> Result<Self, ServicesDiagnostic> {
        Ok(Self)
    }
}

impl Phase for ControlFlowServices {
    fn phase() -> Phases {
        Phases::Semantic
    }
}

impl ControlFlowServices {
    fn model(ctx: &VisitorContext<JsLanguage>) -> ControlFlowModel {
        let db = ctx.services.get_service::<Rc<dyn LanguageDb>>();
        if let Some(db) = db
            && let Some(source) = ctx.services.get_service::<AnyParsedSource>()
        {
            js_control_flow_model(db.as_ref(), source).clone()
        } else {
            // Fix passes and standalone callers can analyze trees that are not stored
            // in the database. A graph for the file's previous tree would have stale nodes.
            control_flow_model(ctx.root)
        }
    }
}

impl QueryMatch for ControlFlowGraph {
    fn text_range(&self) -> TextRange {
        self.graph.node.text_trimmed_range()
    }
}

impl Queryable for ControlFlowGraph {
    type Input = Self;
    type Output = Self;

    type Language = JsLanguage;
    type Services = ControlFlowServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Semantic, || ControlFlowGraphVisitor);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self) -> Self::Output {
        query.clone()
    }
}

/// Queries graphs rooted at `N`, with type inference and semantic bindings available.
pub struct TypedControlFlowGraph<N> {
    graph: JsControlFlowGraph,
    node: N,
}

impl<N> QueryMatch for TypedControlFlowGraph<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    fn text_range(&self) -> TextRange {
        self.node.range()
    }
}

impl<N> Queryable for TypedControlFlowGraph<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Input = Self;
    type Output = ControlFlowGraph;
    type Language = JsLanguage;
    type Services = TypedService;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, || SemanticModelBuilderVisitor);
        analyzer.add_visitor(Phases::Semantic, TypedControlFlowVisitor::<N>::new);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self::Input) -> Self::Output {
        ControlFlowGraph {
            graph: query.graph.clone(),
        }
    }
}

impl ControlFlowGraph {
    /// Visits reachable explicit returns and reports whether a path falls through.
    /// Throwing paths are excluded. Expression-bodied arrows must be handled separately.
    pub(crate) fn visit_return_paths(
        &self,
        mut visit_return: impl FnMut(JsReturnStatement),
    ) -> bool {
        let mut falls_through = false;
        let mut block_stack = vec![ROOT_BLOCK_ID];
        let mut visited_blocks = RoaringBitmap::new();
        visited_blocks.insert(ROOT_BLOCK_ID.index());
        while let Some(block_id) = block_stack.pop() {
            let block = self.get(block_id);
            for handler in block.exception_handlers.iter() {
                if matches!(handler.kind, ExceptionHandlerKind::Catch)
                    && visited_blocks.insert(handler.target.index())
                {
                    block_stack.push(handler.target);
                }
            }
            for instruction in block.instructions.iter() {
                match instruction.kind {
                    InstructionKind::Statement => {}
                    InstructionKind::Jump {
                        conditional, block, ..
                    } => {
                        if visited_blocks.insert(block.index()) {
                            block_stack.push(block);
                        }
                        if !conditional {
                            break;
                        }
                    }
                    InstructionKind::Return => {
                        match &instruction.node {
                            Some(NodeOrToken::Node(node)) => {
                                if let Some(statement) = JsReturnStatement::cast_ref(node) {
                                    visit_return(statement);
                                }
                            }
                            _ => falls_through = true,
                        }
                        break;
                    }
                }
            }
        }
        falls_through
    }
}

struct ControlFlowGraphVisitor;

impl Visitor for ControlFlowGraphVisitor {
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let WalkEvent::Enter(node) = event else {
            return;
        };
        if node.parent().is_some() {
            return;
        }
        let model = ControlFlowServices::model(&ctx);
        // Queue every root together so nested-body diagnostics are sorted by
        // source range before the semantic phase flushes its signals.
        for graph in model.graphs() {
            ctx.match_query(ControlFlowGraph { graph });
        }
    }
}

struct TypedControlFlowVisitor<N> {
    node: PhantomData<N>,
}

impl<N> TypedControlFlowVisitor<N> {
    fn new() -> Self {
        Self { node: PhantomData }
    }
}

impl<N> Visitor for TypedControlFlowVisitor<N>
where
    N: AstNode<Language = JsLanguage> + 'static,
{
    type Language = JsLanguage;

    fn visit(&mut self, event: &WalkEvent<JsSyntaxNode>, mut ctx: VisitorContext<JsLanguage>) {
        let WalkEvent::Enter(node) = event else {
            return;
        };
        if node.parent().is_some() {
            return;
        }

        let mut model = None;
        // Filter before requesting the model or reconstructing graphs, and queue all
        // matching roots before the semantic phase flushes diagnostics.
        for node in node.descendants().filter_map(N::cast) {
            let Some(root) = AnyJsControlFlowRoot::cast_ref(node.syntax()) else {
                continue;
            };
            let model = model.get_or_insert_with(|| ControlFlowServices::model(&ctx));
            if let Some(graph) = model.graph(&root) {
                ctx.match_query(TypedControlFlowGraph { graph, node });
            }
        }
    }
}
