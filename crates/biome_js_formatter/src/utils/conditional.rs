use crate::prelude::*;
use biome_formatter::{
    write, CstFormatContext, FormatContext, FormatOptions, FormatOwnedWithRule, FormatRefWithRule,
    FormatRuleWithOptions,
};

use crate::{AsFormat, IntoFormat};
use biome_js_syntax::{
    AnyJsExpression, AnyTsType, JsAssignmentExpression, JsConditionalExpression, JsFileSource,
    JsInitializerClause, JsReturnStatement, JsStaticMemberExpression, JsSyntaxKind, JsSyntaxNode,
    JsSyntaxToken, JsThrowStatement, JsUnaryExpression, JsYieldArgument, TsConditionalType,
};
use biome_rowan::{declare_node_union, AstNode, SyntaxResult};

declare_node_union! {
    pub AnyJsConditional = JsConditionalExpression | TsConditionalType
}

impl AsFormat<JsFormatContext> for AnyJsConditional {
    type Format<'a> = FormatRefWithRule<'a, AnyJsConditional, FormatJsAnyConditionalRule>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatJsAnyConditionalRule::default())
    }
}

impl IntoFormat<JsFormatContext> for AnyJsConditional {
    type Format = FormatOwnedWithRule<AnyJsConditional, FormatJsAnyConditionalRule>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatJsAnyConditionalRule::default())
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatJsAnyConditionalRule {
    /// Whether the parent is a jsx conditional chain.
    /// Gets passed through from the root to the consequent and alternate of [JsConditionalExpression]s.
    ///
    /// Doesn't apply for [TsConditionalType].
    jsx_chain: ConditionalJsxChain,
}

impl FormatRuleWithOptions<AnyJsConditional> for FormatJsAnyConditionalRule {
    type Options = ConditionalJsxChain;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.jsx_chain = options;
        self
    }
}

impl FormatRule<AnyJsConditional> for FormatJsAnyConditionalRule {
    type Context = JsFormatContext;

