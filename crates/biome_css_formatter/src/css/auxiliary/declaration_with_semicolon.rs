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
