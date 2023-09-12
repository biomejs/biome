use crate::prelude::*;

use crate::utils::FormatStatementBody;
use biome_formatter::{format_args, write};
use biome_js_syntax::JsForInStatement;
use biome_js_syntax::JsForInStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsForInStatement;

impl FormatNodeRule<JsForInStatement> for FormatJsForInStatement {
    fn fmt_fields(&self, node: &JsForInStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForInStatementFields {
            for_token,
            l_paren_token,
            initializer,
            in_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let for_token = for_token.format();
        let initializer = initializer.format();
        let in_token = in_token.format();
        let expression = expression.format();

        write!(
            f,
            [group(&format_args!(
                for_token,
                space(),
                l_paren_token.format(),
                initializer,
                space(),
                in_token,
                space(),
                expression,
                r_paren_token.format(),
                FormatStatementBody::new(&body?)
            ))]
        )
    }
}
