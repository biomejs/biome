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
    AstNode, AstSeparatedElement, AstSeparatedList, BatchMutationExt, SyntaxElement, SyntaxNode,
    SyntaxResult, SyntaxToken, SyntaxTriviaPiece, TriviaPieceKind, chain_trivia_pieces,
    trim_leading_trivia_pieces,
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
        let new_list = break_line_after_trailing_line_comments(list, &new_list)?;

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

/// Breaks the line after a trailing `//` comment when sorting would otherwise
/// place the following token on the same line, where the comment would swallow
/// it.
///
/// Sorting moves each member together with its trailing separator (and the
/// separator's trivia). When that trivia ends with a `//` comment, the token
/// that ends up right after it on the same line would become part of the
/// comment. Breaking the line after the comment keeps the comment attached to
/// its member and the fix safe.
/// See https://github.com/biomejs/biome/issues/12057
fn break_line_after_trailing_line_comments(
    original_list: &JsObjectMemberList,
    sorted_list: &JsObjectMemberList,
) -> Option<JsObjectMemberList> {
    let indent = broken_line_indent(original_list);
    // The closing `}` keeps its own leading trivia, so a comment trailing the
    // last token is only dangerous when `}` doesn't start on a new line.
    let closing_brace_on_new_line = original_list
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

    // Rebuilding the list allocates, so first check (without allocating)
    // whether any line break is needed at all.
    if !needs_any_break(sorted_list, closing_brace_on_new_line)? {
        return Some(sorted_list.clone());
    }

    // Rebuild the list, breaking the line after `//` comments that would
    // otherwise swallow the following token. Members keep traveling with
    // their own trivia; only separators that need a break, and members that
    // then start on a new line, are touched.
    let mut break_before_next = false;
    let mut new_children: Vec<Option<SyntaxElement<JsLanguage>>> = Vec::new();
    let mut elements = sorted_list.elements().peekable();

    while let Some(element) = elements.next() {
        let node_syntax = element.node.ok()?.into_syntax();

        // The previous separator ended with a `//` comment: the member now
        // starts on a new line, so drop the whitespace that used to separate
        // it from the comment. Leading comments are kept.
        if break_before_next {
            break_before_next = false;
            if let Some(stripped) = without_leading_whitespace(&node_syntax) {
                new_children.push(Some(stripped.into()));
            } else {
                new_children.push(Some(node_syntax.into()));
            }
        } else {
            new_children.push(Some(node_syntax.into()));
        }

        if let Some(separator) = element.trailing_separator.ok()? {
            if separator_needs_break(&separator, elements.peek(), closing_brace_on_new_line)? {
                new_children.push(Some(append_line_break(&separator, &indent).into()));
                break_before_next = true;
            } else {
                new_children.push(Some(separator.into()));
            }
        }
    }

    let syntax = JsObjectMemberList::unwrap_cast(SyntaxNode::new_detached(
        sorted_list.syntax().kind(),
        new_children,
    ))
    .into_syntax();

    let syntax = restore_boundary_layout(original_list, &syntax)?;

    // A `//` comment trailing the last member itself (no separator to break
    // after) would swallow the closing `}` when `}` stays on the same line.
    if closing_brace_on_new_line {
        return Some(JsObjectMemberList::unwrap_cast(syntax));
    }
    let last_token = syntax.last_token()?;
    let mut syntax = syntax;
    if let Some(new_token) = terminate_line_comment(&last_token, &indent) {
        syntax = syntax.replace_child(last_token.into(), new_token.into())?;
    }

    Some(JsObjectMemberList::unwrap_cast(syntax))
}

/// Whether any line break is needed: a separator ends with a `//` comment
/// that would swallow the following token, or the last token ends with such
/// a comment that would swallow the closing `}`.
fn needs_any_break(
    sorted_list: &JsObjectMemberList,
    closing_brace_on_new_line: bool,
) -> Option<bool> {
    let mut elements = sorted_list.elements().peekable();
    while let Some(element) = elements.next() {
        if let Some(separator) = element.trailing_separator.ok()?
            && separator_needs_break(&separator, elements.peek(), closing_brace_on_new_line)?
        {
            return Some(true);
        }
    }
    Some(
        !closing_brace_on_new_line
            && sorted_list
                .syntax()
                .last_token()
                .is_some_and(|token| has_unterminated_line_comment(&token)),
    )
}