    fn fmt(
        &self,
        conditional: &AnyJsConditional,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        let syntax = conditional.syntax();
        let consequent = conditional.consequent()?;
        let alternate = conditional.alternate()?;
        let indent_style = f.options().indent_style();
        let layout = self.layout(conditional, f.context().options().source_type());
        let jsx_chain = layout.jsx_chain().unwrap_or(self.jsx_chain);

        let format_consequent_and_alternate = format_with(|f| {
            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.question_mark_token().format(),
                    space()
                ]
            )?;

            let is_consequent_nested = consequent.syntax().kind() == syntax.kind();
            let consequent = format_with(|f| {
                if indent_style.is_space() {
                    write!(f, [align(2, &consequent)])
                } else {
                    write!(f, [indent(&consequent)])
                }
            });
            if is_consequent_nested {
                // Add parentheses around the consequent if it is a conditional expression and fits on the same line
                // so that it's easier to identify the parts that belong to a conditional expression.
                // `a ? b ? c: d : e` -> `a ? (b ? c: d) : e
                write!(
                    f,
                    [
                        if_group_fits_on_line(&text("(")),
                        consequent,
                        if_group_fits_on_line(&text(")"))
                    ]
                )?;
            } else {
                write!(f, [consequent])?;
            }

            write!(
                f,
                [
                    soft_line_break_or_space(),
                    conditional.colon_token().format(),
                    space()
                ]
            )?;
            let alternate = format_with(|f| {
                if indent_style.is_space() {
                    write!(f, [align(2, &alternate)])
                } else {
                    write!(f, [indent(&alternate)])
                }
            });
            write!(f, [alternate])
        });

        let format_tail_with_indent = format_with(|f: &mut JsFormatter| {
            match conditional {
                AnyJsConditional::JsConditionalExpression(conditional) if jsx_chain.is_chain() => {
                    write!(
                        f,
                        [
                            space(),
                            conditional.question_mark_token().format(),
                            space(),
                            format_jsx_chain_consequent(consequent.as_expression().unwrap()),
                            space(),
                            conditional.colon_token().format(),
                            space(),
                            format_jsx_chain_alternate(alternate.as_expression().unwrap())
                        ]
                    )
                }
                _ => {
                    // Add an extra level of indent to nested consequences.
                    if layout.is_nested_consequent() {
                        // This may look silly but the `dedent` is to remove the outer `align` added by the parent's formatting of the consequent.
                        // The `indent` is necessary to indent the content by one level with a tab.
                        // Adding an `indent` without the `dedent` would result in the `outer` align being converted
                        // into a `indent` + the `indent` added here, ultimately resulting in a two-level indention.
                        write!(f, [dedent(&indent(&format_consequent_and_alternate))])
                    } else {
                        format_consequent_and_alternate.fmt(f)
                    }
                }
            }
        });

        let should_extra_indent = self.should_extra_indent(conditional, &layout);

        let format_inner = format_with(|f| {
            write!(
                f,
                [FormatConditionalTest {
                    conditional,
                    layout: &layout,
                }]
            )?;

            // Indent the `consequent` and `alternate` **only if** this is the root conditional expression
            // OR this is the `test` of a conditional expression.
            if jsx_chain.is_no_chain() && (layout.is_root() || layout.is_nested_test()) {
                write!(f, [indent(&format_tail_with_indent)])?;
            } else {
                // Don't indent for nested `alternate`s or `consequence`s
                write!(f, [format_tail_with_indent])?;
            }

            let break_closing_parentheses = jsx_chain.is_no_chain()
                && self.is_parent_static_member_expression(conditional, &layout);

            // Add a soft line break in front of the closing `)` in case the parent is a static member expression
            // ```
            // (veryLongCondition
            //      ? a
            //      : b // <- enforce line break here if the conditional breaks
            // ).more
            // ```
            if break_closing_parentheses && !should_extra_indent {
                write!(f, [soft_line_break()])?;
            }

            Ok(())
        });

        let grouped = format_with(|f| {
            if layout.is_root() {
                group(&format_inner).fmt(f)
            } else {
                format_inner.fmt(f)
            }
        });

        let has_multiline_comment = {
            let comments = f.context().comments();

            let has_block_comment = |syntax: &JsSyntaxNode| {
                comments
                    .leading_trailing_comments(syntax)
                    .any(|comment| comment.kind().is_block())
            };

            let test_has_block_comments = match conditional {
                AnyJsConditional::JsConditionalExpression(expression) => {
                    has_block_comment(expression.test()?.syntax())
                }
                AnyJsConditional::TsConditionalType(ty) => {
                    has_block_comment(ty.check_type()?.syntax())
                        || has_block_comment(ty.extends_type()?.syntax())
                }
            };

            test_has_block_comments
                || has_block_comment(consequent.syntax())
                || has_block_comment(alternate.syntax())
        };

        if layout.is_nested_test() || should_extra_indent {
            group(&soft_block_indent(&grouped))
                .should_expand(has_multiline_comment)
                .fmt(f)
        } else {
            if has_multiline_comment {
                write!(f, [expand_parent()])?;
            }

            grouped.fmt(f)
        }
    }
}

impl FormatJsAnyConditionalRule {
    fn layout(
        &self,
        conditional: &AnyJsConditional,
        source_type: JsFileSource,
    ) -> ConditionalLayout {
        match conditional.syntax().parent() {
            Some(parent) if parent.kind() == conditional.syntax().kind() => {
                let conditional_parent = AnyJsConditional::unwrap_cast(parent);

                if conditional_parent.is_test(conditional.syntax()) {
                    ConditionalLayout::NestedTest {
                        parent: conditional_parent,
                    }
                } else if conditional_parent.is_alternate(conditional.syntax()) {
                    ConditionalLayout::NestedAlternate {
                        parent: conditional_parent,
                    }
                } else {
                    ConditionalLayout::NestedConsequent {
                        parent: conditional_parent,
                    }
                }
            }
            parent => {
                let is_jsx_chain = match conditional {
                    AnyJsConditional::JsConditionalExpression(conditional)
                        if source_type.variant().is_jsx() =>
                    {
                        is_jsx_conditional_chain(conditional)
                    }
                    _ => false,
                };

                ConditionalLayout::Root {
                    parent,
                    jsx_chain: is_jsx_chain.into(),
                }
            }
        }
    }

