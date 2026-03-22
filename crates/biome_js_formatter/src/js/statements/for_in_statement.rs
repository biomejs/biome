use crate::prelude::*;

use crate::utils::FormatStatementBody;
use biome_formatter::write;
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

        let body = body?;
        let should_insert_space = f.options().delimiter_spacing().value();

        let format_inner = format_with(|f| {
            write!(f, [for_token.format(), space(), l_paren_token.format()])?;

            if should_insert_space {
                write!(f, [space()])?;
            }

            write!(
                f,
                [
                    initializer.format(),
                    space(),
                    in_token.format(),
                    space(),
                    expression.format(),
                ]
            )?;

            if should_insert_space {
                write!(f, [space()])?;
            }

            write!(f, [r_paren_token.format(), FormatStatementBody::new(&body)])
        });

        write!(f, [group(&format_inner)])
    }
}
