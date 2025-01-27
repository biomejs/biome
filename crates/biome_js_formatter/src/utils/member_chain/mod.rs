//! Utility function that applies some heuristic to format chain member expressions and call expressions
//!
//! We want to transform code that looks like this:
//!
//! ```js
//! something.execute().then().then().catch()
//! ```
//!
//! To something like this:
//!
//! ```js
//! something
//!   .execute()
//!   .then()
//!   .then()
//!   .catch()
//! ```
//!
//! In order to achieve that we use the same heuristic that [Prettier applies](https://github.com/prettier/prettier/blob/main/src/language-js/print/member-chain.js).
//!
//! The process is the following:
//!
//! ### Flattening the AST
//! We flatten the AST. See, the code above is actually nested, where the first member expression (`something`)
//! that we see is actually the last one. This is a oversimplified version of the AST:
//!
//! ```block
//! [
//!     .catch() [
//!         .then() [
//!             .then() [
//!                 .execute() [
//!                     something
//!                 ]
//!             ]
//!         ]
//!     ]
//! ]
//! ```
//! So we need to navigate the AST and make sure that `something` is actually
//! the first one. In a sense, we have to revert the chain of children. We will do that using a recursion.
//!
//! While we navigate the AST and we found particular nodes that we want to track, we also
//! format them. The format of these nodes is different from the standard version.
//!
//! Our formatter makes sure that we don't format twice the same nodes. Let's say for example that
//! we find a `something().execute()`, its AST is like this:
//!
//! ```block
//! JsCallExpression {
//!     callee: JsStaticMember {
//!         object: JsCallExpression {
//!             callee: Reference {
//!                 execute
//!             }
//!         }
//!     }
//! }
//! ```
//!
//! When we track the first [biome_js_syntax::JsCallExpression], we hold basically all the children,
//! that applies for the rest of the nodes. If we decided to format all the children of each node,
//! we will risk to format the last node, the `Reference`, four times.
//!
//! To avoid this, when we encounter particular nodes, we don't format all of its children, but defer
//! the formatting to the child itself.
//!
//! The end result of the flattening, will create an array of something like this:
//!
//! ```block
//! [ Identifier, JsCallExpression, JsStaticMember, JsCallExpression ]
//! ```
//!
//! ### Grouping
//!
//! After the flattening, we start the grouping. We want to group nodes in a way that will help us
//! to apply a deterministic formatting.
//! - first group will be the identifier
//! - the rest of the groups will be  will start StaticMemberExpression followed by the rest of the nodes,
//!     right before the end of the next StaticMemberExpression
//!
//! The first group is special, because it holds the reference; it has its own heuristic.
//! Inside the first group we store the first element of the flattened array, then:
//!
//! 1. as many as [biome_js_syntax::JsCallExpression] we can find, this cover cases like `something()()().then()`;
//! 2. as many as [biome_js_syntax::JsComputedMemberExpression] we can find, this cover cases like `something()()[1][3].then()`;
//! 3. as many as consecutive [biome_js_syntax::JsStaticMemberExpression] or [biome_js_syntax::JsComputedMemberExpression], this cover cases like `this.items[0].then()`
//!
//! The rest of the groups are essentially a sequence of `[StaticMemberExpression , CallExpression]`.
//! In order to achieve that, we simply start looping through the rest of the flatten items that we haven't seen.
//!
//! Eventually, we should have something like this:
//!
//! ```block
//! [
//!     [ReferenceIdentifier, CallExpression], // with possible computed expressions in the middle
//!     [StaticMemberExpression, StaticMemberExpression, CallExpression],
//!     [StaticMemberExpression, CallExpression],
//!     [StaticMemberExpression],
//! ]
//! ```
mod chain_member;
mod groups;
mod simple_argument;

use crate::context::TabWidth;
use crate::prelude::*;
use crate::utils::is_long_curried_call;
use crate::utils::member_chain::chain_member::{CallExpressionPosition, ChainMember};
use crate::utils::member_chain::groups::{
    MemberChainGroup, MemberChainGroupsBuilder, TailChainGroups,
};
pub use crate::utils::member_chain::simple_argument::SimpleArgument;
use crate::JsLabels;
use biome_formatter::{write, Buffer};
use biome_js_syntax::{
    AnyJsCallArgument, AnyJsExpression, AnyJsLiteralExpression, JsCallExpression,
    JsIdentifierExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsThisExpression,
};
use biome_rowan::{AstNode, SyntaxResult};
use std::iter::FusedIterator;

