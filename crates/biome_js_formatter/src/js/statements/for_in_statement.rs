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

        let l_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [l_paren_token.format(), space()])
            } else {
                write!(f, [l_paren_token.format()])
            }
        });

        let r_paren = format_with(|f: &mut JsFormatter| {
            if f.options().delimiter_spacing().value() {
                write!(f, [space(), r_paren_token.format()])
            } else {
                write!(f, [r_paren_token.format()])
            }
        });

        write!(
            f,
            [group(&format_args!(
                for_token.format(),
                space(),
                l_paren,
                initializer.format(),
                space(),
                in_token.format(),
                space(),
                expression.format(),
                r_paren,
                FormatStatementBody::new(&body?)
            ))]
        )
    }
}