/// Whether a line break must be inserted after `separator`: it ends with a
/// `//` line comment and the following token doesn't start on a new line.
fn separator_needs_break(
    separator: &SyntaxToken<JsLanguage>,
    next: Option<&AstSeparatedElement<JsLanguage, AnyJsObjectMember>>,
    closing_brace_on_new_line: bool,
) -> Option<bool> {
    if !has_line_comment(separator.trailing_trivia().pieces()) {
        return Some(false);
    }
    Some(!next_starts_on_new_line(next, closing_brace_on_new_line)?)
}

/// Whether the token following a separator starts on a new line. The last
/// separator is followed by `}`, which keeps its own leading trivia.
fn next_starts_on_new_line(
    next: Option<&AstSeparatedElement<JsLanguage, AnyJsObjectMember>>,
    closing_brace_on_new_line: bool,
) -> Option<bool> {
    match next {
        Some(next) => Some(
            next.node
                .as_ref()
                .ok()?
                .syntax()
                .first_token()
                .is_some_and(|token| {
                    token
                        .leading_trivia()
                        .pieces()
                        .any(|piece| piece.is_newline())
                }),
        ),
        None => Some(closing_brace_on_new_line),
    }
}

/// Whether the pieces contain a `//` line comment.
fn has_line_comment(mut pieces: impl Iterator<Item = SyntaxTriviaPiece<JsLanguage>>) -> bool {
    pieces.any(|piece| piece.kind().is_single_line_comment())
}

/// Whether the token's trailing trivia ends with a `//` line comment that
/// isn't terminated by a newline.
fn has_unterminated_line_comment(token: &SyntaxToken<JsLanguage>) -> bool {
    let pieces = token.trailing_trivia().pieces();
    pieces
        .clone()
        .rposition(|piece| piece.kind().is_single_line_comment())
        .is_some_and(|index| !pieces.skip(index + 1).any(|piece| piece.is_newline()))
}

/// Appends a line break (plus `indent`) to the token's trailing trivia when it
/// ends with a `//` comment that isn't terminated by a newline. Returns `None`
/// when there's nothing to terminate.
fn terminate_line_comment(
    token: &SyntaxToken<JsLanguage>,
    indent: &str,
) -> Option<SyntaxToken<JsLanguage>> {
    has_unterminated_line_comment(token).then(|| append_line_break(token, indent))
}

/// Returns the node with leading whitespace and newlines dropped from its
/// first token, keeping any comments. Returns `None` when there's nothing to
/// drop.
fn without_leading_whitespace(node: &SyntaxNode<JsLanguage>) -> Option<SyntaxNode<JsLanguage>> {
    let first_token = node.first_token()?;
    let pieces = first_token.leading_trivia().pieces();
    let trimmed = trim_leading_trivia_pieces(pieces.clone());
    if trimmed.len() == pieces.len() {
        return None;
    }
    let new_token = first_token.with_leading_trivia_pieces(trimmed);
    node.clone()
        .replace_child(first_token.into(), new_token.into())
}

/// Appends a line break (plus `indent`) to the trailing trivia of `token`,
/// terminating any `//` comment it may end with.
fn append_line_break(token: &SyntaxToken<JsLanguage>, indent: &str) -> SyntaxToken<JsLanguage> {
    let old_pieces: Vec<SyntaxTriviaPiece<JsLanguage>> = token.trailing_trivia().pieces().collect();
    let mut new_trailing: Vec<(TriviaPieceKind, &str)> = Vec::with_capacity(old_pieces.len() + 2);
    new_trailing.extend(old_pieces.iter().map(|piece| (piece.kind(), piece.text())));
    new_trailing.push((TriviaPieceKind::Newline, "\n"));
    if !indent.is_empty() {
        new_trailing.push((TriviaPieceKind::Whitespace, indent));
    }
    token.with_trailing_trivia(new_trailing)
}