#[derive(Debug, Clone)]
pub(crate) struct MemberChain {
    root: JsCallExpression,
    head: MemberChainGroup,
    tail: TailChainGroups,
}

impl MemberChain {
    pub(crate) fn from_call_expression(
        call_expression: JsCallExpression,
        comments: &JsComments,
        tab_width: TabWidth,
    ) -> SyntaxResult<MemberChain> {
        let parent = call_expression.syntax().parent();
        let mut chain_members =
            ChainMembersIterator::new(call_expression.clone().into(), comments).collect::<Vec<_>>();
        chain_members.reverse();

        // as explained before, the first group is particular, so we calculate it
        let (head_group, remaining_members) =
            split_members_into_head_and_remaining_groups(chain_members);

        // `flattened_items` now contains only the nodes that should have a sequence of
        // `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
        let tail_groups = compute_remaining_groups(remaining_members, comments);

        let mut member_chain = MemberChain {
            head: head_group,
            tail: tail_groups,
            root: call_expression,
        };

        // Here we check if the first element of Groups::groups can be moved inside the head.
        // If so, then we extract it and concatenate it together with the head.
        member_chain.maybe_merge_with_first_group(comments, tab_width, parent.as_ref());

        Ok(member_chain)
    }

    /// Here we check if the first group can be merged to the head. If so, then
    /// we move out the first group out of the groups
    fn maybe_merge_with_first_group(
        &mut self,
        comments: &JsComments,
        tab_width: TabWidth,
        parent: Option<&JsSyntaxNode>,
    ) {
        if self.should_merge_tail_with_head(parent, tab_width, comments) {
            let group = self.tail.pop_first().unwrap();
            self.head.extend_members(group.into_members());
        }
    }

    /// This function checks if the current grouping should be merged with the first group.
    fn should_merge_tail_with_head(
        &self,
        parent: Option<&JsSyntaxNode>,
        tab_width: TabWidth,
        comments: &JsComments,
    ) -> bool {
        let first_group = match self.tail.first() {
            None => {
                return false;
            }
            Some(first_group) => first_group,
        };

        let has_comments = first_group
            .members()
            .first()
            .is_some_and(|member| comments.has_comments(member.syntax()));

        if has_comments {
            return false;
        }

        let has_computed_property = first_group
            .members()
            .first()
            .is_some_and(|item| item.is_computed_expression());

        if self.head.members().len() == 1 {
            let only_member = &self.head.members()[0];

            let in_expression_statement =
                parent.is_some_and(|parent| parent.kind() == JsSyntaxKind::JS_EXPRESSION_STATEMENT);

            match only_member {
                ChainMember::Node(node) => {
                    if JsThisExpression::can_cast(node.kind()) {
                        true
                    } else if let Some(identifier) = JsIdentifierExpression::cast_ref(node) {
                        let is_factory = identifier
                            .name()
                            .and_then(|name| name.value_token())
                            .as_ref()
                            .is_ok_and(is_factory);

                        has_computed_property ||
                            is_factory ||
                            // If an identifier has a name that is shorter than the tab with, then we join it with the "head"
                            (in_expression_statement
                                && has_short_name(&identifier, tab_width))
                    } else {
                        false
                    }
                }
                _ => false,
            }
        } else if let Some(ChainMember::StaticMember { expression }) = self.head.members().last() {
            let member = expression.member().ok();

            let is_factory = member
                .as_ref()
                .and_then(|member| member.as_js_name())
                .and_then(|name| name.value_token().ok())
                .as_ref()
                .is_some_and(is_factory);

            has_computed_property || is_factory
        } else {
            false
        }
    }

