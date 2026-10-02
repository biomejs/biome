use std::{borrow::Cow, cmp::Ordering, ops::Not};

use biome_analyze::{
    Ast, FixKind, Rule, RuleAction, RuleDiagnostic, RuleSource,
    context::RuleContext,
    declare_source_rule,
    utils::{is_separated_list_sorted_by, sorted_separated_list_by},
};
use biome_console::markup;
use biome_deserialize::TextRange;
use biome_diagnostics::Applicability;
use biome_js_factory::make;
use biome_js_syntax::{
    AnyJsExpression, AnyJsObjectMember, JsLanguage, JsObjectExpression, JsObjectMemberList, T,
};
use biome_rowan::{
    AstNode, BatchMutationExt, SyntaxNode, SyntaxResult, SyntaxToken, SyntaxTriviaPiece,
    TriviaPieceKind,
};
use biome_rule_options::use_sorted_keys::{SortOrder, UseSortedKeysOptions};
use biome_string_case::comparable_token::ComparableToken;

use crate::JsRuleAction;

declare_source_rule! {
    /// Sort properties of a JS object in natural order.
    ///
    /// [Natural order](https://en.wikipedia.org/wiki/Natural_sort_order) means
    /// that uppercase letters come before lowercase letters (e.g. `A` < `a` <
    /// `B` < `b`) and numbers are compared in a human way (e.g. `9` < `10`).
    ///
    /// This rule will consider spread/calculated keys e.g [k]: 1 as
    /// non-sortable. Instead, whenever it encounters a non-sortable key, it
    /// will sort all the previous sortable keys up until the nearest
    /// non-sortable key, if one exist. This prevents breaking the override of
    /// certain keys using spread keys.
    ///
    /// Sorting the keys of an object technically changes the semantics of the
    /// program. It affects the result of operations like
    /// `Object.getOwnPropertyNames`. Since ES2020, operations like `for-in`
    /// loops, `Object.keys`, and `JSON.stringify` are guaranteed to process
    /// string keys in insertion order.
    ///
    /// In cases where the order of such operations is important, you can
    /// disable the assist action using a suppression comment:
    ///
    /// `// biome-ignore assist/source/useSortedKeys`
    ///
    /// ## Examples
    ///
    /// ```js,expect_diff
    /// const obj = {
    ///   x: 1,
    ///   a: 2,
    /// };
    /// ```
    ///
    /// ```js,expect_diff
    /// const obj = {
    ///   x: 1,
    ///   ...f,
    ///   y: 4,
    ///   a: 2,
    ///   [calculated()]: true,
    ///   b: 3,
    ///   a: 1,
    /// };
    /// ```
    ///
    /// ```js
    /// const obj = {
    ///   get aab() {
    ///     return this._aab;
    ///   },
    ///   set aac(v) {
    ///     this._aac = v;
    ///   },
    ///   w: 1,
    ///   x: 1,
    ///   ...g,
    ///   get aaa() {
    ///     return "";
    ///   },
    ///   u: 1,
    ///   v: 1,
    ///   [getProp()]: 2,
    ///   o: 1,
    ///   p: 1,
    ///   q: 1,
    /// }
    /// ```
    ///
    /// ## Options
    /// This actions accepts following options
    ///
    /// ### `sortOrder`
    /// This options supports `natural` and `lexicographic` values. Where as `natural` is the default.
    ///
    /// Following will apply the natural sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "natural"
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diff
    /// const obj = {
    ///     val13: 1,
    ///     val1: 1,
    ///     val2: 1,
    ///     val21: 1,
    ///     val11: 1,
    /// };
    /// ```
    ///
    /// Following will apply the lexicographic sort order.
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "sortOrder": "lexicographic"
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diff
    /// const obj = {
    ///     val13: 1,
    ///     val1: 1,
    ///     val2: 1,
    ///     val21: 1,
    ///     val11: 1,
    /// };
    /// ```
    ///
    /// ### `groupByNesting`
    /// When enabled, groups object keys by their value's nesting depth before sorting alphabetically.
    /// Simple values (primitives, single-line arrays, and single-line objects) are sorted first,
    /// followed by nested values (multi-line arrays and multi-line objects).
    ///
    /// > Default: `false`
    ///
    ///
    /// ```json,options
    /// {
    ///     "options": {
    ///         "groupByNesting": true
    ///     }
    /// }
    /// ```
    /// ```js,use_options,expect_diagnostic
    /// const obj = {
    ///     name: "Sample",
    ///     details: {
    ///         description: "nested"
    ///     },
    ///     id: 123
    /// };
    /// ```
    ///
    pub UseSortedKeys {
        version: "2.0.0",
        name: "useSortedKeys",
        language: "js",
        recommended: false,
        sources: &[RuleSource::Eslint("sort-keys").inspired(), RuleSource::EslintPerfectionist("sort-objects").inspired()],
        fix_kind: FixKind::Safe,
    }
}

