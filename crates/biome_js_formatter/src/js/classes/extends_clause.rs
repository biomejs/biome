use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::JsExtendsClause;
use biome_js_syntax::JsExtendsClauseFields;
use biome_js_syntax::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExtendsClause;

impl FormatNodeRule<JsExtendsClause> for FormatJsExtendsClause {
    fn fmt_fields(&self, node: &JsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = node.as_fields();

        let super_class = super_class?;

        let format_super = format_with(|f| {
            let content =
                format_with(|f| write!(f, [super_class.format(), type_arguments.format()]));

            let comments = f.comments();
            let has_trailing_comments = if let Some(type_arguments) = &type_arguments {
                comments.has_trailing_comments(type_arguments.syntax())
            } else {
                comments.has_trailing_comments(super_class.syntax())
            };

            if node
                .syntax()
                .grand_parent()
                .is_some_and(|p| p.kind() == JS_ASSIGNMENT_EXPRESSION)
            {
                if comments.has_leading_comments(super_class.syntax()) || has_trailing_comments {
                    write!(f, [text("("), &content, text(")")])
                } else {
                    let content = content.memoized();
                    write!(
                        f,
                        [
                            if_group_breaks(&format_args![
                                text("("),
                                &soft_block_indent(&content),
                                text(")"),
                            ]),
                            if_group_fits_on_line(&content)
                        ]
                    )
                }
            } else {
                content.fmt(f)
            }
        });

        write![f, [extends_token.format(), space(), group(&format_super)]]
    }
}