    /// It tells if the groups should break on multiple lines
    fn groups_should_break(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        let comments = f.comments();
        let node_has_comments =
            self.head.has_comments(comments) || self.tail.has_comments(comments);

        if node_has_comments {
            return Ok(true);
        }

        let mut call_expressions = self
            .members()
            .filter_map(|member| match member {
                ChainMember::CallExpression { expression, .. } => Some(expression),
                _ => None,
            })
            .peekable();

        let mut calls_count = 0u32;
        let mut any_has_function_like_argument = false;
        let mut any_complex_args = false;

        while let Some(call) = call_expressions.next() {
            calls_count += 1;

            if call_expressions.peek().is_some() {
                any_has_function_like_argument =
                    any_has_function_like_argument || has_arrow_or_function_expression_arg(call)
            }

            any_complex_args = any_complex_args || !has_simple_arguments(call);
        }

        if calls_count > 2 && any_complex_args {
            return Ok(true);
        }

        if self.last_call_breaks(f)? && any_has_function_like_argument {
            return Ok(true);
        }

        if !self.tail.is_empty() && self.head.will_break(f)? {
            return Ok(true);
        }

        if self.tail.any_except_last_will_break(f)? {
            return Ok(true);
        }

        let has_empty_line_inside_tail = self
            .tail
            .iter()
            .skip(1)
            .any(|group| group.needs_empty_line_before());

        if has_empty_line_inside_tail {
            return Ok(true);
        }

        Ok(false)
    }

    /// We retrieve all the call expressions inside the group and we check if
    /// their arguments are not simple.
    fn last_call_breaks(&self, f: &mut JsFormatter) -> FormatResult<bool> {
        let last_group = self.last_group();

        if let Some(ChainMember::CallExpression { .. }) = last_group.members().last() {
            last_group.will_break(f)
        } else {
            Ok(false)
        }
    }

    fn last_group(&self) -> &MemberChainGroup {
        self.tail.last().unwrap_or(&self.head)
    }

    /// Returns an iterator over all members in the member chain
    fn members(&self) -> impl DoubleEndedIterator<Item = &ChainMember> {
        self.head.members().iter().chain(self.tail.members())
    }

    fn has_comments(&self, comments: &JsComments) -> bool {
        let mut members = self.members();

        if let Some(first) = members.next() {
            if comments.has_trailing_comments(first.syntax()) {
                return true;
            }
        }

        // Ignore the root member because comments are printed before/after the member chain.
        members.next_back();

        for member in members {
            if comments.has_leading_comments(member.syntax())
                || comments.has_trailing_comments(member.syntax())
            {
                return true;
            }
        }

        false
    }
}

impl Format<JsFormatContext> for MemberChain {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let has_comments = self.has_comments(f.comments());

        let format_one_line = format_with(|f| {
            let mut joiner = f.join();

            joiner.entry(&self.head);
            joiner.entries(self.tail.iter());

            joiner.finish()
        });

        if self.tail.len() <= 1 && !has_comments {
            return if is_long_curried_call(Some(&self.root)) {
                write!(f, [format_one_line])
            } else if self.root.is_test_call_expression()? && self.head.members().len() >= 2 {
                write!(f, [self.head, soft_line_indent_or_space(&self.tail)])
            } else {
                write!(f, [group(&format_one_line)])
            };
        }

        let format_tail = format_with(|f| {
            for group in self.tail.iter() {
                if group.needs_empty_line_before() {
                    write!(f, [empty_line()])?;
                } else {
                    write!(f, [hard_line_break()])?;
                }
                write!(f, [group])?;
            }
            Ok(())
        });

        let format_expanded = format_with(|f| write!(f, [self.head, indent(&group(&format_tail))]));

        let format_content = format_with(|f| {
            if self.groups_should_break(f)? {
                write!(f, [group(&format_expanded)])
            } else {
                let has_empty_line_before_tail = self
                    .tail
                    .first()
                    .is_some_and(|group| group.needs_empty_line_before());

                if has_empty_line_before_tail || self.last_group().will_break(f)? {
                    write!(f, [expand_parent()])?;
                }

                write!(f, [best_fitting!(format_one_line, format_expanded)])
            }
        });

        write!(
            f,
            [labelled(
                LabelId::of(JsLabels::MemberChain),
                &format_content
            )]
        )
    }
}

