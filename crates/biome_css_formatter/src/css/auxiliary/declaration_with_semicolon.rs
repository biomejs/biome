use crate::prelude::*;
use crate::utils::comment_trivia::has_block_comment_gap_before_token;
use biome_css_syntax::{CssDeclarationWithSemicolon, CssDeclarationWithSemicolonFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssDeclarationWithSemicolon;
impl FormatNodeRule<CssDeclarationWithSemicolon> for FormatCssDeclarationWithSemicolon {
    fn fmt_fields(
        &self,
        node: &CssDeclarationWithSemicolon,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssDeclarationWithSemicolonFields {
            declaration,
            semicolon_token,
        } = node.as_fields();
        // Keep the pre-`;` gap only for `!important` block comments:
        // `a { color: red !important /* c */ ; }`.
        // Plain declarations normalize: `a { --token: /* c */ ; }` -> `a { --token: /* c */; }`.
        let preserve_source_gap_before_semicolon = declaration
            .as_ref()
            .is_ok_and(|declaration| declaration.important().is_some());

        write!(f, [declaration.format()])?;

        // The last declaration of a `style` attribute only needs its semicolon
        // once the attribute breaks across lines. Kept on one line, a trailing
        // `;` is noise the author did not write.
        if f.options().is_html_style_attribute() && node.syntax().next_sibling().is_none() {
            if let Some(semicolon) = semicolon_token.as_ref() {
                write!(f, [format_removed(semicolon)])?;
            }
            return write!(f, [if_group_breaks(&token(";"))]);
        }

        match semicolon_token.as_ref() {
            Some(semicolon) => {
                if preserve_source_gap_before_semicolon
                    && has_block_comment_gap_before_token(semicolon)
                {
                    write!(f, [space()])?;
                }

                write!(f, [semicolon.format()])
            }
            None => write!(f, [token(";")]),
        }
    }
}