    /// It is desired to add an extra indent if this conditional is a [JsConditionalExpression] and is directly inside
    /// of a member chain:
    ///
    /// ```javascript
    /// // Input
    /// return (a ? b : c).member
    ///
    /// // Default
    /// return (a
    ///     ? b
    ///     : c
    /// ).member
    ///
    /// // Preferred
    /// return (
    ///     a
    ///         ? b
    ///         : c
    /// ).member
    /// ```
    fn should_extra_indent(
        &self,
        conditional: &AnyJsConditional,
        layout: &ConditionalLayout,
    ) -> bool {
        enum Ancestor {
            MemberChain(AnyJsExpression),
            Root(JsSyntaxNode),
        }

        let conditional = match conditional {
            AnyJsConditional::JsConditionalExpression(conditional) => conditional,
            AnyJsConditional::TsConditionalType(_) => {
                return false;
            }
        };

        let ancestors = layout
            .parent()
            .into_iter()
            .flat_map(|parent| parent.ancestors());
        let mut parent = None;
        let mut expression = AnyJsExpression::from(conditional.clone());

        // This tries to find the start of a member chain by iterating over all ancestors of the conditional.
        // The iteration "breaks" as soon as a non-member-chain node is found.
        for ancestor in ancestors {
            let ancestor = match AnyJsExpression::try_cast(ancestor) {
                Ok(AnyJsExpression::JsCallExpression(call_expression)) => {
                    if call_expression.callee().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(call_expression.into())
                    } else {
                        Ancestor::Root(call_expression.into_syntax())
                    }
                }

                Ok(AnyJsExpression::JsStaticMemberExpression(member_expression)) => {
                    if member_expression.object().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                Ok(AnyJsExpression::JsComputedMemberExpression(member_expression)) => {
                    if member_expression.object().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(member_expression.into())
                    } else {
                        Ancestor::Root(member_expression.into_syntax())
                    }
                }
                Ok(AnyJsExpression::TsNonNullAssertionExpression(non_null_assertion)) => {
                    if non_null_assertion.expression().as_ref() == Ok(&expression) {
                        Ancestor::MemberChain(non_null_assertion.into())
                    } else {
                        Ancestor::Root(non_null_assertion.into_syntax())
                    }
                }
                Ok(AnyJsExpression::JsNewExpression(new_expression)) => {
                    // Skip over new expressions
                    if new_expression.callee().as_ref() == Ok(&expression) {
                        parent = new_expression.syntax().parent();
                        expression = new_expression.into();
                        break;
                    }

                    Ancestor::Root(new_expression.into_syntax())
                }
                Ok(AnyJsExpression::TsAsExpression(as_expression)) => {
                    if as_expression.expression().as_ref() == Ok(&expression) {
                        parent = as_expression.syntax().parent();
                        expression = as_expression.into();
                        break;
                    }

                    Ancestor::Root(as_expression.into_syntax())
                }
                Ok(AnyJsExpression::TsSatisfiesExpression(satisfies_expression)) => {
                    if satisfies_expression.expression().as_ref() == Ok(&expression) {
                        parent = satisfies_expression.syntax().parent();
                        expression = satisfies_expression.into();
                        break;
                    }

                    Ancestor::Root(satisfies_expression.into_syntax())
                }
                Ok(ancestor) => Ancestor::Root(ancestor.into_syntax()),
                Err(ancestor) => Ancestor::Root(ancestor),
            };

            match ancestor {
                Ancestor::MemberChain(left) => {
                    // Store the node that is highest in the member chain
                    expression = left;
                }
                Ancestor::Root(root) => {
                    parent = Some(root);
                    break;
                }
            }
        }

        // Don't indent if this conditional isn't part of a member chain.
        // e.g. don't indent for `return a ? b : c`, only for `return (a ? b : c).member`
        if expression.syntax() == conditional.syntax() {
            return false;
        }

