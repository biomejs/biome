use biome_control_flow::builder::BlockId;
use biome_js_syntax::{AnyJsSwitchClause, JsLabeledStatement, JsSwitchStatement, JsSyntaxToken};
use biome_rowan::{AstNode, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

pub(in crate::services::control_flow) struct SwitchVisitor {
    entry_block: BlockId,
    // `label` and `break_block` are used by the `BreakVisitor`
    pub(super) label: Option<JsSyntaxToken>,
    pub(super) break_block: BlockId,
    /// Flag used by the [CaseVisitor] to check if it's the first case clause
    /// in a switch statement (used to implement fallthrough)
    is_first_case_clause: bool,
    default_block: Option<(BlockId, JsSyntaxToken)>,
}

impl NodeVisitor for SwitchVisitor {
    type Node = JsSwitchStatement;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<Self> {
        // Execute the discriminant expression as a side-effect
        builder.append_statement().with_node(node.discriminant()?);

        let entry_block = builder.cursor();
        let break_block = builder.append_block();

        let label = node
            .parent::<JsLabeledStatement>()
            .and_then(|label| label.label_token().ok());

        Ok(Self {
            entry_block,
            label,
            break_block,
            is_first_case_clause: true,
            default_block: None,
        })
    }

    fn exit(
        self,
        _: Self::Node,
        builder: &mut FunctionBuilder,
        _: StatementStack,
    ) -> SyntaxResult<()> {
        let Self {
            entry_block,
            break_block,
            is_first_case_clause,
            ..
        } = self;

        // Append an implicit jump to the break block at the end of the last
        // clause, if the statement had at least one
        if !is_first_case_clause {
            builder.append_jump(false, break_block);
        }

        // Also implicitly jump to either the default block or the break block
        // (over the switch statement) at the end of the entry block if no case
        // was matched
        builder.set_cursor(entry_block);
        if let Some((block, token)) = self.default_block {
            builder.append_jump(false, block).with_node(token);
        } else {
            builder.append_jump(false, break_block);
        }

        builder.set_cursor(break_block);

        Ok(())
    }
}

pub(in crate::services::control_flow) struct CaseVisitor;

impl NodeVisitor for CaseVisitor {
    type Node = AnyJsSwitchClause;

    fn enter(
        node: Self::Node,
        builder: &mut FunctionBuilder,
        stack: StatementStack,
    ) -> SyntaxResult<Self> {
        let case_block = builder.append_block();

        let switch_stmt = stack.read_top::<SwitchVisitor>()?;

        if !switch_stmt.is_first_case_clause {
            builder.append_jump(false, case_block);
        } else {
            switch_stmt.is_first_case_clause = false;
        }

        match node {
            AnyJsSwitchClause::JsCaseClause(node) => {
                builder.set_cursor(switch_stmt.entry_block);
                builder
                    .append_jump(true, case_block)
                    .with_node(node.test()?.into_syntax());
            }
            AnyJsSwitchClause::JsDefaultClause(node) => {
                let token = node.default_token()?;
                switch_stmt.default_block = Some((case_block, token));
            }
        }

        builder.set_cursor(case_block);

        Ok(Self)
    }
}