/// Indentation for lines broken after a `//` comment: the indentation of the
/// original members, falling back to the indentation of the line where the
/// object starts when the object fits on a single line.
fn broken_line_indent(list: &JsObjectMemberList) -> String {
    let mut indent = String::new();
    if let Some(first_token) = list.syntax().first_token() {
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
    if indent.is_empty() {
        // Single-line object: the whitespace after `{` belongs to the brace,
        // so align the broken lines with the line where the object starts.
        indent = object_line_indent(list);
    }
    indent
}

/// Whitespace after the last newline preceding the object's `{`, or empty when
/// `{` isn't preceded by a newline on its line.
fn object_line_indent(list: &JsObjectMemberList) -> String {
    let mut indent = String::new();
    let l_curly = list
        .syntax()
        .parent()
        .and_then(JsObjectExpression::cast)
        .and_then(|object| object.l_curly_token().ok());
    if let Some(l_curly) = l_curly {
        let mut seen_newline = false;
        for piece in l_curly.leading_trivia().pieces() {
            if piece.is_newline() {
                seen_newline = true;
                indent.clear();
            } else if seen_newline && piece.kind() == TriviaPieceKind::Whitespace {
                indent.push_str(piece.text());
            }
        }
    }
    indent
}

/// Restores the boundary layout of the original list onto the sorted list.
///
/// Sorting carries each member (with its trivia) to a new position, so the
/// first and last tokens of the sorted list don't start and end the way the
/// original list did. Only the layout — whitespace and newlines preceding any
/// comment — is restored: comments stay attached to the member they were
/// written for and travel with it.
fn restore_boundary_layout(
    original_list: &JsObjectMemberList,
    new_syntax: &SyntaxNode<JsLanguage>,
) -> Option<SyntaxNode<JsLanguage>> {
    let mut new_syntax = new_syntax.clone();

    if let (Some(original_first), Some(new_first)) = (
        original_list.syntax().first_token(),
        new_syntax.first_token(),
    ) {
        // Keep the member's own leading comments; only the indentation comes
        // from the original boundary.
        let merged = chain_trivia_pieces(
            LayoutTrivia::new(original_first.leading_trivia().pieces()),
            trim_leading_trivia_pieces(new_first.leading_trivia().pieces()),
        );
        let new_token = new_first.with_leading_trivia_pieces(merged);
        new_syntax = new_syntax.replace_child(new_first.into(), new_token.into())?;
    }

    if let (Some(original_last), Some(new_last)) =
        (original_list.syntax().last_token(), new_syntax.last_token())
    {
        let merged = chain_trivia_pieces(
            LayoutTrivia::new(original_last.trailing_trivia().pieces()),
            trim_leading_trivia_pieces(new_last.trailing_trivia().pieces()),
        );
        let new_token = new_last.with_trailing_trivia_pieces(merged);
        new_syntax = new_syntax.replace_child(new_last.into(), new_token.into())?;
    }

    Some(new_syntax)
}

/// Iterator over the whitespace and newline pieces that precede the first
/// comment of a trivia.
///
/// Unlike [`Iterator::filter`], this implements [`ExactSizeIterator`], so the
/// pieces can be attached to a token with the `*_pieces` APIs.
struct LayoutTrivia<I> {
    inner: I,
    len: usize,
}

impl<I> LayoutTrivia<I>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>> + Clone,
{
    fn new(pieces: I) -> Self {
        let len = pieces
            .clone()
            .take_while(|piece| piece.is_whitespace() || piece.is_newline())
            .count();
        Self { inner: pieces, len }
    }
}

impl<I> Iterator for LayoutTrivia<I>
where
    I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>,
{
    type Item = SyntaxTriviaPiece<JsLanguage>;

    fn next(&mut self) -> Option<Self::Item> {
        let piece = self.inner.next()?;
        if piece.is_whitespace() || piece.is_newline() {
            self.len -= 1;
            Some(piece)
        } else {
            // The layout ends where the first comment starts.
            self.len = 0;
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl<I> ExactSizeIterator for LayoutTrivia<I> where I: Iterator<Item = SyntaxTriviaPiece<JsLanguage>>
{}

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
