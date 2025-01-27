use crate::comments::is_type_comment;
use biome_formatter::{TransformSourceMap, TransformSourceMapBuilder};
use biome_js_syntax::{
    misc_ext::AnyJsParenthesized, AnyJsAssignment, AnyJsExpression, AnyJsOptionalChainExpression,
    AnyTsType, JsLanguage, JsLogicalExpression, JsSyntaxKind, JsSyntaxNode, JsSyntaxTrivia,
};
use biome_rowan::{
    chain_trivia_pieces, AstNode, SyntaxKind, SyntaxRewriter, SyntaxToken, TextSize,
    VisitNodeSignal,
};
use std::collections::BTreeSet;

pub(super) fn transform(root: JsSyntaxNode) -> (JsSyntaxNode, TransformSourceMap) {
    let mut rewriter = JsFormatSyntaxRewriter::with_offset(root.text_range_with_trivia().start());
    let transformed = rewriter.transform(root);
    (transformed, rewriter.finish())
}

#[derive(Default)]
struct JsFormatSyntaxRewriter {
    source_map: TransformSourceMapBuilder,

    /// Stores a map of the positions at which a `(` paren has been removed.
    /// This is necessary for correctly computing the source offsets for nested parenthesized expressions with whitespace:
    ///
    /// ```javascript
    /// function f() {
    ///     return (
    ///         (
    ///             // prettier-ignore
    ///             /* $FlowFixMe(>=0.53.0) */
    ///             <JSX />
    ///         )
    ///     );
    /// }
    /// ```
    /// The rewriter first removes any leading whitespace from the `JsxTagExpression`'s leading trivia.
    /// However, it then removes the leading/trailing whitespace of the inner `(` as well when handling the outer
    /// parenthesized expressions but the ranges of the `(` trailing trivia pieces no longer match the source ranges because
    /// they are now off by 1 because of the removed `(`.
    l_paren_source_position: BTreeSet<TextSize>,
}

impl JsFormatSyntaxRewriter {
    pub fn with_offset(offset: TextSize) -> Self {
        JsFormatSyntaxRewriter {
            source_map: TransformSourceMapBuilder::with_offset(offset),
            ..Default::default()
        }
    }
}

