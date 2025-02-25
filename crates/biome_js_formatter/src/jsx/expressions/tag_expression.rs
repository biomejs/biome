use crate::prelude::*;
use crate::utils::jsx::{get_wrap_state, WrapState};

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{
    JsArrowFunctionExpression, JsCallArgumentList, JsCallExpression, JsxExpressionChild,
    JsxTagExpression,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxTagExpression;

impl FormatNodeRule<JsxTagExpression> for FormatJsxTagExpression {
    fn fmt_fields(&self, node: &JsxTagExpression, f: &mut JsFormatter) -> FormatResult<()> {
        let wrap = get_wrap_state(node);

        match wrap {
            WrapState::NoWrap => {
                write![
                    f,
                    [
                        format_leading_comments(node.syntax()),
                        node.tag().format(),
                        format_trailing_comments(node.syntax())
                    ]
                ]
            }
            WrapState::WrapOnBreak => {
                let should_expand = should_expand(node);
                let needs_parentheses = node.needs_parentheses();

                let format_inner = format_with(|f| {
                    if !needs_parentheses {
                        write!(f, [if_group_breaks(&text("("))])?;
                    }

                    write!(
                        f,
                        [soft_block_indent(&format_args![
                            format_leading_comments(node.syntax()),
                            node.tag().format(),
                            format_trailing_comments(node.syntax())
                        ])]
                    )?;

                    if !needs_parentheses {
                        write!(f, [if_group_breaks(&text(")"))])?;
                    }

                    Ok(())
                });

                write!(f, [group(&format_inner).should_expand(should_expand)])
            }
        }
    }

    fn needs_parentheses(&self, item: &JsxTagExpression) -> bool {
        item.needs_parentheses()
    }

    fn fmt_leading_comments(&self, _: &JsxTagExpression, _: &mut JsFormatter) -> FormatResult<()> {
        // Handled as part of `fmt_fields`
        Ok(())
    }

    fn fmt_trailing_comments(&self, _: &JsxTagExpression, _: &mut JsFormatter) -> FormatResult<()> {
        // handled as part of `fmt_fields`
        Ok(())
    }
}

/// This is a very special situation where we're returning a JsxElement
/// from an arrow function that's passed as an argument to a function,
/// which is itself inside a JSX expression child.
///
/// If you're wondering why this is the only other case, it's because
/// Prettier defines it to be that way.
///
/// ```jsx
///  let bar = <div>
///    {foo(() => <div> the quick brown fox jumps over the lazy dog </div>)}
///  </div>;
/// ```
pub fn should_expand(expression: &JsxTagExpression) -> bool {
    let arrow = match expression.syntax().parent() {
        Some(parent) if JsArrowFunctionExpression::can_cast(parent.kind()) => parent,
        _ => return false,
    };

    let call = match arrow.parent() {
        // Argument
        Some(grand_parent) if JsCallArgumentList::can_cast(grand_parent.kind()) => {
            let maybe_call_expression = grand_parent.grand_parent();

            match maybe_call_expression {
                Some(call) if JsCallExpression::can_cast(call.kind()) => call,
                _ => return false,
            }
        }
        // Callee
        Some(grand_parent) if JsCallExpression::can_cast(grand_parent.kind()) => grand_parent,
        _ => return false,
    };

    call.parent()
        .is_some_and(|parent| JsxExpressionChild::can_cast(parent.kind()))
}
