use crate::prelude::*;
use biome_formatter::write;

use biome_js_syntax::{JsSyntaxKind, JsVariableDeclaratorList};
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsVariableDeclaratorList;

impl FormatRule<JsVariableDeclaratorList> for FormatJsVariableDeclaratorList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsVariableDeclaratorList, f: &mut JsFormatter) -> FormatResult<()> {
        let length = node.len();

        let is_parent_for_loop = node.syntax().grand_parent().is_some_and(|grand_parent| {
            matches!(
                grand_parent.kind(),
                JsSyntaxKind::JS_FOR_STATEMENT
                    | JsSyntaxKind::JS_FOR_OF_STATEMENT
                    | JsSyntaxKind::JS_FOR_IN_STATEMENT
            )
        });

        let has_any_initializer = node.iter().any(|declarator| {
            declarator.is_ok_and(|declarator| declarator.initializer().is_some())
        });

        let format_separator = format_with(|f| {
            if !is_parent_for_loop && has_any_initializer {
                write!(f, [hard_line_break()])
            } else {
                write!(f, [soft_line_break_or_space()])
            }
        });

        let mut declarators = node.iter().zip(
            node.format_separated(",")
                .with_trailing_separator(TrailingSeparator::Disallowed),
        );

        let (first_declarator, format_first_declarator) = match declarators.next() {
            Some((syntax, format_first_declarator)) => (syntax?, format_first_declarator),
            None => return Err(FormatError::SyntaxError),
        };

        if length == 1 && !f.comments().has_leading_comments(first_declarator.syntax()) {
            return write!(f, [format_first_declarator]);
        }

        write!(
            f,
            [indent(&format_once(|f| {
                write!(f, [format_first_declarator])?;

                if length > 1 {
                    write!(f, [format_separator])?;
                }

                f.join_with(&format_separator)
                    .entries(declarators.map(|(_, format)| format))
                    .finish()
            }))]
        )
    }
}