impl JsFormatSyntaxRewriter {
    /// Replaces parenthesized expression that:
    /// * have no syntax error: has no missing required child or no skipped token trivia attached to the left or right paren
    /// * inner expression isn't an bogus node
    /// * no closure or type cast comment
    ///
    /// with the inner expression.
    ///
    /// ## Trivia Overview
    ///
    /// We have to spend extra attention on the handling of the trivia attached to the left and right parentheses:
    ///
    /// ```javascript
    /// statement;
    /// /* leading l-paren */ ( /* trailing l-paren */
    ///   /* leading a */ a + b /* trailing b */
    ///   /* leading r-paren */ ) /* trailing r-paren */
    /// ```
    ///
    /// The implementation pre-appends the left parenthesis's trivia to the leading trivia of the expression's first token,
    /// and appends the right parenthesis's trivia to the trailing trivia of the expression's last token. So the trivia (ignoring whitespace)
    /// after the transform for the above example is:
    ///
    /// * `a`: `/* leading l-paren */ /* trailing l-paren */ /* leading a */`
    /// * `b`: `/* trailing b */ /* leading r-paren */ /* trailing r-paren */`
    ///
    /// The fact that the implementation appends the right parenthesis's leading trivia to the last token's trailing
    /// trivia is slightly inconsistent with our [biome_rowan::SyntaxToken::trailing_trivia] definition as it can now happen that the
    /// trailing trivia contains line breaks. In practice, this isn't a problem for the formatter.
    ///
    /// ## Leading Whitespace Trivia
    ///
    /// The formatter counts the new lines in a node's leading trivia to determine if it should e.g. insert an
    /// empty new line between two statements. This is why it is necessary to remove any whitespace
    /// between the left parentheses and the token to avoid the insertion of additional new lines if there was a line break
    /// after the left parentheses or
    ///
    /// ```javascript
    /// a
    /// (
    ///   Long &&
    ///   Longer &&
    /// )
    /// ```
    ///
    /// would become
    ///
    /// ```javascript
    /// a
    ///
    /// (
    ///   Long &&
    ///   Longer &&
    /// )
    /// ```
    ///
    /// because the `Long` has two leading new lines after removing parentheses, the one after `a` and the one after the opening `(`.
    ///
    /// However, it is important to leave at least one leading new line in front of the token's leading trivia if there's a comment in the leading trivia
    /// because we want that leading comments that are preceded by a line break to be formatted on their own line.
    ///
    /// ```javascript
    /// (
    ///   // comment
    ///   a
    /// )
    /// ```
    ///
    /// Keep the line break before the `// comment` or the formatter will format the comment on the same line as the `(` token
    ///
    /// ```javascript
    /// ( // comment
    /// a
    /// )
    /// ```
    ///
    /// Which may turn `//comment` into a trailing comment that then gets formatted differently on the next formatting pass, resulting in instability issues.
    fn visit_parenthesized(
        &mut self,
        parenthesized: AnyJsParenthesized,
    ) -> VisitNodeSignal<JsLanguage> {
        let (l_paren, inner, r_paren) = match (
            parenthesized.l_paren_token(),
            parenthesized.inner(),
            parenthesized.r_paren_token(),
        ) {
            (Ok(l_paren), Ok(inner), Ok(r_paren)) => {
                let prev_token = l_paren.prev_token();
                // Keep parentheses around unknown expressions. Biome can't know the precedence.
                if inner.kind().is_bogus()
                    // Don't remove parentheses if the expression in js template element
                    || inner.grand_parent().is_some_and(|parent|
                        parent.grand_parent().is_some_and(|node|
                            node.kind() == JsSyntaxKind::JS_TEMPLATE_ELEMENT
                        )
                    )
                    // Don't remove parentheses if the expression is a decorator
                    || inner.grand_parent().is_some_and(|node| node.kind() == JsSyntaxKind::JS_DECORATOR && decorator_expression_needs_parens(&inner))
                    // Don't remove parentheses if they have skipped trivia. We don't know for certain what the intended syntax is.
                    // Nor if there's a leading type cast comment
                    || has_type_cast_comment_or_skipped(&l_paren.leading_trivia())
                    || prev_token.is_some_and(|prev_token| has_type_cast_comment_or_skipped(&prev_token.trailing_trivia()))
                    || r_paren.leading_trivia().has_skipped()
                    // Don't remove parentheses if it is an optional chain inside a chain that doesn't start by an optional token
                    // (a?.b).c
                    || (parenthesized.syntax().parent().and_then(AnyJsOptionalChainExpression::cast).is_some_and(|chain| chain.optional_chain_token().is_none()) && AnyJsOptionalChainExpression::cast_ref(&inner).is_some_and(|x| x.is_optional_chain()))
                {
                    return VisitNodeSignal::Traverse(parenthesized.into_syntax());
                } else {
                    (l_paren, inner, r_paren)
                }
            }
            _ => {
                // At least one missing child, handle as a regular node
                return VisitNodeSignal::Traverse(parenthesized.into_syntax());
            }
        };

        self.source_map.push_source_text(l_paren.text());

        let inner_trimmed_range = inner.text_trimmed_range();
        // Store away the inner offset because the new returned inner might be a detached node
        let original_inner_offset = inner.text_range_with_trivia().start();
        let inner = self.transform(inner);
        let inner_offset = original_inner_offset - inner.text_range_with_trivia().start();

        match inner.first_token() {
            // This can only happen if we have `()` which is highly unlikely to ever be the case.
            // Return the parenthesized expression as is. This will be formatted as verbatim
            None => {
                let updated = match parenthesized {
                    AnyJsParenthesized::JsParenthesizedExpression(expression) => {
                        // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                        expression
                            .with_expression(AnyJsExpression::unwrap_cast(inner))
                            .into_syntax()
                    }
                    AnyJsParenthesized::JsParenthesizedAssignment(assignment) => {
                        // SAFETY: Safe because the rewriter never rewrites an assignment to a non assignment.
                        assignment
                            .with_assignment(AnyJsAssignment::unwrap_cast(inner))
                            .into_syntax()
                    }
                    AnyJsParenthesized::TsParenthesizedType(ty) => {
                        ty.with_ty(AnyTsType::unwrap_cast(inner)).into_syntax()
                    }
                };

                self.source_map.push_source_text(r_paren.text());

                VisitNodeSignal::Replace(updated)
            }

            Some(first_token) => {
                self.source_map.extend_trimmed_node_range(
                    inner_trimmed_range,
                    parenthesized.syntax().text_trimmed_range(),
                );

                let l_paren_trimmed_range = l_paren.text_trimmed_range();
                self.source_map.add_deleted_range(l_paren_trimmed_range);
                self.l_paren_source_position
                    .insert(l_paren_trimmed_range.start());

                let mut l_paren_trailing = l_paren.trailing_trivia().pieces().peekable();

                // Skip over leading whitespace
                while let Some(piece) = l_paren_trailing.peek() {
                    if piece.is_whitespace() {
                        self.source_map.add_deleted_range(piece.text_range());
                        l_paren_trailing.next();
                    } else {
                        break;
                    }
                }

                let l_paren_trailing_non_whitespace_trivia = l_paren_trailing
                    .peek()
                    .is_some_and(|piece| piece.is_skipped() || piece.is_comments());

                let l_paren_trivia =
                    chain_trivia_pieces(l_paren.leading_trivia().pieces(), l_paren_trailing);

                let mut leading_trivia = first_token.leading_trivia().pieces().peekable();
                let mut first_new_line = None;

                let mut inner_offset = inner_offset;

                // if !is_parent_parenthesized {
                // The leading whitespace before the opening parens replaces the whitespace before the node.
                while let Some(trivia) = leading_trivia.peek() {
                    if self
                        .l_paren_source_position
                        .contains(&(trivia.text_range().start() + inner_offset))
                    {
                        inner_offset += TextSize::from(1);
                    }

                    if trivia.is_newline() && first_new_line.is_none() {
                        first_new_line = Some((
                            trivia.text_range() + inner_offset,
                            leading_trivia.next().unwrap(),
                        ));
                    } else if trivia.is_whitespace() || trivia.is_newline() {
                        self.source_map
                            .add_deleted_range(trivia.text_range() + inner_offset);
                        leading_trivia.next();
                    } else {
                        break;
                    }
                }

                // Remove all leading new lines directly in front of the token but keep the leading new-line if it precedes a skipped token trivia or a comment.
                if !l_paren_trailing_non_whitespace_trivia
                    && leading_trivia.peek().is_none()
                    && first_new_line.is_some()
                {
                    let (inner_offset, _) = first_new_line.take().unwrap();

                    self.source_map.add_deleted_range(inner_offset);
                }
                // }

                let leading_trivia = chain_trivia_pieces(
                    first_new_line.map(|(_, trivia)| trivia).into_iter(),
                    leading_trivia,
                );

                let new_leading = chain_trivia_pieces(l_paren_trivia, leading_trivia);
                let new_first = first_token.with_leading_trivia_pieces(new_leading);

                // SAFETY: Calling `unwrap` is safe because we know that `inner_first` is part of the `inner` subtree.
                let updated = inner
                    .replace_child(first_token.into(), new_first.into())
                    .unwrap();

                let r_paren_trivia = chain_trivia_pieces(
                    r_paren.leading_trivia().pieces(),
                    r_paren.trailing_trivia().pieces(),
                );

                // SAFETY: Calling `unwrap` is safe because `last_token` only returns `None` if a node's subtree
                // doesn't contain ANY token, but we know that the subtree contains at least the first token.
                let last_token = updated.last_token().unwrap();

                let new_last = last_token.append_trivia_pieces(r_paren_trivia);

                self.source_map
                    .add_deleted_range(r_paren.text_trimmed_range());

                self.source_map.push_source_text(r_paren.text());

                // SAFETY: Calling `unwrap` is safe because we know that `last_token` is part of the `updated` subtree.
                VisitNodeSignal::Replace(
                    updated
                        .replace_child(last_token.into(), new_last.into())
                        .unwrap(),
                )
            }
        }
    }