        match parent {
            None => false,
            Some(parent) => {
                let argument = match parent.kind() {
                    JsSyntaxKind::JS_INITIALIZER_CLAUSE => {
                        let initializer = JsInitializerClause::unwrap_cast(parent);
                        initializer.expression().ok().map(AnyJsExpression::from)
                    }
                    JsSyntaxKind::JS_RETURN_STATEMENT => {
                        let return_statement = JsReturnStatement::unwrap_cast(parent);
                        return_statement.argument().map(AnyJsExpression::from)
                    }
                    JsSyntaxKind::JS_THROW_STATEMENT => {
                        let throw_statement = JsThrowStatement::unwrap_cast(parent);
                        throw_statement.argument().ok().map(AnyJsExpression::from)
                    }
                    JsSyntaxKind::JS_UNARY_EXPRESSION => {
                        let unary_expression = JsUnaryExpression::unwrap_cast(parent);
                        unary_expression.argument().ok().map(AnyJsExpression::from)
                    }
                    JsSyntaxKind::JS_YIELD_ARGUMENT => {
                        let yield_argument = JsYieldArgument::unwrap_cast(parent);
                        yield_argument.expression().ok().map(AnyJsExpression::from)
                    }
                    JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                        let assignment_expression = JsAssignmentExpression::unwrap_cast(parent);
                        assignment_expression
                            .right()
                            .ok()
                            .map(AnyJsExpression::from)
                    }
                    _ => None,
                };

                argument.is_some_and(|argument| argument == expression)
            }
        }
    }

    /// Returns `true` if this is the root conditional expression and the parent is a [JsStaticMemberExpression].
    fn is_parent_static_member_expression(
        &self,
        conditional: &AnyJsConditional,
        layout: &ConditionalLayout,
    ) -> bool {
        if !conditional.is_conditional_expression() {
            return false;
        }

        match layout {
            ConditionalLayout::Root {
                parent: Some(parent),
                ..
            } => JsStaticMemberExpression::can_cast(parent.kind()),
            _ => false,
        }
    }
}

/// Formats the test conditional of a conditional expression.
struct FormatConditionalTest<'a> {
    conditional: &'a AnyJsConditional,
    layout: &'a ConditionalLayout,
}

impl Format<JsFormatContext> for FormatConditionalTest<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        let indent_style = f.options().indent_style();
        let format_inner = format_with(|f| match self.conditional {
            AnyJsConditional::JsConditionalExpression(conditional) => {
                write!(f, [conditional.test().format()])
            }
            AnyJsConditional::TsConditionalType(conditional) => {
                write!(
                    f,
                    [
                        conditional.check_type().format(),
                        space(),
                        conditional.extends_token().format(),
                        space(),
                        conditional.extends_type().format()
                    ]
                )
            }
        });

        if self.layout.is_nested_alternate() {
            if indent_style.is_space() {
                write!(f, [align(2, &format_inner)])
            } else {
                write!(f, [indent(&format_inner)])
            }
        } else {
            format_inner.fmt(f)
        }
    }
}

declare_node_union! {
    ExpressionOrType = AnyJsExpression | AnyTsType
}

impl ExpressionOrType {
    fn as_expression(&self) -> Option<&AnyJsExpression> {
        match self {
            ExpressionOrType::AnyJsExpression(expression) => Some(expression),
            ExpressionOrType::AnyTsType(_) => None,
        }
    }
}

