use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::JsLabels;
use biome_formatter::{format_args, write};
use biome_js_syntax::{
    AnyJsAssignment, AnyJsAssignmentPattern, AnyJsComputedMember, AnyJsExpression, AnyJsName,
    JsAssignmentExpression, JsInitializerClause, JsStaticMemberAssignment,
    JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsStaticMemberExpression;

impl FormatNodeRule<JsStaticMemberExpression> for FormatJsStaticMemberExpression {
    fn fmt_fields(&self, node: &JsStaticMemberExpression, f: &mut JsFormatter) -> FormatResult<()> {
        AnyJsStaticMemberLike::from(node.clone()).fmt(f)
    }

    fn needs_parentheses(&self, item: &JsStaticMemberExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Copy, Clone)]
enum StaticMemberLikeLayout {
    /// Forces that there's no line break between the object, operator, and member
    NoBreak,

    /// Breaks the static member expression after the object if the whole expression doesn't fit on a single line
    BreakAfterObject,
}

declare_node_union! {
    pub(crate) AnyJsStaticMemberLike = JsStaticMemberExpression | JsStaticMemberAssignment
}

impl Format<JsFormatContext> for AnyJsStaticMemberLike {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let is_member_chain = {
            let mut recording = f.start_recording();
            write!(recording, [self.object().format()])?;

            recording
                .stop()
                .has_label(LabelId::of(JsLabels::MemberChain))
        };

        let layout = self.layout(is_member_chain)?;

        match layout {
            StaticMemberLikeLayout::NoBreak => {
                let format_no_break = format_with(|f| {
                    write!(f, [self.operator_token().format(), self.member().format()])
                });

                if is_member_chain {
                    write!(
                        f,
                        [labelled(
                            LabelId::of(JsLabels::MemberChain),
                            &format_no_break
                        )]
                    )
                } else {
                    write!(f, [format_no_break])
                }
            }
            StaticMemberLikeLayout::BreakAfterObject => {
                write!(
                    f,
                    [group(&indent(&format_args![
                        soft_line_break(),
                        self.operator_token().format(),
                        self.member().format(),
                    ]))]
                )
            }
        }
    }
}

impl AnyJsStaticMemberLike {
    fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsStaticMemberLike::JsStaticMemberExpression(expression) => expression.object(),
            AnyJsStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.object(),
        }
    }

    fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsStaticMemberLike::JsStaticMemberExpression(expression) => {
                expression.operator_token()
            }
            AnyJsStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.dot_token(),
        }
    }

    fn member(&self) -> SyntaxResult<AnyJsName> {
        match self {
            AnyJsStaticMemberLike::JsStaticMemberExpression(expression) => expression.member(),
            AnyJsStaticMemberLike::JsStaticMemberAssignment(assignment) => assignment.member(),
        }
    }

    fn layout(&self, is_member_chain: bool) -> SyntaxResult<StaticMemberLikeLayout> {
        let parent = self.syntax().parent();
        let object = self.object()?;

        let is_nested = match &parent {
            Some(parent) => {
                if JsAssignmentExpression::can_cast(parent.kind())
                    || JsInitializerClause::can_cast(parent.kind())
                {
                    let no_break = match &object {
                        AnyJsExpression::JsCallExpression(call_expression) => {
                            !call_expression.arguments()?.args().is_empty()
                        }
                        AnyJsExpression::TsNonNullAssertionExpression(non_null_assertion) => {
                            match non_null_assertion.expression()? {
                                AnyJsExpression::JsCallExpression(call_expression) => {
                                    !call_expression.arguments()?.args().is_empty()
                                }
                                _ => false,
                            }
                        }
                        _ => false,
                    };

                    if no_break || is_member_chain {
                        return Ok(StaticMemberLikeLayout::NoBreak);
                    }
                }

                AnyJsStaticMemberLike::can_cast(parent.kind())
                    || AnyJsComputedMember::can_cast(parent.kind())
            }
            None => false,
        };

        if !is_nested && matches!(&object, AnyJsExpression::JsIdentifierExpression(_)) {
            return Ok(StaticMemberLikeLayout::NoBreak);
        }

        let first_non_static_member_ancestor = self.syntax().ancestors().find(|parent| {
            !(AnyJsStaticMemberLike::can_cast(parent.kind())
                || AnyJsComputedMember::can_cast(parent.kind()))
        });

        let layout = match first_non_static_member_ancestor.and_then(AnyJsExpression::cast) {
            Some(AnyJsExpression::JsNewExpression(_)) => StaticMemberLikeLayout::NoBreak,
            Some(AnyJsExpression::JsAssignmentExpression(assignment)) => {
                if matches!(
                    assignment.left()?,
                    AnyJsAssignmentPattern::AnyJsAssignment(
                        AnyJsAssignment::JsIdentifierAssignment(_)
                    )
                ) {
                    StaticMemberLikeLayout::BreakAfterObject
                } else {
                    StaticMemberLikeLayout::NoBreak
                }
            }
            _ => StaticMemberLikeLayout::BreakAfterObject,
        };

        Ok(layout)
    }
}

impl NeedsParentheses for JsStaticMemberExpression {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        if matches!(parent.kind(), JsSyntaxKind::JS_NEW_EXPRESSION) && self.is_optional_chain() {
            return true;
        }

        member_chain_callee_needs_parens(self.clone().into(), parent)
    }
}

pub(crate) fn member_chain_callee_needs_parens(
    node: AnyJsExpression,
    parent: &JsSyntaxNode,
) -> bool {
    use AnyJsExpression::*;

    match parent.kind() {
        // `new (test().a)
        JsSyntaxKind::JS_NEW_EXPRESSION => {
            let mut object_chain =
                std::iter::successors(Some(node), |expression| match expression {
                    JsStaticMemberExpression(member) => member.object().ok(),
                    JsComputedMemberExpression(member) => member.object().ok(),
                    JsTemplateExpression(template) => template.tag(),
                    TsNonNullAssertionExpression(assertion) => assertion.expression().ok(),
                    _ => None,
                });

            object_chain.any(|object| matches!(object, JsCallExpression(_)))
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::JsStaticMemberExpression;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("new (test().a)()", JsStaticMemberExpression);
        assert_needs_parentheses!("new (test()[a].b)()", JsStaticMemberExpression);
        assert_needs_parentheses!("new (test()`template`.length)()", JsStaticMemberExpression);
        assert_needs_parentheses!("new (test()!.member)()", JsStaticMemberExpression);

        assert_not_needs_parentheses!("new (test.a)()", JsStaticMemberExpression);
    }
}
