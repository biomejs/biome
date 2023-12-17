use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_syntax::JsDefaultClause;
use biome_js_syntax::{AnyJsStatement, JsDefaultClauseFields};
use biome_rowan::AstNodeList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDefaultClause;

impl FormatNodeRule<JsDefaultClause> for FormatJsDefaultClause {
    fn fmt_fields(&self, node: &JsDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = node.as_fields();

        // Whether the first statement in the clause is a BlockStatement, and
        // there are no other non-empty statements. Empties may show up when
        // parsing depending on if the input code includes certain newlines.
        //
        // See the comments in `case_clause.rs` for a detailed example.
        let is_single_block_statement = matches!(
            consequent.iter().next(),
            Some(AnyJsStatement::JsBlockStatement(_))
        ) && consequent
            .iter()
            .filter(|statement| !matches!(statement, AnyJsStatement::JsEmptyStatement(_)))
            .count()
            == 1;

        write!(f, [default_token.format(), colon_token.format()])?;

        if f.comments().has_dangling_comments(node.syntax()) {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        if consequent.is_empty() {
            return Ok(());
        }

        if is_single_block_statement {
            write!(f, [space(), consequent.format()])
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args!(
                    hard_line_break(),
                    consequent.format()
                ))]
            )
        }
    }

    fn fmt_dangling_comments(&self, _: &JsDefaultClause, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled inside of `fmt_fields`
        Ok(())
    }
}