impl Format<JsFormatContext> for ExpressionOrType {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match self {
            ExpressionOrType::AnyJsExpression(expression) => expression.format().fmt(f),
            ExpressionOrType::AnyTsType(ty) => ty.format().fmt(f),
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum ConditionalLayout {
    /// Conditional that is the `alternate` of another conditional.
    ///
    /// The `test` condition of a nested alternated is aligned with the parent's `:`.
    ///
    /// ```javascript
    /// outerCondition
    ///     ? consequent
    ///     : nestedAlternate +
    ///       binary + // <- notice how the content is aligned to the `: `
    ///     ? consequentOfnestedAlternate
    ///     : alternateOfNestedAlternate;
    /// ```
    NestedAlternate { parent: AnyJsConditional },

    /// Conditional that is the `test` of another conditional.
    ///
    /// ```javascript
    /// (
    ///     a              // <-- Note the extra indent here
    ///         ? b
    ///         : c
    ///  )
    ///     ? d
    ///     : e;
    /// ```
    ///
    /// Indents the
    NestedTest { parent: AnyJsConditional },

    /// Conditional that is the `consequent` of another conditional.
    ///
    /// ```javascript
    /// condition1
    ///     ? condition2
    ///         ? consequent2 // <-- consequent and alternate gets indented
    ///         : alternate2
    ///     : alternate1;
    /// ```
    NestedConsequent { parent: AnyJsConditional },

    /// This conditional isn't a child of another conditional.
    ///
    /// ```javascript
    /// return a ? b : c;
    /// ```
    Root {
        /// The closest ancestor that isn't a parenthesized node.
        parent: Option<JsSyntaxNode>,

        jsx_chain: ConditionalJsxChain,
    },
}

/// A [JsConditionalExpression] that itself or any of its parent's [JsConditionalExpression] have a a [JsxTagExpression]
/// as its [`test`](JsConditionalExpression::test), [`consequent`](JsConditionalExpression::consequent) or [`alternate`](JsConditionalExpression::alternate).
///
/// Parenthesizes the `consequent` and `alternate` if it the group breaks except if the expressions are
/// * `null`
/// * `undefined`
/// * or a nested [JsConditionalExpression] in the alternate branch
///
/// ```javascript
/// abcdefgh? (
///   <Element>
///     <Sub />
///     <Sub />
///   </Element>
/// ) : (
///   <Element2>
///     <Sub />
///     <Sub />
///   </Element2>
/// );
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum ConditionalJsxChain {
    Chain,
    #[default]
    NoChain,
}

impl ConditionalJsxChain {
    pub const fn is_chain(&self) -> bool {
        matches!(self, ConditionalJsxChain::Chain)
    }
    pub const fn is_no_chain(&self) -> bool {
        matches!(self, ConditionalJsxChain::NoChain)
    }
}

impl From<bool> for ConditionalJsxChain {
    fn from(value: bool) -> Self {
        match value {
            true => ConditionalJsxChain::Chain,
            false => ConditionalJsxChain::NoChain,
        }
    }
}

impl ConditionalLayout {
    const fn jsx_chain(&self) -> Option<ConditionalJsxChain> {
        match self {
            ConditionalLayout::NestedAlternate { .. }
            | ConditionalLayout::NestedTest { .. }
            | ConditionalLayout::NestedConsequent { .. } => None,
            ConditionalLayout::Root { jsx_chain, .. } => Some(*jsx_chain),
        }
    }

    const fn is_root(&self) -> bool {
        matches!(self, ConditionalLayout::Root { .. })
    }

    /// Returns the parent node, if any
    fn parent(&self) -> Option<&JsSyntaxNode> {
        match self {
            ConditionalLayout::NestedAlternate { parent, .. }
            | ConditionalLayout::NestedTest { parent, .. }
            | ConditionalLayout::NestedConsequent { parent, .. } => Some(parent.syntax()),
            ConditionalLayout::Root { parent, .. } => parent.as_ref(),
        }
    }

    const fn is_nested_test(&self) -> bool {
        matches!(self, ConditionalLayout::NestedTest { .. })
    }

    const fn is_nested_alternate(&self) -> bool {
        matches!(self, ConditionalLayout::NestedAlternate { .. })
    }

    const fn is_nested_consequent(&self) -> bool {
        matches!(self, ConditionalLayout::NestedConsequent { .. })
    }
}

impl AnyJsConditional {
    /// Returns `true` if `node` is the `test` of this conditional.
    fn is_test(&self, node: &JsSyntaxNode) -> bool {
        match self {
            AnyJsConditional::JsConditionalExpression(conditional) => conditional
                .test()
                .ok()
                .is_some_and(|resolved| resolved.syntax() == node),
            AnyJsConditional::TsConditionalType(conditional) => {
                conditional.check_type().map(AstNode::into_syntax).as_ref() == Ok(node)
                    || conditional
                        .extends_type()
                        .map(AstNode::into_syntax)
                        .as_ref()
                        == Ok(node)
            }
        }
    }

