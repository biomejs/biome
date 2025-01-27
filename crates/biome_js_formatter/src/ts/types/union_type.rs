use crate::prelude::*;
use crate::utils::{should_hug_type, FormatTypeMemberSeparator};

use biome_formatter::{format_args, write, Buffer};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsUnionTypeFields;
use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken, TsTupleTypeElementList, TsUnionType};
use biome_rowan::SyntaxNodeOptionExt;

#[derive(Debug, Clone, Default)]
pub struct FormatTsUnionType;

impl FormatNodeRule<TsUnionType> for FormatTsUnionType {
    // [Prettier applies]: https://github.com/prettier/prettier/blob/cd3e530c2e51fb8296c0fb7738a9afdd3a3a4410/src/language-js/print/type-annotation.js#L123-L202
    fn fmt_fields(&self, node: &TsUnionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsUnionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();

        // ```ts
        // {
        //   a: string
        // } | null | void
        // ```
        // should be inlined and not be printed in the multi-line variant
        let should_hug = should_hug_type(&node.clone().into(), f);
        if should_hug {
            return write!(
                f,
                [
                    FormatTypeMemberSeparator::new(leading_separator_token.as_ref()),
                    types.format().with_options(should_hug)
                ]
            );
        }

        // Find the head of the nest union type chain
        // ```js
        // type Foo = | (| (A | B))
        //                  ^^^^^
        // ```
        // If the current union type is `A | B`
        // - `A | B` is the inner union type of `| (A | B)`
        // - `| (A | B)` is the inner union type of `| (| (A | B))`
        //
        // So the head of the current nested union type chain is `| (| (A | B))`
        // if we encounter a leading comment when navigating up the chain,
        // we consider the current union type as having leading comments
        let mut has_leading_comments = f.comments().has_leading_comments(node.syntax());
        let mut union_type_at_top = node.clone();
        while let Some(grand_parent) = union_type_at_top
            .syntax()
            .grand_parent()
            .and_then(TsUnionType::cast)
        {
            if grand_parent.types().len() == 1 {
                if f.comments().has_leading_comments(grand_parent.syntax()) {
                    has_leading_comments = true;
                }
                union_type_at_top = grand_parent;
            } else {
                break;
            }
        }

        let should_indent = {
            let parent_kind = union_type_at_top.syntax().parent().kind();

            // These parents have indent for their content, so we don't need to indent here
            !match parent_kind {
                Some(JsSyntaxKind::TS_TYPE_ALIAS_DECLARATION) => has_leading_comments,
                parent_kind => {
                    matches!(
                        parent_kind,
                        Some(
                            JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
                                | JsSyntaxKind::TS_TUPLE_TYPE_ELEMENT_LIST
                                | JsSyntaxKind::TS_TYPE_ASSERTION_ASSIGNMENT
                                | JsSyntaxKind::TS_TYPE_ARGUMENT_LIST
                        )
                    )
                }
            }
        };

        let types = format_with(|f| {
            if has_leading_comments {
                write!(f, [soft_line_break()])?;
            }

            write!(
                f,
                [
                    FormatTypeSetLeadingSeparator {
                        separator: "|",
                        leading_separator: leading_separator_token.as_ref(),
                        leading_soft_line_break_or_space: should_indent && !has_leading_comments,
                    },
                    types.format()
                ]
            )
        });

        let content = format_with(|f| {
            // it is necessary to add parentheses for unions in intersections
            // ```ts
            // type Some = B & (C | A) & D
            // ```
            if node.needs_parentheses() {
                return write!(f, [indent(&types), soft_line_break()]);
            }

            let is_inside_complex_tuple_type = node
                .parent::<TsTupleTypeElementList>()
                .is_some_and(|tuple| tuple.len() > 1);

            if is_inside_complex_tuple_type {
                write!(
                    f,
                    [
                        indent(&format_args![
                            if_group_breaks(&format_args![text("("), soft_line_break()]),
                            types
                        ]),
                        soft_line_break(),
                        if_group_breaks(&text(")"))
                    ]
                )
            } else if should_indent {
                write!(f, [indent(&types)])
            } else {
                write!(f, [types])
            }
        });

        write!(f, [group(&content)])
    }

    fn needs_parentheses(&self, item: &TsUnionType) -> bool {
        item.needs_parentheses()
    }

    fn is_suppressed(&self, node: &TsUnionType, f: &JsFormatter) -> bool {
        f.comments().mark_suppression_checked(node.syntax());

        if node.types().is_empty() {
            f.comments().is_suppressed(node.syntax())
        } else {
            // Suppression applies to first variant
            false
        }
    }
}

pub struct FormatTypeSetLeadingSeparator<'a> {
    separator: &'static str,
    leading_separator: Option<&'a JsSyntaxToken>,
    leading_soft_line_break_or_space: bool,
}

impl Format<JsFormatContext> for FormatTypeSetLeadingSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match &self.leading_separator {
            Some(token) => {
                let content = format_with(|f| {
                    if self.leading_soft_line_break_or_space {
                        write!(f, [soft_line_break_or_space()])?;
                    }
                    write!(f, [token.format(), space()])
                });
                format_only_if_breaks(token, &content).fmt(f)
            }
            None => {
                let content = format_with(|f| {
                    if self.leading_soft_line_break_or_space {
                        write!(f, [soft_line_break_or_space()])?;
                    }
                    write!(f, [text(self.separator), space()])
                });

                write!(f, [if_group_breaks(&content)])
            }
        }
    }
}
