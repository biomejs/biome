use crate::prelude::*;
use biome_css_syntax::{CssSyntaxKind, CssTypeFunction, CssTypeFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssTypeFunction;

impl FormatNodeRule<CssTypeFunction> for FormatCssTypeFunction {
    fn fmt_fields(&self, node: &CssTypeFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let is_css_type = node.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                CssSyntaxKind::CSS_ATTR_FUNCTION
                    | CssSyntaxKind::CSS_FUNCTION_PARAMETER
                    | CssSyntaxKind::CSS_RETURNS_STATEMENT
            )
        });
        if !is_css_type {
            // Standalone Sass type() values are unquoted strings: whitespace
            // changes can alter equality and string operations before CSS emission.
            return format_css_verbatim_range(
                node.syntax(),
                node.syntax().text_trimmed_range(),
                |_, _, _| Ok(CssVerbatimTokenFormat::Source),
            )
            .fmt(f);
        }

        let CssTypeFunctionFields {
            name_token,
            l_paren_token,
            ty,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format()?.with_text_case(CssCase::Preserve),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent(&ty.format()),
                    r_paren_token.format(),
                ])
            ]
        )
    }
}
