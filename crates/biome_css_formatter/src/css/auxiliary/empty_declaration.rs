use crate::prelude::*;
use biome_css_syntax::{CssEmptyDeclaration, CssEmptyDeclarationFields, CssSyntaxKind};
use biome_formatter::write;
use biome_rowan::{AstNode, SyntaxNodeOptionExt};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssEmptyDeclaration;
impl FormatNodeRule<CssEmptyDeclaration> for FormatCssEmptyDeclaration {
    fn fmt_fields(&self, node: &CssEmptyDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssEmptyDeclarationFields { semicolon_token } = node.as_fields();
        let parent_kind = node.syntax().parent().kind();

        if matches!(
            parent_kind,
            Some(CssSyntaxKind::CSS_DECLARATION_WITH_SEMICOLON,)
        ) {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }
}
