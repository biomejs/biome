use crate::prelude::*;
use crate::utils::FormatStatementSemicolon;
use biome_formatter::write;

use biome_js_syntax::JsExportNamedFromClause;
use biome_js_syntax::JsExportNamedFromClauseFields;
use biome_rowan::{AstNode, AstSeparatedElement};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsExportNamedFromClause;

impl FormatNodeRule<JsExportNamedFromClause> for FormatJsExportNamedFromClause {
    fn fmt_fields(&self, node: &JsExportNamedFromClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedFromClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();
        let should_insert_space_around_brackets = f.options().bracket_spacing().value();

        if let Some(type_token) = &type_token {
            write!(f, [type_token.format(), space()])?;
        }

        write!(f, [l_curly_token.format(),])?;

        match specifiers.elements().next() {
            Some(AstSeparatedElement {
                node: Ok(node),
                trailing_separator: Ok(separator),
            }) if specifiers.len() == 1 && !f.comments().has_comments(node.syntax()) => {
                write!(
                    f,
                    [
                        maybe_space(should_insert_space_around_brackets),
                        node.format()
                    ]
                )?;

                if let Some(separator) = separator {
                    write!(f, [format_removed(&separator)])?;
                }

                write!(f, [maybe_space(should_insert_space_around_brackets)])?;
            }
            _ => {
                if specifiers.syntax().has_leading_newline() {
                    write!(f, [block_indent(&specifiers.format()),])?;
                } else {
                    write!(
                        f,
                        [group(&soft_block_indent_with_maybe_space(
                            &specifiers.format(),
                            should_insert_space_around_brackets
                        )),]
                    )?;
                };
            }
        }

        write![
            f,
            [
                r_curly_token.format(),
                space(),
                from_token.format(),
                space(),
                source.format(),
            ]
        ]?;

        if let Some(assertion) = &assertion {
            write!(f, [assertion.format()])?;
        }

        write!(f, [FormatStatementSemicolon::new(semicolon_token.as_ref())])
    }
}
