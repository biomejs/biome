use biome_analyze::QueryMatch;
use biome_analyze::{AddVisitor, Phases, Queryable, ServiceBag};
use biome_js_syntax::AnyJsRoot;
use biome_js_syntax::JsLanguage;
use biome_js_syntax::TextRange;

pub type JsControlFlowGraph = biome_control_flow::ControlFlowGraph<JsLanguage>;
pub(crate) type FunctionBuilder = biome_control_flow::builder::FunctionBuilder<JsLanguage>;

mod nodes;
mod visitor;

pub(crate) use self::visitor::AnyJsControlFlowRoot;
pub(crate) use self::visitor::make_visitor;

pub struct ControlFlowGraph {
    pub graph: JsControlFlowGraph,
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
    type Services = ();

    fn build_visitor(analyzer: &mut impl AddVisitor<JsLanguage>, _: &AnyJsRoot) {
        analyzer.add_visitor(Phases::Syntax, make_visitor);
    }

    fn unwrap_match(_: &ServiceBag, query: &Self) -> Self::Output {
        query.graph.clone()
    }
}
