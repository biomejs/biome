use biome_control_flow::{builder::BlockId, ExceptionHandlerKind};
use biome_js_syntax::{AnyJsTryStatement, JsCatchClause, JsFinallyClause};
use biome_rowan::SyntaxResult;

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct TryVisitor {
    catch_block: Option<BlockId>,
    finally_block: Option<BlockId>,
    next_block: BlockId,
}

impl NodeVisitor for TryVisitor {
    type Node = AnyJsTryStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        let (has_catch, has_finally) = match node {
            AnyJsTryStatement::JsTryStatement(_) => (true, false),
            AnyJsTryStatement::JsTryFinallyStatement(node) => (node.catch_clause().is_some(), true),
        };

        let next_block = builder.append_block();

        let finally_block = if has_finally {
            let finally_block = builder.append_block();
            builder.push_exception_target(ExceptionHandlerKind::Finally, finally_block);
            Some(finally_block)
        } else {
            None
        };

        let catch_block = if has_catch {
            let catch_block = builder.append_block();
            builder.push_exception_target(ExceptionHandlerKind::Catch, catch_block);
            Some(catch_block)
        } else {
            None
        };

        // Create the actual try block (with the exception target set), append
        // an implicit jump to it and move the cursor there
        let try_block = builder.append_block();
        builder.append_jump(false, try_block);
        builder.set_cursor(try_block);

        Ok(Self {
            catch_block,
            finally_block,
            next_block,
        })
    }
}

pub(in crate::services::control_flow) struct CatchVisitor;

impl NodeVisitor for CatchVisitor {
    type Node = JsCatchClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let try_stmt = stack.read_top::<TryVisitor>()?;

        // Insert an implicit jump from the end of the `try` block to the
        // `finally` block if it exists, or to the `next` block otherwise
        builder.append_jump(false, try_stmt.finally_block.unwrap_or(try_stmt.next_block));

        // Pop the catch block from the exception stack
        builder.pop_exception_target();

        // SAFETY: This block should have been created by the `TryVisitor`
        let catch_block = try_stmt.catch_block.unwrap();
        builder.set_cursor(catch_block);

        Ok(Self)
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<()> {
        let try_stmt = stack.read_top::<TryVisitor>()?;

        // Implicit jump from the end of the catch block to the finally block
        // (if it exists), or to the next block otherwise
        let next_block = try_stmt.finally_block.unwrap_or(try_stmt.next_block);
        builder.append_jump(false, next_block);
        builder.set_cursor(next_block);

        Ok(())
    }
}

pub(in crate::services::control_flow) struct FinallyVisitor;

impl NodeVisitor for FinallyVisitor {
    type Node = JsFinallyClause;

    fn enter(
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let try_stmt = stack.read_top::<TryVisitor>()?;

        // SAFETY: This block should have been created by the `TryVisitor`
        let finally_block = try_stmt.finally_block.unwrap();

        // If the try statement has no catch clause
        if try_stmt.catch_block.is_none() {
            // Insert an implicit jump from the end of the try block to the
            // `finally` block
            builder.append_jump(false, finally_block);

            // Move the cursor to the finally block (this has already been done
            // by the `CatchVisitor` if the try statement has a catch clause)
            builder.set_cursor(finally_block);
        }

        // Pop the finally block from the exception stack
        builder.pop_exception_target();

        Ok(Self)
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<()> {
        let try_stmt = stack.read_top::<TryVisitor>()?;

        // Implicit jump from the end of the finally block to the next block
        builder.append_finally_fallthrough(try_stmt.next_block);

        builder.set_cursor(try_stmt.next_block);

        Ok(())
    }
}
