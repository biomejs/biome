use crate::{AnyJsControlFlowRoot, JsControlFlowGraph};
use biome_control_flow::{BasicBlock, ExceptionHandler, Instruction, InstructionKind};
use biome_js_syntax::{AnyJsRoot, JsLanguage, JsSyntaxElement, JsSyntaxNode, JsSyntaxNodePtr};
use biome_rowan::{AstNode, NodeOrToken, SendNode};
use rustc_hash::FxHashMap;
use std::sync::Arc;

/// Immutable control-flow graphs associated with one complete syntax tree.
///
/// Equality includes the source tree, instruction locations, and graph structure.
/// Source edits therefore cannot backdate stale syntax through a tracked query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlFlowModel(Arc<ControlFlowModelData>);

#[derive(Debug, Eq, PartialEq)]
struct ControlFlowModelData {
    root: Option<SendNode>,
    graphs: FxHashMap<JsSyntaxNodePtr, Vec<BlockData>>,
}

impl ControlFlowModel {
    pub(crate) fn new(root: &AnyJsRoot, graphs: Vec<JsControlFlowGraph>) -> Self {
        let root = root.syntax().as_send();
        debug_assert!(root.is_some(), "CFG model requires a root");
        Self(Arc::new(ControlFlowModelData {
            root,
            graphs: graphs
                .into_iter()
                .filter_map(|graph| {
                    Some((
                        JsSyntaxNodePtr::new(&graph.node),
                        graph
                            .blocks
                            .into_iter()
                            .map(BlockData::new)
                            .collect::<Option<_>>()?,
                    ))
                })
                .collect(),
        }))
    }

    /// Returns the graph at the given execution root's source location.
    ///
    /// The root must belong to the syntax snapshot used to build this model.
    /// Returns `None` when malformed syntax prevented constructing that graph.
    pub fn graph(&self, root: &AnyJsControlFlowRoot) -> Option<JsControlFlowGraph> {
        let pointer = JsSyntaxNodePtr::new(root.syntax());
        let blocks = self.0.graphs.get(&pointer)?;
        let syntax_root = self.0.root.clone()?.into_node::<JsLanguage>()?;
        restore_graph(pointer, blocks, &syntax_root)
    }

    /// Iterates over all constructible execution-root graphs in unspecified order.
    pub fn graphs(&self) -> impl Iterator<Item = JsControlFlowGraph> + '_ {
        // Graphs come in arbitrary order, but two roots of the same file
        // usually share several ancestors (a class body, a statement list),
        // so resolving near the previous root creates fewer nodes than
        // resolving from the tree's root.
        let mut near = self
            .0
            .root
            .clone()
            .and_then(SendNode::into_node::<JsLanguage>);
        self.0.graphs.iter().filter_map(move |(pointer, blocks)| {
            let graph = restore_graph(*pointer, blocks, near.as_ref()?)?;
            near = Some(graph.node.clone());
            Some(graph)
        })
    }
}

/// Restores a graph from its stored locations, resolving them in the tree of
/// `near` (see [JsSyntaxNodePtr::as_node_near]).
fn restore_graph(
    pointer: JsSyntaxNodePtr,
    blocks: &[BlockData],
    near: &JsSyntaxNode,
) -> Option<JsControlFlowGraph> {
    let node = pointer.as_node_near(near)?;
    // Consecutive instructions are usually close in the tree, so each one is
    // resolved near the node of the previous one instead of from the root.
    let mut near = node.clone();
    let blocks = blocks
        .iter()
        .map(|block| {
            Some(BasicBlock {
                instructions: block
                    .instructions
                    .iter()
                    .map(|instruction| {
                        Some(Instruction {
                            kind: instruction.kind,
                            node: match &instruction.node {
                                Some(node) => {
                                    let (element, parent) = node.to_element_near(&near)?;
                                    near = parent;
                                    Some(element)
                                }
                                None => None,
                            },
                        })
                    })
                    .collect::<Option<_>>()?,
                exception_handlers: block.exception_handlers.clone(),
                cleanup_handlers: block.cleanup_handlers.clone(),
            })
        })
        .collect::<Option<_>>()?;
    Some(JsControlFlowGraph { node, blocks })
}

#[derive(Debug, Eq, PartialEq)]
struct BlockData {
    instructions: Vec<InstructionData>,
    exception_handlers: Vec<ExceptionHandler>,
    cleanup_handlers: Vec<ExceptionHandler>,
}

impl BlockData {
    fn new(block: BasicBlock<JsLanguage>) -> Option<Self> {
        Some(Self {
            instructions: block
                .instructions
                .into_iter()
                .map(|instruction| {
                    Some(InstructionData {
                        kind: instruction.kind,
                        node: match instruction.node {
                            Some(node) => Some(ElementPtr::new(node)?),
                            None => None,
                        },
                    })
                })
                .collect::<Option<_>>()?,
            exception_handlers: block.exception_handlers,
            cleanup_handlers: block.cleanup_handlers,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct InstructionData {
    kind: InstructionKind,
    node: Option<ElementPtr>,
}

#[derive(Debug, Eq, PartialEq)]
enum ElementPtr {
    Node(JsSyntaxNodePtr),
    Token {
        parent: JsSyntaxNodePtr,
        index: usize,
    },
}

impl ElementPtr {
    fn new(element: JsSyntaxElement) -> Option<Self> {
        Some(match element {
            NodeOrToken::Node(node) => Self::Node(JsSyntaxNodePtr::new(&node)),
            NodeOrToken::Token(token) => Self::Token {
                parent: JsSyntaxNodePtr::new(&token.parent()?),
                index: token.index(),
            },
        })
    }

    /// Resolves the element in the tree of `near` with
    /// [JsSyntaxNodePtr::as_node_near].
    ///
    /// Returns the element together with the node it was resolved from: the
    /// node itself, or the parent of the token.
    fn to_element_near(&self, near: &JsSyntaxNode) -> Option<(JsSyntaxElement, JsSyntaxNode)> {
        Some(match self {
            Self::Node(pointer) => {
                let node = pointer.as_node_near(near)?;
                (node.clone().into(), node)
            }
            Self::Token { parent, index } => {
                let parent = parent.as_node_near(near)?;
                let token = parent.slots().nth(*index)?.into_token()?;
                (token.into(), parent)
            }
        })
    }
}
