use crate::prelude::*;
use biome_css_syntax::{
    CssEmptyDeclaration, CssEmptyDeclarationFields, CssSyntaxKind, ScssInterpolation, T,
};
use biome_formatter::write;
use biome_rowan::Direction;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssEmptyDeclaration;
impl FormatNodeRule<CssEmptyDeclaration> for FormatCssEmptyDeclaration {
    fn fmt_fields(&self, node: &CssEmptyDeclaration, f: &mut CssFormatter) -> FormatResult<()> {
        let CssEmptyDeclarationFields { semicolon_token } = node.as_fields();
        let is_statement_list = node.syntax().parent().is_some_and(|parent| {
            matches!(
                parent.kind(),
                CssSyntaxKind::CSS_ROOT_ITEM_LIST
                    | CssSyntaxKind::CSS_RULE_LIST
                    | CssSyntaxKind::CSS_KEYFRAMES_ITEM_LIST
                    | CssSyntaxKind::CSS_FONT_FEATURE_VALUES_ITEM_LIST
            )
        });
        // Removing these delimiters could absorb the next rule into a missing
        // body or reconnect an `@else` separated from its `@if` by a semicolon.
        let preserve_delimiter = is_statement_list
            && node.syntax().prev_sibling().is_some_and(|previous| {
                let is_unterminated = previous.last_token().is_some_and(|token| {
                    !matches!(token.kind(), T![;] | T!['}'])
                        || token
                            .parent()
                            .is_some_and(|parent| ScssInterpolation::can_cast(parent.kind()))
                });
                let separates_else = previous.kind() != CssSyntaxKind::CSS_EMPTY_DECLARATION
                    && node
                        .syntax()
                        .siblings(Direction::Next)
                        .skip(1)
                        .find(|next| next.kind() != CssSyntaxKind::CSS_EMPTY_DECLARATION)
                        .and_then(|next| next.first_token())
                        .filter(|token| token.kind() == T![@])
                        .and_then(|token| token.next_token())
                        .is_some_and(|token| token.kind() == T![else]);
                is_unterminated || separates_else
            });

        if preserve_delimiter {
            write!(f, [semicolon_token.format()])
        } else {
            write!(f, [format_removed(&semicolon_token?)])
        }
    }
}