    /// Re-balances right-recursive logical expressions with the same operator to be left recursive (relies on the parentheses removal)
    ///
    /// ```javascript
    /// a && (b && c)
    /// ```
    ///
    /// has the tree (parentheses omitted)
    ///
    /// ```text
    ///   &&
    /// a    &&
    ///    b    c
    /// ```
    ///
    /// This transform re-balances the tree so that it becomes left-recursive
    ///
    /// ```text
    ///     &&
    ///  &&    c
    /// a  b
    /// ```
    ///
    /// This is required so that the binary like expression formatting only has to resolve left recursive expressions.
    fn visit_logical_expression(
        &mut self,
        logical: JsLogicalExpression,
    ) -> VisitNodeSignal<JsLanguage> {
        match (logical.left(), logical.operator_token(), logical.right()) {
            (Ok(left), Ok(operator), Ok(right)) => {
                let left_key = left.syntax().key();
                let operator_key = operator.key();
                let right_key = right.syntax().key();

                // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                let left = AnyJsExpression::unwrap_cast(self.transform(left.into_syntax()));
                let operator = self.visit_token(operator);
                // SAFETY: Safe because the rewriter never rewrites an expression to a non expression.
                let right = AnyJsExpression::unwrap_cast(self.transform(right.into_syntax()));

                let updated = match right {
                    AnyJsExpression::JsLogicalExpression(right_logical) => {
                        match (
                            right_logical.left(),
                            right_logical.operator_token(),
                            right_logical.right(),
                        ) {
                            (Ok(right_left), Ok(right_operator), Ok(right_right))
                                if right_operator.kind() == operator.kind() =>
                            {
                                logical
                                    .with_left(
                                        biome_js_factory::make::js_logical_expression(
                                            left, operator, right_left,
                                        )
                                        .into(),
                                    )
                                    .with_operator_token_token(right_operator)
                                    .with_right(right_right)
                            }

                            // Don't re-balance a logical expression that has syntax errors
                            _ => logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right_logical.into()),
                        }
                    }

                    // Don't re-balance logical expressions with different operators
                    // Avoid updating the node if none of the children have changed to avoid
                    // re-spinning all parents.
                    right => {
                        if left.syntax().key() != left_key
                            || operator.key() != operator_key
                            || right.syntax().key() != right_key
                        {
                            logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right)
                        } else {
                            logical
                        }
                    }
                };

                VisitNodeSignal::Replace(updated.into_syntax())
            }
            _ => VisitNodeSignal::Traverse(logical.into_syntax()),
        }
    }

    pub(crate) fn finish(self) -> TransformSourceMap {
        self.source_map.finish()
    }
}

