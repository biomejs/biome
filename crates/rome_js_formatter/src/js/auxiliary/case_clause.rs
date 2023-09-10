use crate::prelude::*;
use biome_rowan::AstNodeList;
use rome_formatter::{format_args, write};
use rome_js_syntax::AnyJsStatement;
use rome_js_syntax::JsCaseClause;
use rome_js_syntax::JsCaseClauseFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsCaseClause;

impl FormatNodeRule<JsCaseClause> for FormatJsCaseClause {
    fn fmt_fields(&self, node: &JsCaseClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsCaseClauseFields {
            case_token,
            test,
            colon_token,
            consequent,
        } = node.as_fields();

        write!(
            f,
            [
                case_token.format(),
                space(),
                test.format(),
                colon_token.format()
            ]
        )?;

        let is_first_child_block_stmt = matches!(
            consequent.iter().next(),
            Some(AnyJsStatement::JsBlockStatement(_))
        );

        if consequent.is_empty() {
            // Skip inserting an indent block is the consequent is empty to print
            // the trailing comments for the case clause inline if there is no
            // block to push them into
            write!(f, [hard_line_break()])
        } else if is_first_child_block_stmt {
            write![f, [space(), consequent.format()]]
        } else {
            // no line break needed after because it is added by the indent in the switch statement
            write!(
                f,
                [indent(&format_args![
                    hard_line_break(),
                    consequent.format()
                ])]
            )
        }
    }
}
