use biome_analyze::QueryMatch;
use biome_analyze::{
    AddVisitor, FromServices, Phase, Phases, Queryable, RuleKey, RuleMetadata, ServiceBag,
    ServicesDiagnostic, Visitor, VisitorContext,
};
use biome_db::AnyParsedSource;
use biome_js_control_flow::{control_flow_model, js_control_flow_model};
use biome_js_syntax::AnyJsRoot;
use biome_js_syntax::JsLanguage;
use biome_js_syntax::JsSyntaxNode;
use biome_js_syntax::TextRange;
use biome_languages::LanguageDb;
use biome_rowan::WalkEvent;
use std::rc::Rc;

pub(crate) use biome_js_control_flow::AnyJsControlFlowRoot;
pub use biome_js_control_flow::JsControlFlowGraph;

pub struct ControlFlowGraph {
    pub graph: JsControlFlowGraph,
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

impl QueryMatch for ControlFlowGraph {
    fn text_range(&self) -> TextRange {
        self.graph.node.text_trimmed_range()
    }
}

impl Queryable for ControlFlowGraph {
    type Input = Self;
    type Output = JsControlFlowGraph;

    type Language = JsLanguage;
    type Services = ControlFlowServices;

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Semantic, || ControlFlowGraphVisitor);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self) -> Self::Output {
        query.graph.clone()
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
        let db = ctx.services.get_service::<Rc<dyn LanguageDb>>();
        let model = if let Some(db) = db
            && let Some(source) = ctx.services.get_service::<AnyParsedSource>()
        {
            js_control_flow_model(db.as_ref(), source).clone()
        } else {
            // Fix passes and standalone callers can analyze trees that are not stored
            // in the database. A graph for the file's previous tree would have stale nodes.
            control_flow_model(ctx.root)
        };
        // Queue every root together so nested-body diagnostics are sorted by
        // source range before the semantic phase flushes its signals.
        for graph in model.graphs() {
            ctx.match_query(ControlFlowGraph { graph });
        }
    }
}