// TODO: This should be handled with a `NeedsParentheses` impl.
fn decorator_expression_needs_parens(inner: &JsSyntaxNode) -> bool {
    match AnyJsExpression::cast_ref(inner) {
        Some(AnyJsExpression::JsCallExpression(call)) => call.callee().map_or(true, |callee| {
            call.is_optional() || decorator_expression_needs_parens(callee.syntax())
        }),
        Some(AnyJsExpression::JsStaticMemberExpression(static_expr)) => {
            static_expr.is_optional()
                || static_expr.object().map_or(true, |object| {
                    object.syntax().kind() != JsSyntaxKind::JS_IDENTIFIER_EXPRESSION
                })
        }
        Some(AnyJsExpression::JsIdentifierExpression(_)) => false,
        _ => true,
    }
}

impl SyntaxRewriter for JsFormatSyntaxRewriter {
    type Language = JsLanguage;

    fn visit_node(&mut self, node: JsSyntaxNode) -> VisitNodeSignal<Self::Language> {
        match node.kind() {
            kind if AnyJsParenthesized::can_cast(kind) => {
                let parenthesized = AnyJsParenthesized::unwrap_cast(node);

                self.visit_parenthesized(parenthesized)
            }
            JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
                let logical = JsLogicalExpression::unwrap_cast(node);

                self.visit_logical_expression(logical)
            }
            _ => VisitNodeSignal::Traverse(node),
        }
    }

    fn visit_token(&mut self, token: SyntaxToken<Self::Language>) -> SyntaxToken<Self::Language> {
        self.source_map.push_source_text(token.text());
        token
    }
}

