use crate::prelude::*;
use biome_css_syntax::{CssEmptyDeclaration, CssEmptyDeclarationFields, CssSyntaxKind};
use biome_formatter::write;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssEmptyDeclaration;
impl FormatNodeRule<CssEmptyDeclaration> for FormatCssEmptyDeclaration {
    fn fmt_fields(&self, node: &CssEmptyDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssEmptyDeclarationFields { semicolon_token } = node.as_fields();

        // The semicolon after a metavariable is kept, because the metavariable
        // may stand for a declaration without its semicolon:
        //
        // ```css
        // ${(props) => props.active && "color: red"};
        // ```
        let is_after_metavariable = node
            .syntax()
            .prev_sibling()
            .is_some_and(|sibling| sibling.kind() == CssSyntaxKind::CSS_METAVARIABLE);
        if is_after_metavariable {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }
}