    pub(crate) fn question_mark_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsConditional::JsConditionalExpression(conditional) => {
                conditional.question_mark_token()
            }
            AnyJsConditional::TsConditionalType(conditional) => conditional.question_mark_token(),
        }
    }

    fn consequent(&self) -> SyntaxResult<ExpressionOrType> {
        match self {
            AnyJsConditional::JsConditionalExpression(conditional) => {
                conditional.consequent().map(ExpressionOrType::from)
            }
            AnyJsConditional::TsConditionalType(conditional) => {
                conditional.true_type().map(ExpressionOrType::from)
            }
        }
    }

    pub(crate) fn colon_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsConditional::JsConditionalExpression(conditional) => conditional.colon_token(),
            AnyJsConditional::TsConditionalType(conditional) => conditional.colon_token(),
        }
    }

    fn alternate(&self) -> SyntaxResult<ExpressionOrType> {
        match self {
            AnyJsConditional::JsConditionalExpression(conditional) => {
                conditional.alternate().map(ExpressionOrType::from)
            }
            AnyJsConditional::TsConditionalType(conditional) => {
                conditional.false_type().map(ExpressionOrType::from)
            }
        }
    }

    /// Returns `true` if the passed node is the `alternate` of this conditional expression.
    fn is_alternate(&self, node: &JsSyntaxNode) -> bool {
        let alternate = match self {
            AnyJsConditional::JsConditionalExpression(conditional) => {
                conditional.alternate().map(AstNode::into_syntax).ok()
            }
            AnyJsConditional::TsConditionalType(ts_conditional) => {
                ts_conditional.false_type().ok().map(AstNode::into_syntax)
            }
        };

        alternate.as_ref() == Some(node)
    }

    const fn is_conditional_expression(&self) -> bool {
        matches!(self, AnyJsConditional::JsConditionalExpression(_))
    }
}

fn is_jsx_conditional_chain(outer_most: &JsConditionalExpression) -> bool {
    fn recurse(expression: SyntaxResult<AnyJsExpression>) -> bool {
        use AnyJsExpression::*;

        match expression {
            Ok(JsConditionalExpression(conditional)) => is_jsx_conditional_chain(&conditional),
            Ok(JsxTagExpression(_)) => true,
            _ => false,
        }
    }

    recurse(outer_most.test())
        || recurse(outer_most.consequent())
        || recurse(outer_most.alternate())
}

fn format_jsx_chain_consequent(expression: &AnyJsExpression) -> FormatJsxChainExpression {
    FormatJsxChainExpression {
        expression,
        alternate: false,
    }
}

fn format_jsx_chain_alternate(alternate: &AnyJsExpression) -> FormatJsxChainExpression {
    FormatJsxChainExpression {
        expression: alternate,
        alternate: true,
    }
}

/// Wraps all expressions in parentheses if they break EXCEPT
/// * Nested conditionals in the alternate
/// * `null`
/// * `undefined`
struct FormatJsxChainExpression<'a> {
    expression: &'a AnyJsExpression,
    alternate: bool,
}

impl Format<JsFormatContext> for FormatJsxChainExpression<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use AnyJsExpression::*;

        let no_wrap = match self.expression {
            JsIdentifierExpression(identifier) if identifier.name()?.is_undefined() => true,
            AnyJsLiteralExpression(
                biome_js_syntax::AnyJsLiteralExpression::JsNullLiteralExpression(_),
            ) => true,
            JsConditionalExpression(_) if self.alternate => true,
            _ => false,
        };

        let format_expression = format_with(|f| match self.expression {
            JsConditionalExpression(conditional) => {
                write!(
                    f,
                    [conditional
                        .format()
                        .with_options(ConditionalJsxChain::Chain)]
                )
            }
            expression => {
                write!(f, [expression.format()])
            }
        });

        if no_wrap {
            write!(f, [format_expression])
        } else {
            write!(
                f,
                [
                    if_group_breaks(&text("(")),
                    soft_block_indent(&format_expression),
                    if_group_breaks(&text(")"))
                ]
            )
        }
    }
}