fn has_type_cast_comment_or_skipped(trivia: &JsSyntaxTrivia) -> bool {
    trivia.pieces().any(|piece| {
        if let Some(comment) = piece.as_comments() {
            is_type_comment(&comment)
        } else {
            piece.is_skipped()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::JsFormatSyntaxRewriter;
    use crate::{format_node, JsFormatOptions, TextRange};
    use biome_formatter::{SourceMarker, TransformSourceMap};
    use biome_js_parser::{parse, parse_module, JsParserOptions};
    use biome_js_syntax::{
        JsArrayExpression, JsBinaryExpression, JsExpressionStatement, JsFileSource,
        JsIdentifierExpression, JsLogicalExpression, JsSequenceExpression,
        JsStringLiteralExpression, JsSyntaxNode, JsUnaryExpression, JsxTagExpression,
    };
    use biome_rowan::{AstNode, SyntaxRewriter, TextSize};

    #[test]
    fn rebalances_logical_expressions() {
        let root = parse_module("a && (b && c)", JsParserOptions::default()).syntax();

        let transformed = JsFormatSyntaxRewriter::default().transform(root.clone());

        // Changed the root tree
        assert_ne!(&transformed, &root);

        // Removes parentheses
        assert_eq!(&transformed.text_with_trivia().to_string(), "a && b && c");

        let mut logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(JsLogicalExpression::cast)
            .collect();

        assert_eq!(logical_expressions.len(), 2);

        let left = logical_expressions.pop().unwrap();
        let top = logical_expressions.pop().unwrap();

        assert_eq!(top.left().unwrap().syntax(), left.syntax());
        assert_eq!(&top.right().unwrap().to_trimmed_string(), "c");

        assert_eq!(left.left().unwrap().to_trimmed_string(), "a");
        assert_eq!(left.right().unwrap().to_trimmed_string(), "b");
    }

    #[test]
    fn only_rebalances_logical_expressions_with_same_operator() {
        let root = parse_module("a && (b || c)", JsParserOptions::default()).syntax();
        let transformed = JsFormatSyntaxRewriter::default().transform(root);

        // Removes parentheses
        assert_eq!(&transformed.text_with_trivia().to_string(), "a && b || c");

        let logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(JsLogicalExpression::cast)
            .collect();

        assert_eq!(logical_expressions.len(), 2);

        let top = logical_expressions.first().unwrap();
        let right = logical_expressions.last().unwrap();

        assert_eq!(top.left().unwrap().to_trimmed_string(), "a");
        assert_eq!(top.right().unwrap().syntax(), right.syntax());
        assert_eq!(right.left().unwrap().to_trimmed_string(), "b");
        assert_eq!(right.right().unwrap().to_trimmed_string(), "c");
    }

    #[test]
    fn single_parentheses() {
        let (transformed, source_map) = source_map_test("(a)");

        assert_eq!(&transformed.text_with_trivia(), "a");

        let identifier = transformed
            .descendants()
            .find_map(JsIdentifierExpression::cast)
            .unwrap();

        assert_eq!(source_map.trimmed_source_text(identifier.syntax()), "(a)");
    }

    #[test]
    fn nested_parentheses() {
        let (transformed, source_map) = source_map_test("((a))");

        assert_eq!(&transformed.text_with_trivia(), "a");

        let identifier = transformed
            .descendants()
            .find_map(JsIdentifierExpression::cast)
            .unwrap();

        assert_eq!(source_map.trimmed_source_text(identifier.syntax()), "((a))");
    }

    #[test]
    fn test_logical_expression_source_map() {
        let (transformed, source_map) = source_map_test("(a && (b && c))");

        let logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(JsLogicalExpression::cast)
            .collect();

        assert_eq!(2, logical_expressions.len());

        assert_eq!(
            source_map.trimmed_source_text(logical_expressions[0].syntax()),
            "(a && (b && c))"
        );

        assert_eq!(
            source_map.trimmed_source_text(logical_expressions[1].syntax()),
            "a && (b"
        );
    }

    #[test]
    fn adjacent_nodes() {
        let (transformed, source_map) = source_map_test("(a + b)");

        assert_eq!(&transformed.text_with_trivia(), "a + b");

        let identifiers: Vec<_> = transformed
            .descendants()
            .filter_map(JsIdentifierExpression::cast)
            .collect();

        assert_eq!(2, identifiers.len());
        // Parentheses should be associated with the binary expression
        assert_eq!(source_map.trimmed_source_text(identifiers[0].syntax()), "a");
        assert_eq!(source_map.trimmed_source_text(identifiers[1].syntax()), "b");

        let binary = transformed
            .descendants()
            .find_map(JsBinaryExpression::cast)
            .unwrap();
        assert_eq!(source_map.trimmed_source_text(binary.syntax()), "(a + b)")
    }

    #[test]
    fn intersecting_ranges() {
        let (transformed, source_map) = source_map_test("(interface, \"foo\");");

        assert_eq!(&transformed.text_with_trivia(), "interface, \"foo\";");

        let string_literal = transformed
            .descendants()
            .find_map(JsStringLiteralExpression::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(string_literal.syntax()),
            "\"foo\""
        );

        let sequence = transformed
            .descendants()
            .find_map(JsSequenceExpression::cast)
            .unwrap();
        assert_eq!(
            source_map.trimmed_source_text(sequence.syntax()),
            "(interface, \"foo\")"
        );
    }

    #[test]
    fn deep() {
        let src = r#"[
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
];"#;

        let (transformed, source_map) = source_map_test(src);

        let array = transformed
            .descendants()
            .find_map(JsArrayExpression::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(array.syntax()),
            r#"[
    (2*n)/(r-l), 0,            (r+l)/(r-l),  0,
    0,           (2*n)/(t-b),  (t+b)/(t-b),  0,
    0,           0,           -(f+n)/(f-n), -(2*f*n)/(f-n),
    0,           0,           -1,            0,
]"#
        );
    }

    #[test]
    fn enclosing_node() {
        let (transformed, source_map) = source_map_test("(a + b);");

        assert_eq!(&transformed.text_with_trivia(), "a + b;");

        let expression_statement = transformed
            .descendants()
            .find_map(JsExpressionStatement::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(expression_statement.syntax()),
            "(a + b);"
        );
    }

    #[test]
    fn trailing_whitespace() {
        let (transformed, source_map) = source_map_test("(a + b   );");

        assert_eq!(&transformed.text_with_trivia(), "a + b   ;");

        let binary = transformed
            .descendants()
            .find_map(JsBinaryExpression::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(binary.syntax()),
            "(a + b   )"
        );
    }

    #[test]
    fn first_token_leading_whitespace() {
        let (transformed, _) = source_map_test("a;\n(\n a + b);");

        // Trims the leading whitespace in front of the expression's first token.
        assert_eq!(&transformed.text_with_trivia(), "a;\na + b;");
    }

    #[test]
    fn first_token_leading_whitespace_before_comment() {
        let (transformed, _) = source_map_test("a;(\n\n/* comment */\n a + b);");

        // Keeps at least one new line before a leading comment.
        assert_eq!(
            &transformed.text_with_trivia(),
            "a;\n/* comment */\n a + b;"
        );
    }

    #[test]
    fn comments() {
        let (transformed, source_map) =
            source_map_test("/* outer */ (/* left */ a + b /* right */) /* outer end */;");

        assert_eq!(
            &transformed.text_with_trivia(),
            "/* outer */ /* left */ a + b /* right */ /* outer end */;"
        );

        let binary = transformed
            .descendants()
            .find_map(JsBinaryExpression::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(binary.syntax()),
            "(/* left */ a + b /* right */)"
        );
    }

    #[test]
    fn parentheses() {
        let (transformed, source_map) = source_map_test(
            r#"!(
  /* foo */
  x
);
!(
  x // foo
);
!(
  /* foo */
  x + y
);
!(
  x + y
  /* foo */
);
!(
  x + y // foo
);"#,
        );

        let unary_expressions = transformed
            .descendants()
            .filter_map(JsUnaryExpression::cast)
            .collect::<Vec<_>>();
        assert_eq!(unary_expressions.len(), 5);

        assert_eq!(
            source_map.trimmed_source_text(unary_expressions[0].syntax()),
            r#"!(
  /* foo */
  x
)"#
        );

        assert_eq!(
            source_map.source_range(unary_expressions[1].syntax().text_range_with_trivia()),
            TextRange::new(TextSize::from(21), TextSize::from(36))
        );

        assert_eq!(
            source_map.trimmed_source_text(unary_expressions[1].syntax()),
            r#"!(
  x // foo
)"#
        );

        assert_eq!(
            source_map.trimmed_source_text(unary_expressions[2].syntax()),
            r#"!(
  /* foo */
  x + y
)"#
        );

        assert_eq!(
            source_map.trimmed_source_text(unary_expressions[3].syntax()),
            r#"!(
  x + y
  /* foo */
)"#
        );

        assert_eq!(
            source_map.trimmed_source_text(unary_expressions[4].syntax()),
            r#"!(
  x + y // foo
)"#
        );
    }

    #[test]
    fn nested_parentheses_with_whitespace() {
        let (transformed, source_map) = source_map_test(
            r#"function f() {
	return (
		(
			// prettier-ignore
			/* $FlowFixMe(>=0.53.0) */
			<JSX />
		)
	);
}"#,
        );

        let tag_expression = transformed
            .descendants()
            .find_map(JsxTagExpression::cast)
            .unwrap();

        assert_eq!(
            source_map.trimmed_source_text(tag_expression.syntax()),
            r#"(
		(
			// prettier-ignore
			/* $FlowFixMe(>=0.53.0) */
			<JSX />
		)
	)"#
        );
    }

    fn source_map_test(input: &str) -> (JsSyntaxNode, TransformSourceMap) {
        let tree = parse(input, JsFileSource::jsx(), JsParserOptions::default()).syntax();

        let mut rewriter = JsFormatSyntaxRewriter::default();
        let transformed = rewriter.transform(tree);
        let source_map = rewriter.finish();

        (transformed, source_map)
    }

    #[test]
    fn mappings() {
        let (transformed, source_map) = source_map_test("(((a * b) * c)) / 3");

        let formatted =
            format_node(JsFormatOptions::new(JsFileSource::default()), &transformed).unwrap();
        let printed = formatted.print().unwrap();

        assert_eq!(printed.as_code(), "(a * b * c) / 3;\n");

        let mapped = source_map.map_printed(printed);
        let markers = mapped.into_sourcemap();

        assert_eq!(
            markers,
            vec![
                // `(`
                SourceMarker {
                    source: TextSize::from(3),
                    dest: TextSize::from(0)
                },
                // `a`
                SourceMarker {
                    source: TextSize::from(3),
                    dest: TextSize::from(1)
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(4),
                    dest: TextSize::from(2)
                },
                // `*`
                SourceMarker {
                    source: TextSize::from(5),
                    dest: TextSize::from(3)
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(6),
                    dest: TextSize::from(4)
                },
                // `b`
                SourceMarker {
                    source: TextSize::from(7),
                    dest: TextSize::from(5)
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(9),
                    dest: TextSize::from(6)
                },
                // `*`
                SourceMarker {
                    source: TextSize::from(10),
                    dest: TextSize::from(7)
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(11),
                    dest: TextSize::from(8)
                },
                // `c`
                SourceMarker {
                    source: TextSize::from(12),
                    dest: TextSize::from(9)
                },
                // `)`
                SourceMarker {
                    source: TextSize::from(15),
                    dest: TextSize::from(10),
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(15),
                    dest: TextSize::from(11),
                },
                // `/`
                SourceMarker {
                    source: TextSize::from(16),
                    dest: TextSize::from(12),
                },
                // ` `
                SourceMarker {
                    source: TextSize::from(17),
                    dest: TextSize::from(13),
                },
                // `3`
                SourceMarker {
                    source: TextSize::from(18),
                    dest: TextSize::from(14),
                },
                // `;`
                SourceMarker {
                    source: TextSize::from(19),
                    dest: TextSize::from(15),
                },
                // `\n`
                SourceMarker {
                    source: TextSize::from(19),
                    dest: TextSize::from(16),
                },
            ]
        );
    }
}
