use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::JsLabeledStatementFields;
use biome_js_syntax::{AnyJsStatement, JsLabeledStatement};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsLabeledStatement;

impl FormatNodeRule<JsLabeledStatement> for FormatJsLabeledStatement {
    fn fmt_fields(&self, node: &JsLabeledStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsLabeledStatementFields {
            label,
            colon_token,
            body,
        } = node.as_fields();

        write!(f, [label.format(), colon_token.format()])?;

        match body? {
            AnyJsStatement::JsEmptyStatement(empty) => {
                // If the body is an empty statement, force semicolon insertion
                write!(f, [empty.format(), text(";")])
            }
            body => {
                write!(f, [space(), body.format()])
            }
        }
    }
}