/// Splits the members into two groups:
/// * The head group that contains all notes that are not a sequence of: `[ StaticMemberExpression -> AnyNode + JsCallExpression ]`
/// * The remaining members
fn split_members_into_head_and_remaining_groups(
    mut members: Vec<ChainMember>,
) -> (MemberChainGroup, Vec<ChainMember>) {
    // This where we apply the first two points explained in the description of the main public function.
    // We want to keep iterating over the items until we have call expressions
    // - `something()()()()`
    // - `something[1][2][4]`
    // - `something[1]()[3]()`
    // - `something()[2].something.else[0]`
    let non_call_or_array_member_access_start = members
        .iter()
        .enumerate()
        // The first member is always part of the first group
        .skip(1)
        .find_map(|(index, member)| match member {
            ChainMember::CallExpression { .. }
            | ChainMember::TsNonNullAssertionExpression { .. } => None,

            ChainMember::ComputedMember { expression } => {
                if expression.member().is_ok_and(|member| {
                    matches!(
                        member,
                        AnyJsExpression::AnyJsLiteralExpression(
                            AnyJsLiteralExpression::JsNumberLiteralExpression(_),
                        )
                    )
                }) {
                    None
                } else {
                    Some(index)
                }
            }

            _ => Some(index),
        })
        .unwrap_or(members.len());

    let first_group_end_index = if !members
        .first()
        .is_some_and(|member| member.is_call_expression())
    {
        // Take as many member access chains as possible
        let rest = &members[non_call_or_array_member_access_start..];
        let member_end = rest
            .iter()
            .enumerate()
            .find_map(|(index, member)| match member {
                ChainMember::StaticMember { .. } | ChainMember::ComputedMember { .. } => {
                    let next_is_member = matches!(
                        rest.get(index + 1),
                        Some(ChainMember::ComputedMember { .. } | ChainMember::StaticMember { .. })
                    );

                    (!next_is_member).then_some(index)
                }
                _ => Some(index),
            })
            .unwrap_or(rest.len());

        non_call_or_array_member_access_start + member_end
    } else {
        non_call_or_array_member_access_start
    };

    let remaining = members.split_off(first_group_end_index);
    (MemberChainGroup::from(members), remaining)
}

/// computes groups coming after the first group
fn compute_remaining_groups(members: Vec<ChainMember>, comments: &JsComments) -> TailChainGroups {
    let mut has_seen_call_expression = false;
    let mut groups_builder = MemberChainGroupsBuilder::default();

    for member in members {
        let has_trailing_comments = comments.has_trailing_comments(member.syntax());

        match member {
            // [0] should be appended at the end of the group instead of the
            // beginning of the next one
            ChainMember::ComputedMember { .. } if is_computed_array_member_access(&member) => {
                groups_builder.start_or_continue_group(member);
            }

            ChainMember::StaticMember { .. } | ChainMember::ComputedMember { .. } => {
                // if we have seen a JsCallExpression, we want to close the group.
                // The resultant group will be something like: [ . , then, () ];
                // `.` and `then` belong to the previous StaticMemberExpression,
                // and `()` belong to the call expression we just encountered
                if has_seen_call_expression {
                    groups_builder.close_group();
                    groups_builder.start_group(member);
                    has_seen_call_expression = false;
                } else {
                    groups_builder.start_or_continue_group(member);
                }
            }

            ChainMember::CallExpression { .. } => {
                groups_builder.start_or_continue_group(member);
                has_seen_call_expression = true;
            }

            ChainMember::TsNonNullAssertionExpression { .. } => {
                groups_builder.start_or_continue_group(member);
            }

            ChainMember::Node(_) if member.is_call_like_expression() => {
                groups_builder.start_or_continue_group(member);
                has_seen_call_expression = true;
            }

            ChainMember::Node(_) => groups_builder.continue_group(member),
        }

        // Close the group immediately if the node had any trailing comments to
        // ensure those are printed in a trailing position for the token they
        // were originally commenting
        if has_trailing_comments {
            groups_builder.close_group();
            has_seen_call_expression = false;
        }
    }

    groups_builder.finish()
}

fn is_computed_array_member_access(member: &ChainMember) -> bool {
    if let ChainMember::ComputedMember { expression } = member {
        expression.member().is_ok_and(|member| {
            matches!(
                member,
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                )
            )
        })
    } else {
        false
    }
}

