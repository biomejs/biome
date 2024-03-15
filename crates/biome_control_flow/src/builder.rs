use biome_rowan::{Language, SyntaxElement, SyntaxNode};

use crate::{
    BasicBlock, ControlFlowGraph, ExceptionHandler, ExceptionHandlerKind, Instruction,
    InstructionKind,
};

/// Identifier for a block in a [ControlFlowGraph]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BlockId {
    pub(crate) index: u32,
}

impl BlockId {
    /// Returns the index of the block in the function
    pub fn index(self) -> u32 {
        self.index
    }
}

/// Id of the first block of every Control Flow Graph
pub const ROOT_BLOCK_ID: BlockId = BlockId { index: 0 };

/// Helper struct for building an instance of [ControlFlowGraph], the builder
/// keeps track of an "insertion cursor" within the graph where [Instruction]
/// should be added
pub struct FunctionBuilder<L: Language> {
    result: ControlFlowGraph<L>,
    exception_target: Vec<ExceptionHandler>,
    block_cursor: BlockId,
}

impl<L: Language> FunctionBuilder<L> {
    /// Create a new [FunctionBuilder] instance from a function node
    pub fn new(node: SyntaxNode<L>) -> Self {
        Self {
            result: ControlFlowGraph::new(node),
            exception_target: Vec::new(),
            block_cursor: BlockId { index: 0 },
        }
    }

    /// Finishes building the function
    pub fn finish(mut self) -> ControlFlowGraph<L> {
        // Append the implicit return instruction that resumes execution of the
        // parent procedure when control flow reaches the end of a function
        self.append_return();
        self.result
    }

    /// Allocate a new empty block, returning its [BlockId]
    pub fn append_block(&mut self) -> BlockId {
        let index = self
            .result
            .blocks
            .len()
            .try_into()
            .expect("BlockId overflow");

        let mut has_catch_handler = false;
        self.result.blocks.push(BasicBlock::new(
            // The exception handlers for a block are all the handlers in the
            // current exception stack up to the first catch handler
            self.exception_target
                .iter()
                .rev()
                .copied()
                .take_while(|handler| {
                    let has_previous_catch = has_catch_handler;
                    has_catch_handler |= matches!(handler.kind, ExceptionHandlerKind::Catch);
                    !has_previous_catch
                }),
            // The cleanup handlers for a block are all the handlers in the
            // current exception stack with the catch handlers filtered out
            self.exception_target
                .iter()
                .rev()
                .filter_map(|handler| match handler.kind {
                    ExceptionHandlerKind::Finally => Some(*handler),
                    ExceptionHandlerKind::Catch => None,
                }),
        ));

        BlockId { index }
    }

    /// Get the [BlockId] at the current position of the cursor
    pub fn cursor(&self) -> BlockId {
        self.block_cursor
    }

    /// Move the cursor to the end of `block`
    pub fn set_cursor(&mut self, block: BlockId) {
        debug_assert!(block.index < self.result.blocks.len() as u32);
        self.block_cursor = block;
    }

    /// Push a block as a target on the "exception stack": all blocks created
    /// with this builder will automatically declare an exception edge towards
    /// the topmost entry in this stack
    pub fn push_exception_target(&mut self, kind: ExceptionHandlerKind, target: BlockId) {
        self.exception_target
            .push(ExceptionHandler { kind, target });
    }

    /// Remove the topmost entry from the exception stack
    pub fn pop_exception_target(&mut self) {
        self.exception_target.pop();
    }

    /// Insert an instruction at the current position of the cursor
    fn append_instruction(&mut self, kind: InstructionKind) -> InstructionBuilder<L> {
        let index = self.block_cursor.index as usize;
        let block = &mut self.result.blocks[index];

        let index = block.instructions.len();
        block.instructions.push(Instruction { kind, node: None });

        InstructionBuilder(&mut block.instructions[index])
    }

    pub fn append_statement(&mut self) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Statement)
    }

    pub fn append_return(&mut self) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Return)
    }

    pub fn append_jump(&mut self, conditional: bool, block: BlockId) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Jump {
            conditional,
            block,
            finally_fallthrough: false,
        })
    }

    pub fn append_finally_fallthrough(&mut self, block: BlockId) -> InstructionBuilder<L> {
        self.append_instruction(InstructionKind::Jump {
            conditional: false,
            block,
            finally_fallthrough: true,
        })
    }
}

pub struct InstructionBuilder<'a, L: Language>(&'a mut Instruction<L>);

impl<L: Language> InstructionBuilder<'_, L> {
    pub fn with_node(self, node: impl Into<SyntaxElement<L>>) -> Self {
        self.0.node = Some(node.into());
        self
    }
}
