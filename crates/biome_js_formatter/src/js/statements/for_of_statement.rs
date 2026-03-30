use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::JsForOfStatement;

use crate::utils::FormatStatementBody;
use biome_js_syntax::JsForOfStatementFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsForOfStatement;

impl FormatNodeRule<JsForOfStatement> for FormatJsForOfStatement {
    fn fmt_fields(&self, node: &JsForOfStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsForOfStatementFields {
            for_token,
            await_token,
            l_paren_token,
            initializer,
            of_token,
            expression,
            r_paren_token,
            body,
        } = node.as_fields();

        let body = body?;

        let should_insert_space_around_delimiters = f.options().delimiter_spacing().value();
        let format_inner = format_with(|f| {
            write!(f, [for_token.format()])?;

            if let Some(await_token) = await_token.as_ref() {
                write!(f, [space(), await_token.format()])?;
            }

            write!(f, [space(), l_paren_token.format()])?;

            if should_insert_space_around_delimiters {
                write!(f, [space()])?;
            }

            write!(
                f,
                [
                    initializer.format(),
                    space(),
                    of_token.format(),
                    space(),
                    expression.format(),
                ]
            )?;

            if should_insert_space_around_delimiters {
                write!(f, [space()])?;
            }

            write!(f, [r_paren_token.format(), FormatStatementBody::new(&body)])
        });

        write!(f, [group(&format_inner)])
    }
}
