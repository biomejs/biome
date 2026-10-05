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
        let root = self
            .0
            .root
            .clone()
            .and_then(SendNode::into_node::<JsLanguage>);
        self.0
            .graphs
            .iter()
            .filter_map(move |(pointer, blocks)| restore_graph(*pointer, blocks, root.as_ref()?))
    }
}

fn restore_graph(
    pointer: JsSyntaxNodePtr,
    blocks: &[BlockData],
    root: &JsSyntaxNode,
) -> Option<JsControlFlowGraph> {
    Some(JsControlFlowGraph {
        node: pointer.try_to_node(root)?,
        blocks: blocks
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
                                    Some(node) => Some(node.to_element(root)?),
                                    None => None,
                                },
                            })
                        })
                        .collect::<Option<_>>()?,
                    exception_handlers: block.exception_handlers.clone(),
                    cleanup_handlers: block.cleanup_handlers.clone(),
                })
            })
            .collect::<Option<_>>()?,
    })
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

    fn to_element(&self, root: &JsSyntaxNode) -> Option<JsSyntaxElement> {
        Some(match self {
            Self::Node(pointer) => pointer.try_to_node(root)?.into(),
            Self::Token { parent, index } => parent
                .try_to_node(root)?
                .slots()
                .nth(*index)?
                .into_token()?
                .into(),
        })
    }
}