fn has_arrow_or_function_expression_arg(call: &JsCallExpression) -> bool {
    call.arguments().is_ok_and(|arguments| {
        arguments.args().iter().any(|argument| {
            matches!(
                argument,
                Ok(AnyJsCallArgument::AnyJsExpression(
                    AnyJsExpression::JsArrowFunctionExpression(_)
                        | AnyJsExpression::JsFunctionExpression(_)
                ))
            )
        })
    })
}

fn has_simple_arguments(call: &JsCallExpression) -> bool {
    call.arguments().is_ok_and(|arguments| {
        arguments.args().iter().all(|argument| {
            argument.is_ok_and(|argument| SimpleArgument::new(argument).is_simple())
        })
    })
}

/// In order to detect those cases, we use an heuristic: if the first
/// node is an identifier with the name starting with a capital
/// letter or just a sequence of _$. The rationale is that they are
/// likely to be factories.
fn is_factory(token: &JsSyntaxToken) -> bool {
    let text = token.text_trimmed();

    let mut bytes = text.bytes();

    match text.chars().next() {
        // Any sequence of '$' or '_' characters
        Some('_' | '$') => bytes.all(|b| matches!(b, b'_' | b'$')),
        Some(c) => c.is_uppercase(),
        _ => false,
    }
}

/// Here we check if the length of the groups exceeds the cutoff or there are comments
/// This function is the inverse of the prettier function
/// [Prettier applies]: https://github.com/prettier/prettier/blob/a043ac0d733c4d53f980aa73807a63fc914f23bd/src/language-js/print/member-chain.js#L342
pub fn is_member_call_chain(
    expression: JsCallExpression,
    comments: &JsComments,
    tab_width: TabWidth,
) -> SyntaxResult<bool> {
    let chain = MemberChain::from_call_expression(expression, comments, tab_width)?;

    Ok(chain.tail.is_member_call_chain(comments))
}

fn has_short_name(identifier: &JsIdentifierExpression, tab_width: TabWidth) -> bool {
    identifier
        .name()
        .and_then(|name| name.value_token())
        .is_ok_and(|name| name.text_trimmed().len() <= u8::from(tab_width) as usize)
}

struct ChainMembersIterator<'a> {
    next: Option<AnyJsExpression>,
    comments: &'a JsComments,
    root: bool,
}

impl<'a> ChainMembersIterator<'a> {
    fn new(root: AnyJsExpression, comments: &'a JsComments) -> Self {
        Self {
            next: Some(root),
            comments,
            root: true,
        }
    }
}

impl Iterator for ChainMembersIterator<'_> {
    type Item = ChainMember;

    fn next(&mut self) -> Option<Self::Item> {
        use AnyJsExpression::*;

        let expression = self.next.take()?;

        if self.comments.is_suppressed(expression.syntax()) {
            return Some(ChainMember::Node(expression.into_syntax()));
        }

        let member = match expression {
            JsCallExpression(call_expression) => {
                let callee = call_expression.callee().ok();

                let is_chain = matches!(
                    callee,
                    Some(
                        JsStaticMemberExpression(_)
                            | JsComputedMemberExpression(_)
                            | JsCallExpression(_)
                    )
                );

                if is_chain {
                    self.next = callee;
                }

                let position = if self.root {
                    CallExpressionPosition::End
                } else if !is_chain {
                    CallExpressionPosition::Start
                } else {
                    CallExpressionPosition::Middle
                };

                ChainMember::CallExpression {
                    expression: call_expression,
                    position,
                }
            }

            JsStaticMemberExpression(static_member) => {
                self.next = static_member.object().ok();
                ChainMember::StaticMember {
                    expression: static_member,
                }
            }

            JsComputedMemberExpression(computed_expression) => {
                self.next = computed_expression.object().ok();

                ChainMember::ComputedMember {
                    expression: computed_expression,
                }
            }

            TsNonNullAssertionExpression(expression) => {
                self.next = expression.expression().ok();
                ChainMember::TsNonNullAssertionExpression { expression }
            }

            expression => ChainMember::Node(expression.into_syntax()),
        };

        self.root = false;

        Some(member)
    }
}

impl FusedIterator for ChainMembersIterator<'_> {}
