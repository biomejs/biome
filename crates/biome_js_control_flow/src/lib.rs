//! JavaScript execution structure shared by workspace semantic services.
//!
//! Construction records one graph per execution root. The immutable model stores
//! source locators instead of thread-local syntax nodes; consumers reconstruct
//! syntax-backed graphs when inspecting a body.

#![deny(clippy::use_self)]
#![deny(rustdoc::broken_intra_doc_links)]

mod db;
mod model;
mod nodes;
mod visitor;

#[cfg(test)]
mod tests;

pub use db::{
    control_flow_model_from_snippet, control_flow_model_from_source, js_control_flow_model,
};
pub use model::ControlFlowModel;
pub use visitor::AnyJsControlFlowRoot;

use biome_js_syntax::{AnyJsRoot, JsLanguage};
use biome_rowan::AstNode;

pub type JsControlFlowGraph = biome_control_flow::ControlFlowGraph<JsLanguage>;
pub(crate) type FunctionBuilder = biome_control_flow::builder::FunctionBuilder<JsLanguage>;

/// Builds graphs for the execution roots in a JavaScript or TypeScript tree.
///
/// A malformed statement discards its enclosing graph. Independently constructible
/// nested and sibling execution roots still contribute their graphs.
pub fn control_flow_model(root: &AnyJsRoot) -> ControlFlowModel {
    let mut visitor = visitor::ControlFlowVisitor::new();
    let mut graphs = Vec::new();
    for event in root.syntax().preorder() {
        visitor.visit(event, &mut graphs);
    }
    ControlFlowModel::new(root, graphs)
}
