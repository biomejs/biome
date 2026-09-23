use crate::js::expressions::call_arguments::FormatJsCallArgumentsOptions;
use crate::js::expressions::computed_member_expression::FormatComputedMemberLookup;
use crate::prelude::*;
use biome_formatter::write;
use biome_js_syntax::{
    JsCallExpression, JsCallExpressionFields, JsComputedMemberExpression, JsImportCallExpression,
    JsStaticMemberExpression, JsStaticMemberExpressionFields, JsSyntaxNode,
    TsNonNullAssertionExpression, TsNonNullAssertionExpressionFields,
};
use biome_rowan::AstNode;
use std::fmt::Debug;

#[derive(Copy, Clone, Debug)]
pub(crate) enum CallExpressionPosition {
    /// At the start of a call chain.
    /// `of` in `of().test`
    Start,

    /// Somewhere in the middle.
    ///
    /// `b` in `a.b().c()`
    Middle,

    /// At the end of a call chain (root)
    /// `c` in `a.b.c()`
    End,
}

/// Data structure that holds the node with its formatted version
#[derive(Clone, Debug)]
pub(crate) enum ChainMember {
    /// Holds onto a [biome_js_syntax::JsStaticMemberExpression]
    StaticMember {
        expression: JsStaticMemberExpression,
    },

    /// Holds onto a [biome_js_syntax::JsCallExpression]
    CallExpression {
        expression: JsCallExpression,
        position: CallExpressionPosition,
    },

    /// Holds onto a [biome_js_syntax::JsComputedMemberExpression]
    ComputedMember {
        expression: JsComputedMemberExpression,
    },

    TsNonNullAssertionExpression {
        expression: TsNonNullAssertionExpression,
    },

    /// Any other node that are not  [biome_js_syntax::JsCallExpression] or [biome_js_syntax::JsStaticMemberExpression]
    /// Are tracked using this variant
    Node(JsSyntaxNode),
}

impl ChainMember {
    /// checks if the current node is a [biome_js_syntax::JsCallExpression], or a [biome_js_syntax::JsImportExpression]
    pub fn is_call_like_expression(&self) -> bool {
        match self {
            Self::CallExpression { .. } => true,
            Self::Node(node) => {
                JsImportCallExpression::can_cast(node.kind())
                    | JsCallExpression::can_cast(node.kind())
            }
            _ => false,
        }
    }

    pub(crate) const fn is_call_expression(&self) -> bool {
        matches!(self, Self::CallExpression { .. })
    }

    pub(crate) fn syntax(&self) -> &JsSyntaxNode {
        match self {
            Self::StaticMember { expression, .. } => expression.syntax(),
            Self::CallExpression { expression, .. } => expression.syntax(),
            Self::ComputedMember { expression, .. } => expression.syntax(),
            Self::TsNonNullAssertionExpression { expression } => expression.syntax(),
            Self::Node(node) => node,
        }
    }

    pub const fn is_computed_expression(&self) -> bool {
        matches!(self, Self::ComputedMember { .. })
    }
}

impl Format<JsFormatContext> for ChainMember {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            Self::StaticMember { expression } => {
                let JsStaticMemberExpressionFields {
                    // Formatted as part of the previous item
                    object: _,
                    operator_token,
                    member,
                } = expression.as_fields();

                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        operator_token.format(),
                        member.format(),
                        format_trailing_comments(expression.syntax())
                    ]
                )
            }

            Self::TsNonNullAssertionExpression { expression } => {
                let TsNonNullAssertionExpressionFields {
                    expression: _,
                    excl_token,
                } = expression.as_fields();

                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        excl_token.format(),
                        format_trailing_comments(expression.syntax())
                    ]
                )
            }

            Self::CallExpression {
                expression,
                position,
            } => {
                let JsCallExpressionFields {
                    // Formatted as part of the previous item
                    callee: _,
                    optional_chain_token,
                    type_arguments,
                    arguments,
                } = expression.as_fields();

                match position {
                    CallExpressionPosition::Start => write!(f, [expression.format()]),
                    CallExpressionPosition::Middle => {
                        write!(
                            f,
                            [
                                format_leading_comments(expression.syntax()),
                                optional_chain_token.format(),
                                type_arguments.format(),
                                arguments.format(),
                                format_trailing_comments(expression.syntax())
                            ]
                        )
                    }
                    CallExpressionPosition::End => {
                        write!(
                            f,
                            [
                                optional_chain_token.format(),
                                type_arguments.format(),
                                arguments.format(),
                            ]
                        )
                    }
                }
            }
            Self::ComputedMember { expression } => {
                write!(
                    f,
                    [
                        format_leading_comments(expression.syntax()),
                        FormatComputedMemberLookup::new(&expression.clone().into()),
                        format_trailing_comments(expression.syntax())
                    ]
                )
            }
            Self::Node(node) => {
                write!(f, [node.format()])
            }
        }
    }
}

/// Formats a [`ChainMember`], forcing the last call argument (if the member is a
/// call expression) to be grouped/hugged.
///
/// The member-chain formatter uses this to build the "chain inline, last
/// argument broken" layout, which keeps the chain's break decision independent
/// of whether its final object/array argument already spans multiple lines in
/// the source. See <https://github.com/biomejs/biome/issues/10531>.
pub(crate) struct FormatChainMemberGroupedLastArgument<'a> {
    member: &'a ChainMember,
}

impl ChainMember {
    /// Returns a formatter that hugs this member's last call argument.
    pub(crate) fn format_grouped_last_argument(&self) -> FormatChainMemberGroupedLastArgument<'_> {
        FormatChainMemberGroupedLastArgument { member: self }
    }
}

impl Format<JsFormatContext> for FormatChainMemberGroupedLastArgument<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self.member {
            // The last member of a chain is the root call expression, printed in
            // the `End` position (only the optional chain token, type arguments
            // and arguments are formatted here).
            ChainMember::CallExpression { expression, .. } => {
                let JsCallExpressionFields {
                    // Formatted as part of the previous item
                    callee: _,
                    optional_chain_token,
                    type_arguments,
                    arguments,
                } = expression.as_fields();
                let arguments = arguments?;

                write!(
                    f,
                    [
                        optional_chain_token.format(),
                        type_arguments.format(),
                        arguments
                            .format()
                            .with_options(FormatJsCallArgumentsOptions {
                                force_group_last_argument: true,
                            })
                    ]
                )
            }
            _ => write!(f, [self.member]),
        }
    }
}