impl Rule for UseSortedKeys {
    type Query = Ast<JsObjectMemberList>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = UseSortedKeysOptions;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let options = ctx.options();
        let sort_order = options.sort_order.unwrap_or_default();
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        if options.group_by_nesting.unwrap_or(false) {
            is_separated_list_sorted_by(
                ctx.query(),
                |node| {
                    let depth = get_member_depth(node)?;
                    let name = node.name().map(ComparableToken::new)?;
                    Some((depth, name))
                },
                |(d1, n1), (d2, n2)| d1.cmp(d2).then_with(|| comparator(n1, n2)),
            )
            .ok()?
            .not()
            .then_some(())
        } else {
            is_separated_list_sorted_by(
                ctx.query(),
                |node| node.name().map(ComparableToken::new),
                comparator,
            )
            .ok()?
            .not()
            .then_some(())
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let options = ctx.options();
        let message = if options.group_by_nesting.unwrap_or(false) {
            markup! {
                "The object properties are not sorted by nesting level and key."
            }
        } else {
            markup! {
                "The object properties are not sorted by key."
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            message,
        ))
    }

    fn text_range(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<TextRange> {
        ctx.query()
            .syntax()
            .ancestors()
            .skip(1)
            .find_map(JsObjectExpression::cast)
            .map(|object| object.range())
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let list = ctx.query();
        let options = ctx.options();
        let sort_order = options.sort_order.unwrap_or_default();
        let comparator = match sort_order {
            SortOrder::Natural => ComparableToken::ascii_nat_cmp,
            SortOrder::Lexicographic => ComparableToken::lexicographic_cmp,
        };

        let new_list = if options.group_by_nesting.unwrap_or(false) {
            sorted_separated_list_by(
                list,
                |node| {
                    let depth = get_member_depth(node)?;
                    let name = node.name().map(ComparableToken::new)?;
                    Some((depth, name))
                },
                || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                |(d1, n1), (d2, n2)| d1.cmp(d2).then_with(|| comparator(n1, n2)),
            )
            .ok()?
        } else {
            sorted_separated_list_by(
                list,
                |node| node.name().map(ComparableToken::new),
                || make::token(T![,]).with_trailing_trivia([(TriviaPieceKind::Whitespace, " ")]),
                comparator,
            )
            .ok()?
        };

        // A `//` comment swallows the rest of its line. Sorting carries each
        // member together with its trailing separator (and the separator's
        // trivia), so a member sorted to follow such a comment on the same
        // line would become part of the comment. Break the line after the
        // comment instead of giving up the fix, keeping the comment attached
        // to its member.
        // See https://github.com/biomejs/biome/issues/12057
        //
        // The closing `}` keeps its own leading trivia, so a comment trailing
        // the last token is only dangerous when `}` doesn't start on a new line.
        let closing_brace_on_new_line = list
            .syntax()
            .parent()
            .and_then(JsObjectExpression::cast)
            .and_then(|object| object.r_curly_token().ok())
            .is_some_and(|token| {
                token
                    .leading_trivia()
                    .pieces()
                    .any(|piece| piece.is_newline())
            });

        let new_list =
            break_line_after_trailing_line_comments(list, &new_list, closing_brace_on_new_line)?;

        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia(list.clone(), new_list);

        Some(RuleAction::new(
            rule_action_category!(),
            Applicability::Always,
            markup! { "Sort the object properties by key." },
            mutation,
        ))
    }
}

/// Breaks the line after a trailing `//` comment when the sorted order would
/// otherwise place the following token on the same line, where the comment
/// would swallow it.
///
/// A `//` comment swallows the rest of its line. Sorting reorders members
/// together with their trailing separators (and the separators' trivia), which
/// can place a token right after such a comment on the same line, turning it
/// into part of the comment. Instead of giving up the fix, break the line
/// after the comment so the following token starts on a new line, keeping the
/// comment attached to its member.
/// See https://github.com/biomejs/biome/issues/12057
fn break_line_after_trailing_line_comments(
    original_list: &JsObjectMemberList,
    sorted_list: &JsObjectMemberList,
    closing_brace_on_new_line: bool,
) -> Option<JsObjectMemberList> {
    use biome_rowan::{AstSeparatedElement, AstSeparatedList};

    // Collect the (member, separator) pairs in order.
    let mut pairs: Vec<(AnyJsObjectMember, Option<SyntaxToken<JsLanguage>>)> =
        Vec::with_capacity(sorted_list.len());
    for AstSeparatedElement {
        node,
        trailing_separator,
    } in sorted_list.elements()
    {
        pairs.push((node.ok()?, trailing_separator.ok()?));
    }

    // Indent the new line like the original list: the whitespace after
    // the last newline in the leading trivia of its first token.
    let mut indent = String::new();
    if let Some(first_token) = original_list.syntax().first_token() {
        for piece in first_token.leading_trivia().pieces() {
            if piece.is_newline() {
                indent.clear();
            } else if piece.kind() == TriviaPieceKind::Whitespace {
                indent.push_str(piece.text());
            } else {
                // A comment or anything else: don't guess the indent.
                indent.clear();
                break;
            }
        }
    }

    let mut changed = false;
    for i in 0..pairs.len() {
        let separator_has_line_comment = pairs[i].1.as_ref().is_some_and(|separator| {
            separator
                .trailing_trivia()
                .pieces()
                .any(|piece| piece.kind().is_single_line_comment())
        });
        if !separator_has_line_comment {
            continue;
        }
        // The token following the comment is the next member, or the closing
        // `}` for the last pair. The `}` keeps its own leading trivia, so the
        // comment is only dangerous when `}` doesn't start on a new line.
        let next_starts_on_new_line = if let Some((next_node, _)) = pairs.get(i + 1) {
            next_node.syntax().first_token().is_some_and(|token| {
                token
                    .leading_trivia()
                    .pieces()
                    .any(|piece| piece.is_newline())
            })
        } else {
            closing_brace_on_new_line
        };
        if next_starts_on_new_line {
            continue;
        }
        changed = true;

        if pairs.get(i + 1).is_some() {
            // Disjoint mutable borrows of `pairs[i]` and `pairs[i + 1]`.
            let (head, tail) = pairs.split_at_mut(i + 1);
            let (_, separator) = &mut head[i];
            let (next_node, _) = &mut tail[0];

            let separator_token = separator.take()?;
            *separator = Some(append_line_break(&separator_token, &indent));

            // The next member now starts on a new line; drop its leading whitespace.
            if let Some(first_token) = next_node.syntax().first_token() {
                let new_leading: Vec<SyntaxTriviaPiece<JsLanguage>> = first_token
                    .leading_trivia()
                    .pieces()
                    .skip_while(|piece| piece.kind() == TriviaPieceKind::Whitespace)
                    .collect();
                if let Some(new_node) = next_node.clone().with_leading_trivia_pieces(new_leading) {
                    *next_node = new_node;
                }
            }
        } else {
            // Last pair: the closing `}` follows the comment on the same line.
            // The `}` is outside the replaced list, so only the break is added here.
            let (_, separator) = &mut pairs[i];
            let separator_token = separator.take()?;
            *separator = Some(append_line_break(&separator_token, &indent));
        }
    }

    // A `//` comment trailing the last member itself (no separator) would
    // swallow the closing `}` when `}` stays on the same line.
    let last_token_needs_break = !closing_brace_on_new_line
        && sorted_list.syntax().last_token().is_some_and(|token| {
            token
                .trailing_trivia()
                .pieces()
                .any(|piece| piece.kind().is_single_line_comment())
        });

    if !changed && !last_token_needs_break {
        return Some(sorted_list.clone());
    }

    // Rebuild the list from the modified pairs.
    let node_count = sorted_list.len();
    let separators: Vec<SyntaxToken<JsLanguage>> = pairs
        .iter_mut()
        .filter_map(|(_, separator)| separator.take())
        .collect();
    let separator_count = separators.len();
    let mut separators = separators.into_iter();
    let mut items = pairs.into_iter().map(|(node, _)| node);
    let mut result = JsObjectMemberList::unwrap_cast(SyntaxNode::new_detached(
        sorted_list.syntax().kind(),
        (0..node_count + separator_count).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ));

    // The sorted list carries each member's original trivia, so the first and
    // last tokens may not have the boundary trivia of the original list.
    // Restore it from the original list so the replacement keeps the
    // surrounding layout.
    if let (Some(original_first), Some(_)) = (
        original_list.syntax().first_token(),
        result.syntax().first_token(),
    ) {
        let leading: Vec<SyntaxTriviaPiece<JsLanguage>> =
            original_first.leading_trivia().pieces().collect();
        result = JsObjectMemberList::unwrap_cast(
            result.into_syntax().with_leading_trivia_pieces(leading)?,
        );
    }
    if let (Some(original_last), Some(result_last)) = (
        original_list.syntax().last_token(),
        result.syntax().last_token(),
    ) {
        // A `//` comment in the original trailing trivia always still exists
        // on its own token in the sorted list: separators with comments are
        // kept, and member trivia travels with the member. Copying it here
        // would duplicate it onto the wrong member, so only the comment-free
        // pieces (usually just whitespace) are restored.
        // Conversely, a `//` comment already trailing the result's last token
        // traveled with that token and must be kept, not overwritten.
        let result_has_line_comment = result_last
            .trailing_trivia()
            .pieces()
            .any(|piece| piece.kind().is_single_line_comment());
        if !result_has_line_comment {
            let trailing: Vec<SyntaxTriviaPiece<JsLanguage>> = original_last
                .trailing_trivia()
                .pieces()
                .filter(|piece| !piece.kind().is_single_line_comment())
                .collect();
            result = JsObjectMemberList::unwrap_cast(
                result.into_syntax().with_trailing_trivia_pieces(trailing)?,
            );
        }
    }

    // Terminate a `//` comment trailing the last token so the closing `}`
    // isn't swallowed. This covers the last member carrying the comment
    // itself (no separator to break after in the loop above). When the
    // closing `}` already starts on a new line, the comment is harmless.
    let token_needing_break = (!closing_brace_on_new_line)
        .then(|| result.syntax().last_token())
        .flatten()
        .filter(|token| {
            let pieces: Vec<SyntaxTriviaPiece<JsLanguage>> =
                token.trailing_trivia().pieces().collect();
            pieces
                .iter()
                .rposition(|piece| piece.kind().is_single_line_comment())
                .is_some_and(|index| !pieces[index + 1..].iter().any(|piece| piece.is_newline()))
        });
    if let Some(last_token) = token_needing_break {
        let new_token = append_line_break(&last_token, &indent);
        result = JsObjectMemberList::unwrap_cast(
            result
                .into_syntax()
                .replace_child(last_token.into(), new_token.into())?,
        );
    }

    Some(result)
}

/// Appends a line break (plus `indent`) to the trailing trivia of `token`,
/// terminating any `//` comment it may end with.
fn append_line_break(token: &SyntaxToken<JsLanguage>, indent: &str) -> SyntaxToken<JsLanguage> {
    let old_pieces: Vec<SyntaxTriviaPiece<JsLanguage>> = token.trailing_trivia().pieces().collect();
    let mut new_trailing: Vec<(TriviaPieceKind, &str)> = old_pieces
        .iter()
        .map(|piece| (piece.kind(), piece.text()))
        .collect();
    new_trailing.push((TriviaPieceKind::Newline, "\n"));
    if !indent.is_empty() {
        new_trailing.push((TriviaPieceKind::Whitespace, indent));
    }
    token.with_trailing_trivia(new_trailing)
}

/// Checks if an object/array spans multiple lines by examining CST trivia.
/// For non-empty containers, checks the first token of the members/elements.
/// For empty containers, checks the closing brace/bracket token.
fn has_multiline_content(
    members_first_token: Option<SyntaxToken<JsLanguage>>,
    closing_token: SyntaxResult<SyntaxToken<JsLanguage>>,
) -> bool {
    members_first_token.map_or_else(
        || closing_token.is_ok_and(|token| token.has_leading_newline()),
        |token| token.has_leading_newline(),
    )
}

/// Determines the nesting depth of a JavaScript expression for grouping purposes.
fn get_nesting_depth(value: &AnyJsExpression) -> Ordering {
    match value {
        AnyJsExpression::JsObjectExpression(obj) => {
            let members = obj.members();
            if has_multiline_content(members.syntax().first_token(), obj.r_curly_token()) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        AnyJsExpression::JsArrayExpression(array) => {
            let elements = array.elements();
            if has_multiline_content(elements.syntax().first_token(), array.r_brack_token()) {
                Ordering::Greater
            } else {
                Ordering::Equal
            }
        }
        // Function and class expressions are treated as nested
        AnyJsExpression::JsArrowFunctionExpression(_)
        | AnyJsExpression::JsFunctionExpression(_)
        | AnyJsExpression::JsClassExpression(_) => Ordering::Greater,
        _ => Ordering::Equal,
    }
}

/// Determines the nesting depth for an object member:
/// - properties: based on value expression;
/// - methods/getters/setters: treat as nested (1);
/// - spreads/computed or unnamed: non-sortable (None).
fn get_member_depth(node: &AnyJsObjectMember) -> Option<Ordering> {
    match node {
        AnyJsObjectMember::JsPropertyObjectMember(prop) => {
            let value = prop.value().ok()?;
            Some(get_nesting_depth(&value))
        }
        AnyJsObjectMember::JsMethodObjectMember(_)
        | AnyJsObjectMember::JsGetterObjectMember(_)
        | AnyJsObjectMember::JsSetterObjectMember(_) => Some(Ordering::Greater),
        _ => None,
    }
}
