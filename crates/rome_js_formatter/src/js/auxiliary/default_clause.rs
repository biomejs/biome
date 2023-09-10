use crate::prelude::*;
use biome_rowan::AstNodeList;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsDefaultClause;
use rome_js_syntax::{AnyJsStatement, JsDefaultClauseFields};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsDefaultClause;

impl FormatNodeRule<JsDefaultClause> for FormatJsDefaultClause {
    fn fmt_fields(&self, node: &JsDefaultClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDefaultClauseFields {
            default_token,
            colon_token,
            consequent,
        } = node.as_fields();

        let first_child_is_block_stmt = matches!(
            consequent.iter().next(),
            Some(AnyJsStatement::JsBlockStatement(_))
        );

        write!(f, [default_token.format(), colon_token.format()])?;

        if f.comments().has_dangling_comments(node.syntax()) {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        if consequent.is_empty() {
            write!(f, [hard_line_break()])
        } else if first_child_is_block_stmt {
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
