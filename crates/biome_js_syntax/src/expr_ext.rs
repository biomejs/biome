//! Extensions for things which are not easily generated in ast expr nodes
use crate::numbers::parse_js_number;
use crate::static_value::StaticValue;
use crate::{
    inner_string_text, AnyJsArrayElement, AnyJsArrowFunctionParameters, AnyJsCallArgument,
    AnyJsClassMemberName, AnyJsExpression, AnyJsFunctionBody, AnyJsLiteralExpression, AnyJsName,
    AnyJsObjectMemberName, AnyJsTemplateElement, AnyTsEnumMemberName, JsArrayExpression,
    JsArrayHole, JsAssignmentExpression, JsBinaryExpression, JsCallArgumentList, JsCallArguments,
    JsCallExpression, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsLiteralMemberName, JsLogicalExpression, JsNewExpression, JsNumberLiteralExpression,
    JsObjectExpression, JsPostUpdateExpression, JsPreUpdateExpression, JsReferenceIdentifier,
    JsRegexLiteralExpression, JsStaticMemberExpression, JsStringLiteralExpression, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsTemplateChunkElement, JsTemplateExpression, JsUnaryExpression,
    JsWhileStatement, OperatorPrecedence, TsStringLiteralType, T,
};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, NodeOrToken, SyntaxNodeCast,
    SyntaxResult, TextRange, TextSize, TokenText,
};
use core::iter;

const GLOBAL_THIS: &str = "globalThis";
const UNDEFINED: &str = "undefined";
const WINDOW: &str = "window";

declare_node_union! {
    pub JsNewOrCallExpression = JsNewExpression | JsCallExpression
}

impl JsNewOrCallExpression {
    pub fn callee(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            JsNewOrCallExpression::JsNewExpression(node) => node.callee(),
            JsNewOrCallExpression::JsCallExpression(node) => node.callee(),
        }
    }

    pub fn arguments(&self) -> Option<JsCallArguments> {
        match self {
            JsNewOrCallExpression::JsNewExpression(node) => node.arguments(),
            JsNewOrCallExpression::JsCallExpression(node) => node.arguments().ok(),
        }
    }
}
impl From<JsNewOrCallExpression> for AnyJsExpression {
    fn from(value: JsNewOrCallExpression) -> Self {
        match value {
            JsNewOrCallExpression::JsNewExpression(expr) => Self::JsNewExpression(expr),
            JsNewOrCallExpression::JsCallExpression(expr) => Self::JsCallExpression(expr),
        }
    }
}

impl From<AnyJsCallArgument> for AnyJsArrayElement {
    fn from(value: AnyJsCallArgument) -> Self {
        match value {
            AnyJsCallArgument::AnyJsExpression(expr) => Self::AnyJsExpression(expr),
            AnyJsCallArgument::JsSpread(spread) => Self::JsSpread(spread),
        }
    }
}

impl JsReferenceIdentifier {
    /// Returns `true` if this identifier refers to the `undefined` symbol.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make::{js_reference_identifier, ident};
    ///
    /// assert!(js_reference_identifier(ident("undefined")).is_undefined());
    /// assert!(!js_reference_identifier(ident("x")).is_undefined());
    /// ```
    pub fn is_undefined(&self) -> bool {
        self.has_name(UNDEFINED)
    }

    /// Returns `true` if this identifier refers to the `globalThis` symbol.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make::{js_reference_identifier, ident};
    ///
    /// assert!(js_reference_identifier(ident("globalThis")).is_global_this());
    /// assert!(!js_reference_identifier(ident("x")).is_global_this());
    /// ```
    pub fn is_global_this(&self) -> bool {
        self.has_name(GLOBAL_THIS)
    }

    /// Returns `true` if this identifier has the given name.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make::{js_reference_identifier, ident};
    ///
    /// assert!(js_reference_identifier(ident("foo")).has_name("foo"));
    /// assert!(!js_reference_identifier(ident("bar")).has_name("foo"));
    /// ```
    pub fn has_name(&self, name: &str) -> bool {
        self.value_token()
            .is_ok_and(|token| token.text_trimmed() == name)
    }

    pub fn name(&self) -> SyntaxResult<TokenText> {
        Ok(self.value_token()?.token_text_trimmed())
    }
}

impl JsLiteralMemberName {
    /// Returns the name of the member as a syntax text
    ///
    /// ## Examples
    ///
    /// Getting the name of a static member containing a string literal
    ///
    /// ```
    /// use biome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use biome_js_factory::JsSyntaxTreeBuilder;
    /// use biome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::JS_STRING_LITERAL, "\"abcd\"");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("abcd", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing a number literal
    ///
    /// ```
    /// use biome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use biome_js_factory::JsSyntaxTreeBuilder;
    /// use biome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::JS_NUMBER_LITERAL, "5");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("5", static_member_name.name().unwrap());
    /// ```
    ///
    /// Getting the name of a static member containing an identifier
    ///
    /// ```
    /// use biome_js_syntax::{JsSyntaxKind, JsLanguage, JsSyntaxNode, JsLiteralMemberName};
    /// use biome_js_factory::JsSyntaxTreeBuilder;
    /// use biome_rowan::AstNode;
    ///
    /// let node: JsSyntaxNode =
    ///     JsSyntaxTreeBuilder::wrap_with_node(JsSyntaxKind::JS_LITERAL_MEMBER_NAME, |builder| {
    ///         builder.token(JsSyntaxKind::IDENT, "abcd");
    ///     });
    ///
    /// let static_member_name = JsLiteralMemberName::unwrap_cast(node);
    ///
    /// assert_eq!("abcd", static_member_name.name().unwrap().text());
    /// ```
    pub fn name(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value()?))
    }
}

/// A binary operation applied to two expressions
///
/// The variants are ordered based on their precedence
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsBinaryOperator {
    /// `<`
    LessThan,
    /// `>`
    GreaterThan,
    /// `<=`
    LessThanOrEqual,
    /// `>=`
    GreaterThanOrEqual,
    /// `==`
    Equality,
    /// `===`
    StrictEquality,
    /// `!=`
    Inequality,
    /// `!==`
    StrictInequality,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Times,
    /// `/`
    Divide,
    /// `%`
    Remainder,
    /// `**`
    Exponent,
    /// `<<`
    LeftShift,
    /// `>>`
    RightShift,
    /// `>>>`
    UnsignedRightShift,
    /// `&`
    BitwiseAnd,
    /// `|`
    BitwiseOr,
    /// `^`
    BitwiseXor,
}

impl JsBinaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            JsBinaryOperator::LessThan
            | JsBinaryOperator::GreaterThan
            | JsBinaryOperator::LessThanOrEqual
            | JsBinaryOperator::GreaterThanOrEqual => OperatorPrecedence::Relational,

            JsBinaryOperator::Equality
            | JsBinaryOperator::StrictEquality
            | JsBinaryOperator::Inequality
            | JsBinaryOperator::StrictInequality => OperatorPrecedence::Equality,

            JsBinaryOperator::Plus | JsBinaryOperator::Minus => OperatorPrecedence::Additive,

            JsBinaryOperator::Times | JsBinaryOperator::Divide | JsBinaryOperator::Remainder => {
                OperatorPrecedence::Multiplicative
            }
            JsBinaryOperator::Exponent => OperatorPrecedence::Exponential,

            JsBinaryOperator::LeftShift
            | JsBinaryOperator::RightShift
            | JsBinaryOperator::UnsignedRightShift => OperatorPrecedence::Shift,

            JsBinaryOperator::BitwiseAnd => OperatorPrecedence::BitwiseAnd,
            JsBinaryOperator::BitwiseOr => OperatorPrecedence::BitwiseOr,
            JsBinaryOperator::BitwiseXor => OperatorPrecedence::BitwiseXor,
        }
    }

    /// Determines whether a binary operator is commutative, meaning that the order of its operands
    /// does not affect the result.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_syntax::JsBinaryOperator;
    ///
    /// let times = JsBinaryOperator::Times;
    ///
    /// assert!(times.is_commutative());
    ///
    ///  let plus = JsBinaryOperator::Plus; // Non-commutative operator
    /// assert!(!plus.is_commutative());
    /// ```
    pub const fn is_commutative(&self) -> bool {
        matches!(
            self,
            JsBinaryOperator::Times
                | JsBinaryOperator::BitwiseAnd
                | JsBinaryOperator::BitwiseOr
                | JsBinaryOperator::BitwiseXor
        )
    }
}

impl JsBinaryExpression {
    pub fn operator(&self) -> SyntaxResult<JsBinaryOperator> {
        let kind = match self.operator_token()?.kind() {
            T![<] => JsBinaryOperator::LessThan,
            T![>] => JsBinaryOperator::GreaterThan,
            T![<=] => JsBinaryOperator::LessThanOrEqual,
            T![>=] => JsBinaryOperator::GreaterThanOrEqual,
            T![==] => JsBinaryOperator::Equality,
            T![===] => JsBinaryOperator::StrictEquality,
            T![!=] => JsBinaryOperator::Inequality,
            T![!==] => JsBinaryOperator::StrictInequality,
            T![+] => JsBinaryOperator::Plus,
            T![-] => JsBinaryOperator::Minus,
            T![*] => JsBinaryOperator::Times,
            T![/] => JsBinaryOperator::Divide,
            T![%] => JsBinaryOperator::Remainder,
            T![**] => JsBinaryOperator::Exponent,
            T![<<] => JsBinaryOperator::LeftShift,
            T![>>] => JsBinaryOperator::RightShift,
            T![>>>] => JsBinaryOperator::UnsignedRightShift,
            T![&] => JsBinaryOperator::BitwiseAnd,
            T![|] => JsBinaryOperator::BitwiseOr,
            T![^] => JsBinaryOperator::BitwiseXor,
            _ => unreachable!(),
        };

        Ok(kind)
    }

    /// Whether this is a numeric operation, such as `+`, `-`, `*`, `%`, `**`.
    pub fn is_numeric_operation(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![+] | T![-] | T![*] | T![/] | T![%] | T![**])
        )
    }

    /// Whether this is a binary operation, such as `<<`, `>>`, `>>>`, `&`, `|`, `^`.
    pub fn is_binary_operation(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![<<] | T![>>] | T![>>>] | T![&] | T![|] | T![^])
        )
    }

    /// Whether this is a comparison operation, such as `>`, `<`, `==`, `!=`, `===`, etc.
    pub fn is_comparison_operator(&self) -> bool {
        matches!(
            self.operator_token().map(|t| t.kind()),
            Ok(T![>] | T![<] | T![>=] | T![<=] | T![==] | T![===] | T![!=] | T![!==])
        )
    }

    /// Whether this is a comparison operation similar to the optional chain
    /// ```js
    /// foo !== undefined;
    /// foo != undefined;
    /// foo !== null;
    /// foo != null;
    ///```
    pub fn is_optional_chain_like(&self) -> SyntaxResult<bool> {
        if matches!(
            self.operator(),
            Ok(JsBinaryOperator::StrictInequality | JsBinaryOperator::Inequality)
        ) {
            Ok(self
                .right()?
                .as_static_value()
                .is_some_and(|x| x.is_null_or_undefined()))
        } else {
            Ok(false)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum JsLogicalOperator {
    /// `??`
    NullishCoalescing,
    /// `||`
    LogicalOr,
    /// `&&`
    LogicalAnd,
}

impl JsLogicalOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            JsLogicalOperator::NullishCoalescing => OperatorPrecedence::Coalesce,
            JsLogicalOperator::LogicalOr => OperatorPrecedence::LogicalOr,
            JsLogicalOperator::LogicalAnd => OperatorPrecedence::LogicalAnd,
        }
    }
}

impl JsLogicalExpression {
    pub fn operator(&self) -> SyntaxResult<JsLogicalOperator> {
        let kind = match self.operator_token()?.kind() {
            T![&&] => JsLogicalOperator::LogicalAnd,
            T![||] => JsLogicalOperator::LogicalOr,
            T![??] => JsLogicalOperator::NullishCoalescing,
            _ => unreachable!(),
        };

        Ok(kind)
    }
}

impl JsArrayHole {
    pub fn hole_token(&self) -> Option<JsSyntaxToken> {
        None
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsUnaryOperator {
    /// `delete`
    Delete,
    /// `void`
    Void,
    /// `typeof`
    Typeof,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `~`
    BitwiseNot,
    /// `!`
    LogicalNot,
}

impl JsUnaryOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsUnaryExpression {
    pub fn operator(&self) -> SyntaxResult<JsUnaryOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![+] => JsUnaryOperator::Plus,
            T![-] => JsUnaryOperator::Minus,
            T![~] => JsUnaryOperator::BitwiseNot,
            T![!] => JsUnaryOperator::LogicalNot,
            T![typeof] => JsUnaryOperator::Typeof,
            T![void] => JsUnaryOperator::Void,
            T![delete] => JsUnaryOperator::Delete,
            _ => unreachable!(),
        })
    }

    pub fn is_void(&self) -> SyntaxResult<bool> {
        let operator = self.operator()?;

        Ok(matches!(operator, JsUnaryOperator::Void))
    }

    /// This function checks that `JsUnaryExpression` is a signed numeric literal:
    /// ```js
    ///     +123
    ///     -321
    /// ```
    pub fn is_signed_numeric_literal(&self) -> SyntaxResult<bool> {
        let argument = self.argument()?;

        let is_signed = matches!(
            self.operator()?,
            JsUnaryOperator::Plus | JsUnaryOperator::Minus
        );

        let is_numeric_literal = matches!(
            argument,
            AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsNumberLiteralExpression(_)
            )
        );

        Ok(is_signed && is_numeric_literal)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsPreUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl JsPreUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsPreUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<JsPreUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => JsPreUpdateOperator::Increment,
            T![--] => JsPreUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsPostUpdateOperator {
    /// `++`
    Increment,
    /// `--`
    Decrement,
}

impl JsPostUpdateOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        OperatorPrecedence::Unary
    }
}

impl JsPostUpdateExpression {
    pub fn operator(&self) -> SyntaxResult<JsPostUpdateOperator> {
        let operator = self.operator_token()?;

        Ok(match operator.kind() {
            T![++] => JsPostUpdateOperator::Increment,
            T![--] => JsPostUpdateOperator::Decrement,
            _ => unreachable!(),
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum JsAssignmentOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    TimesAssign,
    SlashAssign,
    RemainderAssign,
    ExponentAssign,
    LeftShiftAssign,
    RightShiftAssign,
    UnsignedRightShiftAssign,
    BitwiseAndAssign,
    BitwiseOrAssign,
    BitwiseXorAssign,
    LogicalAndAssign,
    LogicalOrAssign,
    NullishCoalescingAssign,
}

impl JsAssignmentExpression {
    pub fn operator(&self) -> SyntaxResult<JsAssignmentOperator> {
        let operator = match self.operator_token()?.kind() {
            T![=] => JsAssignmentOperator::Assign,
            T![+=] => JsAssignmentOperator::AddAssign,
            T![-=] => JsAssignmentOperator::SubtractAssign,
            T![*=] => JsAssignmentOperator::TimesAssign,
            T![/=] => JsAssignmentOperator::SlashAssign,
            T![%=] => JsAssignmentOperator::RemainderAssign,
            T![**=] => JsAssignmentOperator::ExponentAssign,
            T![>>=] => JsAssignmentOperator::LeftShiftAssign,
            T![<<=] => JsAssignmentOperator::RightShiftAssign,
            T![>>>=] => JsAssignmentOperator::UnsignedRightShiftAssign,
            T![&=] => JsAssignmentOperator::BitwiseAndAssign,
            T![|=] => JsAssignmentOperator::BitwiseOrAssign,
            T![^=] => JsAssignmentOperator::BitwiseXorAssign,
            T![&&=] => JsAssignmentOperator::LogicalAndAssign,
            T![||=] => JsAssignmentOperator::LogicalOrAssign,
            T![??=] => JsAssignmentOperator::NullishCoalescingAssign,
            _ => unreachable!(),
        };

        Ok(operator)
    }
}

impl JsArrayExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.elements().trailing_separator().is_some()
    }
}

impl JsObjectExpression {
    pub fn has_trailing_comma(&self) -> bool {
        self.members().trailing_separator().is_some()
    }

    pub fn is_empty(&self) -> bool {
        self.members().is_empty()
    }
}

impl JsNumberLiteralExpression {
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let number = make::js_number_literal_expression(make::js_number_literal("1.23")
    ///     .with_trailing_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(number.as_number().unwrap(), 1.23);
    /// ```
    pub fn as_number(&self) -> Option<f64> {
        parse_js_number(self.value_token().unwrap().text_trimmed())
    }
}

impl JsStringLiteralExpression {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    ///let string = make::js_string_literal_expression(make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "foo");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

impl JsTemplateExpression {
    /// Returns true if `self` is a template expression without a tag and without template elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{AnyJsExpression, AnyJsTemplateElement, JsSyntaxKind, JsSyntaxToken};
    /// use std::iter;
    ///
    /// let tick = make::token(JsSyntaxKind::BACKTICK);
    /// let empty_str = make::js_template_expression(
    ///     tick.clone(),
    ///     make::js_template_element_list([]),
    ///     tick.clone(),
    /// ).build();
    ///
    /// let chunk = AnyJsTemplateElement::JsTemplateChunkElement(
    ///     make::js_template_chunk_element(
    ///         JsSyntaxToken::new_detached(JsSyntaxKind::TEMPLATE_CHUNK, "text", [], [])
    ///     )
    /// );
    /// let constant_str = make::js_template_expression(
    ///     tick.clone(),
    ///     make::js_template_element_list([chunk.clone()]),
    ///     tick.clone(),
    /// ).build();
    ///
    /// let constant_str2 = make::js_template_expression(
    ///     tick.clone(),
    ///     make::js_template_element_list([chunk.clone(), chunk]),
    ///     tick.clone(),
    /// ).build();
    ///
    /// let template_elt = AnyJsTemplateElement::JsTemplateElement(
    ///     make::js_template_element(
    ///         JsSyntaxToken::new_detached(JsSyntaxKind::DOLLAR_CURLY, "${", [], []),
    ///         AnyJsExpression::JsIdentifierExpression(
    ///             make::js_identifier_expression(
    ///                 make::js_reference_identifier(make::ident("var")),
    ///             ),
    ///         ),
    ///         make::token(JsSyntaxKind::R_CURLY),
    ///     )
    /// );
    /// let template_str = make::js_template_expression(
    ///     tick.clone(),
    ///     make::js_template_element_list([template_elt]),
    ///     tick,
    /// ).build();
    ///
    /// assert!(empty_str.is_constant());
    /// assert!(constant_str.is_constant());
    /// assert!(constant_str2.is_constant());
    /// assert!(!template_str.is_constant());
    /// ```
    ///
    pub fn is_constant(&self) -> bool {
        self.tag().is_none()
            && self
                .elements()
                .into_iter()
                .all(|e| JsTemplateChunkElement::can_cast(e.syntax().kind()))
    }

    /// The string chunks of the template. aka:
    /// `foo ${bar} foo` breaks down into:
    /// `QUASIS ELEMENT{EXPR} QUASIS`
    pub fn quasis(&self) -> impl Iterator<Item = JsSyntaxToken> {
        self.syntax()
            .children_with_tokens()
            .filter_map(NodeOrToken::into_token)
            .filter(|t| t.kind() == JsSyntaxKind::TEMPLATE_CHUNK)
    }

    pub fn template_range(&self) -> Option<TextRange> {
        let start = self
            .syntax()
            .children_with_tokens()
            .filter_map(|x| x.into_token())
            .find(|tok| tok.kind() == JsSyntaxKind::BACKTICK)?;
        Some(TextRange::new(
            start.text_range().start(),
            self.syntax().text_range_with_trivia().end(),
        ))
    }

    pub fn is_test_each_pattern(&self) -> bool {
        self.is_test_each_pattern_callee() && self.is_test_each_pattern_elements()
    }

    /// This function checks if a call expressions has one of the following members:
    /// - `describe.each`
    /// - `describe.only.each`
    /// - `describe.skip.each`
    /// - `test.concurrent.each`
    /// - `test.concurrent.only.each`
    /// - `test.concurrent.skip.each`
    /// - `test.each`
    /// - `test.only.each`
    /// - `test.skip.each`
    /// - `test.failing.each`
    /// - `it.concurrent.each`
    /// - `it.concurrent.only.each`
    /// - `it.concurrent.skip.each`
    /// - `it.each`
    /// - `it.only.each`
    /// - `it.skip.each`
    /// - `it.failing.each`
    ///
    /// - `xdescribe.each`
    /// - `xdescribe.only.each`
    /// - `xdescribe.skip.each`
    /// - `xtest.concurrent.each`
    /// - `xtest.concurrent.only.each`
    /// - `xtest.concurrent.skip.each`
    /// - `xtest.each`
    /// - `xtest.only.each`
    /// - `xtest.skip.each`
    /// - `xtest.failing.each`
    /// - `xit.concurrent.each`
    /// - `xit.concurrent.only.each`
    /// - `xit.concurrent.skip.each`
    /// - `xit.each`
    /// - `xit.only.each`
    /// - `xit.skip.each`
    /// - `xit.failing.each`
    ///
    /// - `fdescribe.each`
    /// - `fdescribe.only.each`
    /// - `fdescribe.skip.each`
    /// - `ftest.concurrent.each`
    /// - `ftest.concurrent.only.each`
    /// - `ftest.concurrent.skip.each`
    /// - `ftest.each`
    /// - `ftest.only.each`
    /// - `ftest.skip.each`
    /// - `ftest.failing.each`
    /// - `fit.concurrent.each`
    /// - `fit.concurrent.only.each`
    /// - `fit.concurrent.skip.each`
    /// - `fit.each`
    /// - `fit.only.each`
    /// - `fit.skip.each`
    /// - `xit.failing.each`
    ///
    /// Based on this [article]
    ///
    /// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
    pub fn is_test_each_pattern_callee(&self) -> bool {
        if let Some(tag) = self.tag() {
            let mut members = CalleeNamesIterator::new(tag);

            let texts: [Option<TokenText>; 5] = [
                members.next(),
                members.next(),
                members.next(),
                members.next(),
                members.next(),
            ];

            let mut rev = texts.iter().rev().flatten();

            let first = rev.next().map(|t| t.text());
            let second = rev.next().map(|t| t.text());
            let third = rev.next().map(|t| t.text());
            let fourth = rev.next().map(|t| t.text());
            let fifth = rev.next().map(|t| t.text());

            match first {
                Some("describe" | "xdescribe" | "fdescribe") => match second {
                    Some("each") => third.is_none(),
                    Some("skip" | "only") => match third {
                        Some("each") => fourth.is_none(),
                        _ => false,
                    },
                    _ => false,
                },
                Some("test" | "xtest" | "ftest" | "it" | "xit" | "fit") => match second {
                    Some("each") => third.is_none(),
                    Some("skip" | "only" | "failing") => match third {
                        Some("each") => fourth.is_none(),
                        _ => false,
                    },
                    Some("concurrent") => match third {
                        Some("each") => fourth.is_none(),
                        Some("only" | "skip") => match fourth {
                            Some("each") => fifth.is_none(),
                            _ => false,
                        },
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            }
        } else {
            false
        }
    }

    fn is_test_each_pattern_elements(&self) -> bool {
        let mut iter = self.elements().into_iter();

        // the table must have a header as JsTemplateChunkElement
        // e.g. a | b | expected
        if !matches!(
            iter.next(),
            Some(AnyJsTemplateElement::JsTemplateChunkElement(_))
        ) {
            return false;
        }

        // Guarding against skipped token trivia on elements that we remove.
        // Because that would result in the skipped token trivia being emitted before the template.
        for element in self.elements() {
            if let AnyJsTemplateElement::JsTemplateChunkElement(element) = element {
                if let Some(leading_trivia) = element.syntax().first_leading_trivia() {
                    if leading_trivia.has_skipped() {
                        return false;
                    }
                }
            }
        }

        true
    }
}

impl JsRegexLiteralExpression {
    /// Decompose a regular expression into its pattern and flags.
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken};
    ///
    /// let token = JsSyntaxToken::new_detached(JsSyntaxKind::JS_REGEX_LITERAL, &format!("/a+/igu"), [], []);
    /// let regex = make::js_regex_literal_expression(token);
    /// let (pattern, flags) = regex.decompose().unwrap();
    /// assert_eq!(pattern.text(), "a+");
    /// assert_eq!(flags.text(), "igu");
    ///
    /// let token = JsSyntaxToken::new_detached(JsSyntaxKind::JS_REGEX_LITERAL, &format!("/a+/"), [], []);
    /// let regex = make::js_regex_literal_expression(token);
    /// let (pattern, flags) = regex.decompose().unwrap();
    /// assert_eq!(pattern.text(), "a+");
    /// assert_eq!(flags.text(), "");
    ///
    /// let token = JsSyntaxToken::new_detached(JsSyntaxKind::JS_REGEX_LITERAL, &format!("/a+"), [], []);
    /// let regex = make::js_regex_literal_expression(token);
    /// let (pattern, flags) = regex.decompose().unwrap();
    /// assert_eq!(pattern.text(), "a+");
    /// assert_eq!(flags.text(), "");
    /// ```
    pub fn decompose(&self) -> SyntaxResult<(TokenText, TokenText)> {
        let token = self.value_token()?;
        let text_trimmed = token.text_trimmed();
        let token_text = token.token_text_trimmed();
        let len = TextSize::from(text_trimmed.len() as u32);
        let Some(end_slash_pos) = text_trimmed[1..].rfind('/').map(|x| x + 1) else {
            return Ok((
                token_text
                    .clone()
                    .slice(TextRange::new(TextSize::from(1), len)),
                token_text.slice(TextRange::empty(len)),
            ));
        };
        let end_slash_pos = end_slash_pos as u32;
        let pattern = token_text.clone().slice(TextRange::new(
            TextSize::from(1),
            TextSize::from(end_slash_pos),
        ));
        let flags = token_text.slice(TextRange::new(TextSize::from(end_slash_pos + 1), len));
        Ok((pattern, flags))
    }
}

impl AnyJsExpression {
    /// Try to extract non `JsParenthesizedExpression` from `JsAnyExpression`
    pub fn omit_parentheses(self) -> AnyJsExpression {
        let first = self
            .as_js_parenthesized_expression()
            .and_then(|expression| expression.expression().ok());

        iter::successors(first, |expression| {
            let parenthesized = expression.as_js_parenthesized_expression()?;
            parenthesized.expression().ok()
        })
        .last()
        .unwrap_or(self)
    }

    pub fn precedence(&self) -> SyntaxResult<OperatorPrecedence> {
        let precedence = match self {
            AnyJsExpression::JsSequenceExpression(_) => OperatorPrecedence::Comma,
            AnyJsExpression::JsYieldExpression(_) => OperatorPrecedence::Yield,
            AnyJsExpression::JsConditionalExpression(_) => OperatorPrecedence::Conditional,
            AnyJsExpression::JsAssignmentExpression(_) => OperatorPrecedence::Assignment,
            AnyJsExpression::JsInExpression(_)
            | AnyJsExpression::JsInstanceofExpression(_)
            | AnyJsExpression::TsAsExpression(_)
            | AnyJsExpression::TsSatisfiesExpression(_) => OperatorPrecedence::Relational,
            AnyJsExpression::JsLogicalExpression(expression) => expression.operator()?.precedence(),
            AnyJsExpression::JsBinaryExpression(expression) => expression.operator()?.precedence(),
            AnyJsExpression::TsTypeAssertionExpression(_)
            | AnyJsExpression::TsNonNullAssertionExpression(_)
            | AnyJsExpression::JsUnaryExpression(_)
            | AnyJsExpression::JsAwaitExpression(_) => OperatorPrecedence::Unary,
            AnyJsExpression::JsPostUpdateExpression(_)
            | AnyJsExpression::JsPreUpdateExpression(_) => OperatorPrecedence::Update,
            AnyJsExpression::JsCallExpression(_)
            | AnyJsExpression::JsImportCallExpression(_)
            | AnyJsExpression::JsSuperExpression(_) => OperatorPrecedence::LeftHandSide,

            AnyJsExpression::JsNewExpression(expression) => {
                if expression.arguments().is_none() {
                    OperatorPrecedence::NewWithoutArguments
                } else {
                    OperatorPrecedence::LeftHandSide
                }
            }
            AnyJsExpression::JsComputedMemberExpression(_)
            | AnyJsExpression::JsStaticMemberExpression(_)
            | AnyJsExpression::JsImportMetaExpression(_)
            | AnyJsExpression::TsInstantiationExpression(_)
            | AnyJsExpression::JsNewTargetExpression(_) => OperatorPrecedence::Member,

            AnyJsExpression::JsThisExpression(_)
            | AnyJsExpression::AnyJsLiteralExpression(_)
            | AnyJsExpression::JsArrayExpression(_)
            | AnyJsExpression::JsArrowFunctionExpression(_)
            | AnyJsExpression::JsClassExpression(_)
            | AnyJsExpression::JsFunctionExpression(_)
            | AnyJsExpression::JsIdentifierExpression(_)
            | AnyJsExpression::JsObjectExpression(_)
            | AnyJsExpression::JsxTagExpression(_) => OperatorPrecedence::Primary,

            AnyJsExpression::JsTemplateExpression(template) => {
                if template.tag().is_some() {
                    OperatorPrecedence::Member
                } else {
                    OperatorPrecedence::Primary
                }
            }

            AnyJsExpression::JsBogusExpression(_) | AnyJsExpression::JsMetavariable(_) => {
                OperatorPrecedence::lowest()
            }
            AnyJsExpression::JsParenthesizedExpression(_) => OperatorPrecedence::highest(),
        };

        Ok(precedence)
    }

    /// Return identifier if the expression is an identifier expression.
    pub fn as_js_reference_identifier(&self) -> Option<JsReferenceIdentifier> {
        self.as_js_identifier_expression()?.name().ok()
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            AnyJsExpression::AnyJsLiteralExpression(literal) => literal.as_static_value(),
            AnyJsExpression::JsTemplateExpression(template) => {
                let element_list = template.elements();
                if element_list.len() == 0 {
                    let range = template
                        .l_tick_token()
                        .ok()?
                        .text_trimmed_range()
                        .add_start(1.into());
                    return Some(StaticValue::EmptyString(range));
                }
                if element_list.len() > 1 {
                    return None;
                }
                match element_list.first()? {
                    AnyJsTemplateElement::JsTemplateChunkElement(element) => {
                        Some(StaticValue::String(element.template_chunk_token().ok()?))
                    }
                    _ => None,
                }
            }
            AnyJsExpression::JsIdentifierExpression(identifier) => {
                let identifier_token = identifier.name().ok()?.value_token().ok()?;
                match identifier_token.text_trimmed() {
                    UNDEFINED => Some(StaticValue::Undefined(identifier_token)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    pub fn get_callee_object_name(&self) -> Option<JsSyntaxToken> {
        let identifier = self.get_callee_object_identifier()?;
        identifier.value_token().ok()
    }

    pub fn get_callee_object_identifier(&self) -> Option<JsReferenceIdentifier> {
        match self {
            AnyJsExpression::JsStaticMemberExpression(node) => {
                let member = node.object().ok()?;
                member.as_js_identifier_expression()?.name().ok()
            }
            AnyJsExpression::JsTemplateExpression(node) => {
                let tag = node.tag()?;
                let tag = tag.as_js_static_member_expression()?;
                let member = tag.object().ok()?;
                member.as_js_identifier_expression()?.name().ok()
            }
            AnyJsExpression::JsIdentifierExpression(node) => node.name().ok(),
            _ => None,
        }
    }

    pub fn get_callee_member_name(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsExpression::JsStaticMemberExpression(node) => {
                let member = node.member().ok()?;
                let member = member.as_js_name()?;
                member.value_token().ok()
            }
            AnyJsExpression::JsTemplateExpression(node) => {
                let tag = node.tag()?;
                let tag = tag.as_js_static_member_expression()?;
                let member = tag.member().ok()?;
                let member = member.as_js_name()?;
                member.value_token().ok()
            }
            AnyJsExpression::JsIdentifierExpression(node) => node.name().ok()?.value_token().ok(),
            _ => None,
        }
    }

    /// This function checks if a call expressions has one of the following members:
    /// - `it`
    /// - `it.only`
    /// - `it.skip`
    /// - `describe`
    /// - `describe.only`
    /// - `describe.skip`
    /// - `test`
    /// - `test.only`
    /// - `test.skip`
    /// - `test.step`
    /// - `test.describe`
    /// - `test.describe.only`
    /// - `test.describe.parallel`
    /// - `test.describe.parallel.only`
    /// - `test.describe.serial`
    /// - `test.describe.serial.only`
    /// - `skip`
    /// - `xit`
    /// - `xdescribe`
    /// - `xtest`
    /// - `fit`
    /// - `fdescribe`
    /// - `ftest`
    /// - `Deno.test`
    ///
    /// Based on this [article]
    ///
    /// [article]: https://craftinginterpreters.com/scanning-on-demand.html#tries-and-state-machines
    pub fn contains_a_test_pattern(&self) -> SyntaxResult<bool> {
        let mut members = CalleeNamesIterator::new(self.clone());

        let texts: [Option<TokenText>; 5] = [
            members.next(),
            members.next(),
            members.next(),
            members.next(),
            members.next(),
        ];

        let mut rev = texts.iter().rev().flatten();

        let first = rev.next().map(|t| t.text());
        let second = rev.next().map(|t| t.text());
        let third = rev.next().map(|t| t.text());
        let fourth = rev.next().map(|t| t.text());
        let fifth = rev.next().map(|t| t.text());

        Ok(match first {
            Some("it" | "describe" | "Deno") => match second {
                None => true,
                Some("only" | "skip" | "test") => third.is_none(),
                _ => false,
            },
            Some("test") => match second {
                None => true,
                Some("only" | "skip" | "step") => third.is_none(),
                Some("describe") => match third {
                    None => true,
                    Some("only") => fourth.is_none(),
                    Some("parallel" | "serial") => match fourth {
                        None => true,
                        Some("only") => fifth.is_none(),
                        _ => false,
                    },
                    _ => false,
                },
                _ => false,
            },
            Some("skip" | "xit" | "xdescribe" | "xtest" | "fit" | "fdescribe" | "ftest") => true,
            _ => false,
        })
    }

    /// Checks whether the current function call is:
    /// - `describe`
    pub fn contains_describe_call(&self) -> bool {
        let mut members = CalleeNamesIterator::new(self.clone());

        if let Some(member) = members.next() {
            return member.text() == "describe";
        }
        false
    }

    /// Checks whether the current function call is:
    /// - `it`: many libraries such as Node.js, Mocha, Jest, etc.
    /// - `test`: many libraries such as Node.js, bun, etc.
    /// - [`Deno.test`](https://docs.deno.com/runtime/manual/basics/testing/)
    /// - [`waitFor`](https://testing-library.com/docs/dom-testing-library/api-async/#waitfor)
    pub fn contains_it_call(&self) -> bool {
        let mut members = CalleeNamesIterator::new(self.clone());

        let texts: [Option<TokenText>; 2] = [members.next(), members.next()];

        let mut rev = texts.iter().rev().flatten();

        let first = rev.next().map(|t| t.text());
        let second = rev.next().map(|t| t.text());

        match first {
            Some("test" | "it" | "waitFor") => true,
            Some("Deno") => matches!(second, Some("test")),
            _ => false,
        }
    }

    /// Checks whether the current called is named:
    /// - `expect`
    /// - `assert`
    /// - `assertEquals`
    pub fn to_assertion_call(&self) -> Option<TokenText> {
        let mut members = CalleeNamesIterator::new(self.clone());

        let texts: [Option<TokenText>; 2] = [members.next(), members.next()];

        let mut rev = texts.iter().rev().flatten();

        let first = rev.next();
        let second = rev.next();

        match first {
            Some(first) => {
                if first.text() == "assert" {
                    if second.is_some() {
                        Some(first.clone())
                    } else {
                        None
                    }
                } else if matches!(first.text(), "expect" | "assertEquals") {
                    Some(first.clone())
                } else {
                    None
                }
            }
            None => None,
        }
    }

    /// Determining if an expression is literal
    /// - Any literal: 1, true, null, etc
    /// - Static template literals: `foo`
    /// - Negative numeric literal: -1
    /// - Parenthesized expression: (1)
    ///
    /// ## Example
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_js_syntax::{
    ///     AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, JsSyntaxToken, JsUnaryOperator, T
    /// };
    ///
    /// // Any literal: 1, true, null, etc
    /// let number_literal = AnyJsExpression::AnyJsLiteralExpression(
    ///     AnyJsLiteralExpression::from(make::js_number_literal_expression(make::js_number_literal("1")))
    /// );
    /// assert_eq!(number_literal.is_literal_expression(), true);
    ///
    /// // Static template literals: `foo`
    /// let template = AnyJsExpression::JsTemplateExpression(
    ///     make::js_template_expression(
    ///         make::token(T!['`']),
    ///         make::js_template_element_list(
    ///             vec![
    ///                 AnyJsTemplateElement::from(make::js_template_chunk_element(
    ///                     make::js_template_chunk("foo"),
    ///                 ))
    ///             ]
    ///         ),
    ///         make::token(T!['`']),
    ///     )
    ///     .build()
    /// );
    /// assert_eq!(template.is_literal_expression(), true);
    ///
    /// // Negative numeric literal: -1
    /// let negative_numeric_literal = AnyJsExpression::JsUnaryExpression(
    ///     make::js_unary_expression(make::token(T![-]), number_literal.clone())
    /// );
    /// assert_eq!(negative_numeric_literal.is_literal_expression(), true);
    ///
    /// // Parenthesized expression: (1)
    /// let parenthesized = AnyJsExpression::JsParenthesizedExpression(
    ///     make::js_parenthesized_expression(make::token(T!['(']), number_literal, make::token(T![')']))
    /// );
    /// assert_eq!(parenthesized.is_literal_expression(), true);
    /// ```
    pub fn is_literal_expression(&self) -> bool {
        match self {
            // Any literal: 1, true, null, etc
            AnyJsExpression::AnyJsLiteralExpression(_) => true,

            // Static template literals: `foo`
            AnyJsExpression::JsTemplateExpression(template_expression) => template_expression
                .elements()
                .into_iter()
                .all(|element| element.as_js_template_chunk_element().is_some()),

            // Negative numeric literal: -1
            AnyJsExpression::JsUnaryExpression(unary_expression) => {
                let is_minus_operator =
                    matches!(unary_expression.operator(), Ok(JsUnaryOperator::Minus));
                let is_number_expression = matches!(
                    unary_expression.argument(),
                    Ok(AnyJsExpression::AnyJsLiteralExpression(
                        AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                    ))
                );

                is_minus_operator && is_number_expression
            }

            // Parenthesized expression: (1)
            AnyJsExpression::JsParenthesizedExpression(parenthesized_expression) => {
                parenthesized_expression
                    .expression()
                    .is_ok_and(|expression| expression.is_literal_expression())
            }

            _ => false,
        }
    }
}

/// Iterator that returns the callee names in "top down order".
///
/// # Examples
///
/// ```javascript
/// it.only() -> [`only`, `it`]
/// ```
struct CalleeNamesIterator {
    next: Option<AnyJsExpression>,
}

impl CalleeNamesIterator {
    fn new(callee: AnyJsExpression) -> Self {
        Self { next: Some(callee) }
    }
}

impl Iterator for CalleeNamesIterator {
    type Item = TokenText;

    fn next(&mut self) -> Option<Self::Item> {
        use AnyJsExpression::*;

        let current = self.next.take()?;

        match current {
            JsIdentifierExpression(identifier) => identifier
                .name()
                .and_then(|reference| reference.value_token())
                .ok()
                .map(|value| value.token_text_trimmed()),
            JsStaticMemberExpression(member_expression) => match member_expression.member() {
                Ok(AnyJsName::JsName(name)) => {
                    self.next = member_expression.object().ok();
                    name.value_token()
                        .ok()
                        .map(|name| name.token_text_trimmed())
                }
                _ => None,
            },
            _ => None,
        }
    }
}

impl AnyJsLiteralExpression {
    pub fn value_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsLiteralExpression::JsBigintLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyJsLiteralExpression::JsBooleanLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(expression) => expression.value_token(),
            AnyJsLiteralExpression::JsNumberLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(expression) => {
                expression.value_token()
            }
            AnyJsLiteralExpression::JsStringLiteralExpression(expression) => {
                expression.value_token()
            }
        }
    }

    pub fn as_static_value(&self) -> Option<StaticValue> {
        match self {
            AnyJsLiteralExpression::JsBigintLiteralExpression(bigint) => {
                Some(StaticValue::BigInt(bigint.value_token().ok()?))
            }
            AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                Some(StaticValue::Boolean(boolean.value_token().ok()?))
            }
            AnyJsLiteralExpression::JsNullLiteralExpression(null) => {
                Some(StaticValue::Null(null.value_token().ok()?))
            }
            AnyJsLiteralExpression::JsNumberLiteralExpression(number) => {
                Some(StaticValue::Number(number.value_token().ok()?))
            }
            AnyJsLiteralExpression::JsRegexLiteralExpression(_) => None,
            AnyJsLiteralExpression::JsStringLiteralExpression(string) => {
                Some(StaticValue::String(string.value_token().ok()?))
            }
        }
    }
}

impl JsStaticMemberExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a.b -> false,
    /// a?.b -> true
    /// a?.[b][c][d].e -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.operator_token().map(|token| token.kind()) == Ok(T![?.])
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a.b -> false
    /// a?.b-> true
    /// a?.[b][c][d].e -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        AnyJsOptionalChainExpression::from(self.clone()).is_optional_chain()
    }
}

impl JsComputedMemberExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a[b] -> false,
    /// a?.[b] -> true
    /// a?.b.c.d[e] -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.optional_chain_token().is_some()
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a[b] -> false
    /// a?.[b]-> true
    /// a?.b.c.d[e] -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        AnyJsOptionalChainExpression::from(self.clone()).is_optional_chain()
    }
}

declare_node_union! {
    pub AnyJsComputedMember = JsComputedMemberExpression | JsComputedMemberAssignment
}

impl AnyJsComputedMember {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(expression) => expression.object(),
            AnyJsComputedMember::JsComputedMemberAssignment(assignment) => assignment.object(),
        }
    }

    pub fn l_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(expression) => {
                expression.l_brack_token()
            }
            AnyJsComputedMember::JsComputedMemberAssignment(assignment) => {
                assignment.l_brack_token()
            }
        }
    }

    pub fn optional_chain_token(&self) -> Option<JsSyntaxToken> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(expression) => {
                expression.optional_chain_token()
            }
            AnyJsComputedMember::JsComputedMemberAssignment(_) => None,
        }
    }

    pub fn member(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(expression) => expression.member(),
            AnyJsComputedMember::JsComputedMemberAssignment(assignment) => assignment.member(),
        }
    }

    pub fn r_brack_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            AnyJsComputedMember::JsComputedMemberExpression(expression) => {
                expression.r_brack_token()
            }
            AnyJsComputedMember::JsComputedMemberAssignment(assignment) => {
                assignment.r_brack_token()
            }
        }
    }
}

declare_node_union! {
    pub AnyJsMemberExpression = JsStaticMemberExpression | JsComputedMemberExpression
}

impl AnyJsMemberExpression {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsMemberExpression::JsStaticMemberExpression(expr) => expr.object(),
            AnyJsMemberExpression::JsComputedMemberExpression(expr) => expr.object(),
        }
    }

    pub fn is_optional_chain(&self) -> bool {
        match self {
            AnyJsMemberExpression::JsComputedMemberExpression(e) => e.is_optional_chain(),
            AnyJsMemberExpression::JsStaticMemberExpression(e) => e.is_optional_chain(),
        }
    }

    /// Returns the member name of `self` if `self` is a static member or a computed member with a literal string.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let math_id = make::js_reference_identifier(make::ident("Math"));
    /// let math_id = make::js_identifier_expression(math_id);
    /// let pow_ident_token = make::ident("pow");
    /// let pow_name = make::js_name(pow_ident_token);
    /// let static_member = make::js_static_member_expression(math_id.clone().into(), make::token(T![.]), pow_name.into());
    /// let static_member: AnyJsMemberExpression = static_member.into();
    /// let member_name = static_member.member_name().unwrap();
    /// assert_eq!(member_name.text(), "pow");
    ///
    /// let pow_str_token = make::js_string_literal("pow");
    /// let pow_str_literal = make::js_string_literal_expression(pow_str_token.clone());
    /// let pow_str_literal = AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(pow_str_literal));
    /// let computed_member = make::js_computed_member_expression(math_id.into(), make::token(T!['[']), pow_str_literal, make::token(T![']'])).build();
    /// let computed_member: AnyJsMemberExpression = computed_member.into();
    /// let member_name = computed_member.member_name().unwrap();
    /// assert_eq!(member_name.text(), "pow");
    /// ```
    pub fn member_name(&self) -> Option<StaticValue> {
        let value = match self {
            AnyJsMemberExpression::JsStaticMemberExpression(e) => {
                StaticValue::String(e.member().ok()?.as_js_name()?.value_token().ok()?)
            }
            AnyJsMemberExpression::JsComputedMemberExpression(e) => {
                let member = e.member().ok()?.omit_parentheses();
                let result = member.as_static_value()?;
                if !matches!(result, StaticValue::String(_) | StaticValue::EmptyString(_)) {
                    return None;
                }
                result
            }
        };
        Some(value)
    }
}

declare_node_union! {
    pub AnyJsOptionalChainExpression = JsStaticMemberExpression | JsComputedMemberExpression | JsCallExpression
}

impl AnyJsOptionalChainExpression {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsStaticMemberExpression(expr) => expr.object(),
            Self::JsComputedMemberExpression(expr) => expr.object(),
            Self::JsCallExpression(expr) => expr.callee(),
        }
    }

    pub fn optional_chain_token(&self) -> Option<JsSyntaxToken> {
        match self {
            Self::JsStaticMemberExpression(expr) => {
                expr.operator_token().ok().filter(|x| x.kind() == T![?.])
            }
            Self::JsComputedMemberExpression(expr) => expr.optional_chain_token(),
            Self::JsCallExpression(expr) => expr.optional_chain_token(),
        }
    }

    /// Returns `true` if this expression has an optional chain token.
    pub fn is_optional(&self) -> bool {
        self.optional_chain_token().is_some()
    }

    /// Returns `true` if this expression is a chain of at least one expression with an optional chain token.
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsOptionalChainExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let a_id = make::js_identifier_expression(make::js_reference_identifier(make::ident("a")));
    /// let b_name = make::js_name(make::ident("b"));
    /// let c_name = make::js_name(make::ident("c"));
    /// let d_name = make::js_name(make::ident("c"));
    ///
    /// // a.b?.c.d
    /// let ab = make::js_static_member_expression(a_id.into(), make::token(T![.]), b_name.into());
    /// let abc = make::js_static_member_expression(ab.clone().into(), make::token(T![?.]), c_name.into());
    /// let abcd = make::js_static_member_expression(abc.into(), make::token(T![.]), d_name.into());
    /// let abcd: AnyJsOptionalChainExpression = abcd.into();
    /// assert!(abcd.is_optional_chain());
    /// assert!(!ab.is_optional_chain());
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        if self.optional_chain_token().is_some() {
            return true;
        }
        let mut current_expression = self.object();
        while let Some(member_expr) = current_expression
            .ok()
            .and_then(|expr| Self::cast(expr.into_syntax()))
        {
            if member_expr.optional_chain_token().is_some() {
                return true;
            }
            current_expression = member_expr.object();
        }
        false
    }
}

impl From<AnyJsOptionalChainExpression> for AnyJsExpression {
    fn from(node: AnyJsOptionalChainExpression) -> AnyJsExpression {
        match node {
            AnyJsOptionalChainExpression::JsStaticMemberExpression(expression) => expression.into(),
            AnyJsOptionalChainExpression::JsComputedMemberExpression(expression) => {
                expression.into()
            }
            AnyJsOptionalChainExpression::JsCallExpression(expression) => expression.into(),
        }
    }
}

impl AnyJsObjectMemberName {
    /// Returns the member name of the current node
    /// if it is a literal member name or a computed member with a literal value.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsObjectMemberName, AnyJsExpression, AnyJsLiteralExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let name = make::js_literal_member_name(make::ident("a"));
    /// let name = AnyJsObjectMemberName::JsLiteralMemberName(name);
    /// assert_eq!(name.name().unwrap().text(), "a");
    ///
    /// let quoted_name = make::js_literal_member_name(make::js_string_literal("a"));
    /// let quoted_name = AnyJsObjectMemberName::JsLiteralMemberName(quoted_name);
    /// assert_eq!(quoted_name.name().unwrap().text(), "a");
    ///
    /// let number_name = make::js_literal_member_name(make::js_number_literal(42));
    /// let number_name = AnyJsObjectMemberName::JsLiteralMemberName(number_name);
    /// assert_eq!(number_name.name().unwrap().text(), "42");
    ///
    /// let string_literal = make::js_string_literal_expression(make::js_string_literal("a"));
    /// let string_literal = AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(string_literal));
    /// let computed = make::js_computed_member_name(make::token(T!['[']), string_literal, make::token(T![']']));
    /// let computed = AnyJsObjectMemberName::JsComputedMemberName(computed);
    /// assert_eq!(computed.name().unwrap().text(), "a");
    /// ```
    pub fn name(&self) -> Option<TokenText> {
        let token = match self {
            AnyJsObjectMemberName::JsComputedMemberName(expr) => {
                let expr = expr.expression().ok()?;
                match expr.omit_parentheses() {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => expr.value_token().ok()?,
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        if !expr.is_constant() {
                            return None;
                        }
                        let chunk = expr.elements().first()?;
                        let chunk = chunk.as_js_template_chunk_element()?;
                        chunk.template_chunk_token().ok()?
                    }
                    _ => return None,
                }
            }
            AnyJsObjectMemberName::JsLiteralMemberName(expr) => expr.value().ok()?,
            AnyJsObjectMemberName::JsMetavariable(_) => return None,
        };
        Some(inner_string_text(&token))
    }
}

impl AnyTsEnumMemberName {
    /// Returns the member name of the current node
    /// if it is a literal member name or a computed member with a literal value.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyTsEnumMemberName, AnyJsExpression, AnyJsLiteralExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let name = make::ts_literal_enum_member_name(make::ident("a"));
    /// let name = AnyTsEnumMemberName::TsLiteralEnumMemberName(name);
    /// assert_eq!(name.name().unwrap().text(), "a");
    ///
    /// let quoted_name = make::ts_literal_enum_member_name(make::js_string_literal("a"));
    /// let quoted_name = AnyTsEnumMemberName::TsLiteralEnumMemberName(quoted_name);
    /// assert_eq!(quoted_name.name().unwrap().text(), "a");
    ///
    /// let number_name = make::ts_literal_enum_member_name(make::js_number_literal(42));
    /// let number_name = AnyTsEnumMemberName::TsLiteralEnumMemberName(number_name);
    /// assert_eq!(number_name.name().unwrap().text(), "42");
    ///
    /// let string_literal = make::js_string_literal_expression(make::js_string_literal("a"));
    /// let string_literal = AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(string_literal));
    /// let computed = make::js_computed_member_name(make::token(T!['[']), string_literal, make::token(T![']']));
    /// let computed = AnyTsEnumMemberName::JsComputedMemberName(computed);
    /// assert_eq!(computed.name().unwrap().text(), "a");
    /// ```
    pub fn name(&self) -> Option<TokenText> {
        let token = match self {
            AnyTsEnumMemberName::JsComputedMemberName(expr) => {
                let expr = expr.expression().ok()?;
                match expr.omit_parentheses() {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => expr.value_token().ok()?,
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        if !expr.is_constant() {
                            return None;
                        }
                        let chunk = expr.elements().first()?;
                        let chunk = chunk.as_js_template_chunk_element()?;
                        chunk.template_chunk_token().ok()?
                    }
                    _ => return None,
                }
            }
            AnyTsEnumMemberName::TsLiteralEnumMemberName(expr) => expr.value().ok()?,
        };
        Some(inner_string_text(&token))
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ClassMemberName {
    /// Name that is preceded in the source code by the private marker `#`.
    /// For example `class { #f(){} }`
    Private(TokenText),
    /// Name that is NOT preceded in the source code by the private marker `#`.
    /// For example `class { f(){} }`
    Public(TokenText),
}
impl ClassMemberName {
    pub fn text(&self) -> &str {
        match self {
            Self::Private(name) => name.text(),
            Self::Public(name) => name.text(),
        }
    }
}
impl From<ClassMemberName> for TokenText {
    fn from(value: ClassMemberName) -> Self {
        match value {
            ClassMemberName::Private(name) => name,
            ClassMemberName::Public(name) => name,
        }
    }
}

impl AnyJsClassMemberName {
    /// Returns the member name of the current node
    /// if it is a literal, a computed, or a private class member with a literal value.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{AnyJsClassMemberName, AnyJsExpression, AnyJsLiteralExpression, T};
    /// use biome_js_factory::make;
    ///
    /// let name = make::js_literal_member_name(make::ident("a"));
    /// let name = AnyJsClassMemberName::JsLiteralMemberName(name);
    /// assert_eq!(name.name().unwrap().text(), "a");
    ///
    /// let quoted_name = make::js_literal_member_name(make::js_string_literal("a"));
    /// let quoted_name = AnyJsClassMemberName::JsLiteralMemberName(quoted_name);
    /// assert_eq!(quoted_name.name().unwrap().text(), "a");
    ///
    /// let number_name = make::js_literal_member_name(make::js_number_literal(42));
    /// let number_name = AnyJsClassMemberName::JsLiteralMemberName(number_name);
    /// assert_eq!(number_name.name().unwrap().text(), "42");
    ///
    /// let string_literal = make::js_string_literal_expression(make::js_string_literal("a"));
    /// let string_literal = AnyJsExpression::AnyJsLiteralExpression(AnyJsLiteralExpression::from(string_literal));
    /// let computed = make::js_computed_member_name(make::token(T!['[']), string_literal, make::token(T![']']));
    /// let computed = AnyJsClassMemberName::JsComputedMemberName(computed);
    /// assert_eq!(computed.name().unwrap().text(), "a");
    /// ```
    pub fn name(&self) -> Option<ClassMemberName> {
        let token = match self {
            AnyJsClassMemberName::JsComputedMemberName(expr) => {
                let expr = expr.expression().ok()?;
                match expr.omit_parentheses() {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => expr.value_token().ok()?,
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        if !expr.is_constant() {
                            return None;
                        }
                        let chunk = expr.elements().first()?;
                        let chunk = chunk.as_js_template_chunk_element()?;
                        chunk.template_chunk_token().ok()?
                    }
                    _ => return None,
                }
            }
            AnyJsClassMemberName::JsLiteralMemberName(expr) => expr.value().ok()?,
            AnyJsClassMemberName::JsPrivateClassMemberName(expr) => {
                return Some(ClassMemberName::Private(inner_string_text(
                    &expr.id_token().ok()?,
                )));
            }
            AnyJsClassMemberName::JsMetavariable(_) => return None,
        };
        Some(ClassMemberName::Public(inner_string_text(&token)))
    }
}

/// Check if `expr` refers to a name that is directly referenced or indirectly via `globalThis` or `window`.
/// Returns the reference and the name.
///
/// ### Examples
///
/// ```
/// use biome_js_syntax::{AnyJsExpression, AnyJsLiteralExpression, AnyJsMemberExpression, global_identifier, T};
/// use biome_js_factory::make;
///
/// let math_reference = make::js_reference_identifier(make::ident("Math"));
/// let math_id = make::js_identifier_expression(math_reference.clone());
/// let math_id = AnyJsExpression::from(math_id);
/// let (reference, name) = global_identifier(&math_id).unwrap();
/// assert_eq!(name.text(), "Math");
/// assert_eq!(reference, math_reference);
///
/// let global_this_reference = make::js_reference_identifier(make::ident("globalThis"));
/// let global_this_id = make::js_identifier_expression(global_this_reference.clone());
/// let global_this_id = AnyJsExpression::from(global_this_id);
///
/// let math_ident_token = make::ident("Math");
/// let math_name = make::js_name(math_ident_token);
/// let static_member = make::js_static_member_expression(global_this_id.clone().into(), make::token(T![.]), math_name.into());
/// let static_member = AnyJsExpression::from(static_member);
/// let (reference, name) = global_identifier(&static_member).unwrap();
/// assert_eq!(name.text(), "Math");
/// assert_eq!(reference, global_this_reference);
/// ```
pub fn global_identifier(expr: &AnyJsExpression) -> Option<(JsReferenceIdentifier, StaticValue)> {
    if let Some(reference) = expr.as_js_reference_identifier() {
        let name = StaticValue::String(reference.value_token().ok()?);
        return Some((reference, name));
    }
    let member_expr = AnyJsMemberExpression::cast_ref(expr.syntax())?;
    let name = member_expr.member_name()?;
    let mut expr = member_expr.object().ok()?.omit_parentheses();
    while let Some(member_expr) = AnyJsMemberExpression::cast_ref(expr.syntax()) {
        if !matches!(member_expr.member_name()?.text(), GLOBAL_THIS | WINDOW) {
            return None;
        }
        expr = member_expr.object().ok()?.omit_parentheses();
    }
    if let Some(reference) = expr.as_js_reference_identifier() {
        if matches!(reference.name().ok()?.text(), GLOBAL_THIS | WINDOW) {
            return Some((reference, name));
        }
    }
    None
}

impl From<AnyJsMemberExpression> for AnyJsExpression {
    fn from(expression: AnyJsMemberExpression) -> Self {
        match expression {
            AnyJsMemberExpression::JsComputedMemberExpression(expr) => expr.into(),
            AnyJsMemberExpression::JsStaticMemberExpression(expr) => expr.into(),
        }
    }
}

impl JsCallExpression {
    /// Returns `true` if this is an optional member access
    ///
    /// ```javascript
    /// a() -> false,
    /// a?.() -> true
    /// a?.b() -> false
    /// ```
    pub fn is_optional(&self) -> bool {
        self.optional_chain_token().is_some()
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a() -> false
    /// a?.()-> true
    /// a?.b.c.d() -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        AnyJsOptionalChainExpression::from(self.clone()).is_optional_chain()
    }

    pub fn has_callee(&self, name: &str) -> bool {
        self.callee().is_ok_and(|it| {
            it.as_js_reference_identifier()
                .is_some_and(|it| it.has_name(name))
        })
    }

    /// This is a specialized function that checks if the current [call expression]
    /// resembles a call expression usually used by a testing frameworks.
    ///
    /// If the [call expression] matches the criteria, a different formatting is applied.
    ///
    /// To evaluate the eligibility of a  [call expression] to be a test framework like,
    /// we need to check its [callee] and its [arguments].
    ///
    /// 1. The [callee] must contain a name or a chain of names that belongs to the
    ///     test frameworks, for example: `test()`, `test.only()`, etc.
    /// 2. The [arguments] should be at the least 2
    /// 3. The first argument has to be a string literal
    /// 4. The third argument, if present, has to be a number literal
    /// 5. The second argument has to be an [arrow function expression] or [function expression]
    /// 6. Both function must have zero or one parameters
    ///
    /// [call expression]: crate::JsCallExpression
    /// [callee]: crate::AnyJsExpression
    /// [arguments]: crate::JsCallArgumentList
    /// [arrow function expression]: crate::JsArrowFunctionExpression
    /// [function expression]: crate::JsCallArgumentList
    pub fn is_test_call_expression(&self) -> SyntaxResult<bool> {
        use AnyJsExpression::*;

        let callee = self.callee()?;
        let arguments = self.arguments()?;

        let mut args = arguments.args().iter();

        match (args.next(), args.next(), args.next()) {
            (Some(Ok(argument)), None, None) if arguments.args().len() == 1 => {
                if is_angular_test_wrapper(&self.clone().into())
                    && self
                        .parent::<JsCallArgumentList>()
                        .and_then(|arguments_list| arguments_list.parent::<JsCallArguments>())
                        .and_then(|arguments| arguments.parent::<self::JsCallExpression>())
                        .is_some_and(|parent| parent.is_test_call_expression().unwrap_or(false))
                {
                    return Ok(matches!(
                        argument,
                        AnyJsCallArgument::AnyJsExpression(
                            JsArrowFunctionExpression(_) | JsFunctionExpression(_)
                        )
                    ));
                }

                if is_unit_test_set_up_callee(&callee) {
                    return Ok(argument
                        .as_any_js_expression()
                        .is_some_and(is_angular_test_wrapper));
                }

                Ok(false)
            }

            // it("description", ..)
            // it(Test.name, ..)
            (Some(Ok(AnyJsCallArgument::AnyJsExpression(_))), Some(Ok(second)), third)
                if arguments.args().len() <= 3 && callee.contains_a_test_pattern()? =>
            {
                // it('name', callback, duration)
                if !matches!(
                    third,
                    None | Some(Ok(AnyJsCallArgument::AnyJsExpression(
                        AnyJsLiteralExpression(
                            self::AnyJsLiteralExpression::JsNumberLiteralExpression(_)
                        )
                    )))
                ) {
                    return Ok(false);
                }

                if second
                    .as_any_js_expression()
                    .is_some_and(is_angular_test_wrapper)
                {
                    return Ok(true);
                }

                let (parameters, has_block_body) = match second {
                    AnyJsCallArgument::AnyJsExpression(JsFunctionExpression(function)) => (
                        function
                            .parameters()
                            .map(AnyJsArrowFunctionParameters::from),
                        true,
                    ),
                    AnyJsCallArgument::AnyJsExpression(JsArrowFunctionExpression(arrow)) => (
                        arrow.parameters(),
                        arrow
                            .body()
                            .is_ok_and(|body| matches!(body, AnyJsFunctionBody::JsFunctionBody(_))),
                    ),
                    _ => return Ok(false),
                };

                Ok(arguments.args().len() == 2 || (parameters?.len() <= 1 && has_block_body))
            }
            _ => Ok(false),
        }
    }
}

/// Note: `inject` is used in AngularJS 1.x, `async` and `fakeAsync` in
/// Angular 2+, although `async` is deprecated and replaced by `waitForAsync`
/// since Angular 12.
///
/// example: https://docs.angularjs.org/guide/unit-testing#using-beforeall-
///
/// @param {CallExpression} node
/// @returns {boolean}
///
fn is_angular_test_wrapper(expression: &AnyJsExpression) -> bool {
    use AnyJsExpression::*;
    match expression {
        JsCallExpression(call_expression) => match call_expression.callee() {
            Ok(JsIdentifierExpression(identifier)) => identifier
                .name()
                .and_then(|name| name.value_token())
                .is_ok_and(|name| {
                    matches!(
                        name.text_trimmed(),
                        "async" | "inject" | "fakeAsync" | "waitForAsync"
                    )
                }),
            _ => false,
        },
        _ => false,
    }
}

/// Tests if the callee is a `beforeEach`, `beforeAll`, `afterEach` or `afterAll` identifier
/// that is commonly used in test frameworks.
fn is_unit_test_set_up_callee(callee: &AnyJsExpression) -> bool {
    match callee {
        AnyJsExpression::JsIdentifierExpression(identifier) => identifier
            .name()
            .and_then(|name| name.value_token())
            .is_ok_and(|name| {
                matches!(
                    name.text_trimmed(),
                    "beforeEach" | "beforeAll" | "afterEach" | "afterAll"
                )
            }),
        _ => false,
    }
}

impl JsNewExpression {
    pub fn has_callee(&self, name: &str) -> bool {
        self.callee().is_ok_and(|it| {
            it.as_js_reference_identifier()
                .is_some_and(|it| it.has_name(name))
        })
    }
}

impl TsStringLiteralType {
    /// Get the inner text of a string not including the quotes
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let string = make::ts_string_literal_type(make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "foo");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.literal_token()?))
    }
}

/// Check if the SyntaxNode is a Negate Unary Expression
/// ## Example
/// ```js
/// !x
/// ```
pub fn is_negation(node: &JsSyntaxNode) -> Option<JsUnaryExpression> {
    let unary_expr = JsUnaryExpression::cast_ref(node)?;
    if unary_expr.operator().ok()? == JsUnaryOperator::LogicalNot {
        Some(unary_expr)
    } else {
        None
    }
}

/// Check if this node is in the position of `test` slot of parent syntax node.
/// ## Example
/// ```js
/// if (!!x) {
///     ^^^ this is a boolean context
/// }
/// ```
pub fn is_in_boolean_context(node: &JsSyntaxNode) -> Option<bool> {
    let parent = node.parent()?;
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => {
            Some(parent.cast::<JsIfStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
            Some(parent.cast::<JsDoWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_WHILE_STATEMENT => {
            Some(parent.cast::<JsWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_FOR_STATEMENT => {
            Some(parent.cast::<JsForStatement>()?.test()?.syntax() == node)
        }
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => Some(
            parent
                .cast::<JsConditionalExpression>()?
                .test()
                .ok()?
                .syntax()
                == node,
        ),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use biome_js_factory::syntax::{JsCallExpression, JsTemplateExpression};
    use biome_js_parser::parse_module;
    use biome_js_parser::JsParserOptions;
    use biome_rowan::AstNodeList;

    fn extract_call_expression(src: &str) -> JsCallExpression {
        let result = parse_module(src, JsParserOptions::default());
        let module = result.tree().items().first().unwrap();

        module
            .as_any_js_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_call_expression()
            .unwrap()
            .clone()
    }

    fn extract_template(src: &str) -> JsTemplateExpression {
        let result = parse_module(src, JsParserOptions::default());
        let module = result.tree().items().first().unwrap();

        module
            .as_any_js_statement()
            .unwrap()
            .as_js_expression_statement()
            .unwrap()
            .expression()
            .unwrap()
            .as_js_template_expression()
            .unwrap()
            .clone()
    }

    #[test]
    fn matches_simple_call() {
        let call_expression = extract_call_expression("test();");
        assert_eq!(
            call_expression.callee().unwrap().contains_a_test_pattern(),
            Ok(true)
        );

        let call_expression = extract_call_expression("it();");
        assert_eq!(
            call_expression.callee().unwrap().contains_a_test_pattern(),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression() {
        let call_expression = extract_call_expression("test.only();");
        assert_eq!(
            call_expression.callee().unwrap().contains_a_test_pattern(),
            Ok(true)
        );
    }

    #[test]
    fn matches_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only();");
        assert_eq!(
            call_expression.callee().unwrap().contains_a_test_pattern(),
            Ok(true)
        );
    }

    #[test]
    fn doesnt_static_member_expression_deep() {
        let call_expression = extract_call_expression("test.describe.parallel.only.AHAHA();");
        assert_eq!(
            call_expression.callee().unwrap().contains_a_test_pattern(),
            Ok(false)
        );
    }

    #[test]
    fn matches_test_call_expression() {
        let call_expression = extract_call_expression("test.only(name, () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));

        let call_expression = extract_call_expression("test.only(Test.name, () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));

        let call_expression =
            extract_call_expression("test.only(name = name || 'test', () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));

        let call_expression = extract_call_expression("describe.only(name, () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));

        let call_expression = extract_call_expression("describe.only(Test.name, () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));

        let call_expression =
            extract_call_expression("describe.only(name = name || 'test', () => {});");
        assert_eq!(call_expression.is_test_call_expression(), Ok(true));
    }

    #[test]
    fn matches_simple_each() {
        let template = extract_template("describe.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("test.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xdescribe.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fdescribe.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_skip_each() {
        let template = extract_template("describe.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("test.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xdescribe.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fdescribe.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.skip.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_only_each() {
        let template = extract_template("describe.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("test.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xdescribe.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fdescribe.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.only.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_failing_each() {
        let template = extract_template("test.failing.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.failing.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.failing.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.failing.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.failing.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.failing.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_concurrent_each() {
        let template = extract_template("test.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.concurrent.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_concurrent_only_each() {
        let template = extract_template("test.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.concurrent.only.each``");
        assert!(template.is_test_each_pattern_callee());
    }

    #[test]
    fn matches_concurrent_skip_each() {
        let template = extract_template("test.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("it.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xtest.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("xit.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("ftest.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());

        let template = extract_template("fit.concurrent.skip.each``");
        assert!(template.is_test_each_pattern_callee());
    }
}

declare_node_union! {
    /// Subset of expressions supported by this rule.
    ///
    /// ## Examples
    ///
    /// - `JsStringLiteralExpression` &mdash; `"5"`
    /// - `JsNumberLiteralExpression` &mdash; `5`
    /// - `JsUnaryExpression` &mdash; `+5` | `-5`
    ///
    pub AnyNumberLikeExpression = JsStringLiteralExpression | JsNumberLiteralExpression | JsUnaryExpression
}

impl AnyNumberLikeExpression {
    /// Returns the value of a number-like expression; it returns the expression
    /// text for literal expressions. However, for unary expressions, it only
    /// returns the value for signed numeric expressions.
    pub fn value(&self) -> Option<String> {
        match self {
            AnyNumberLikeExpression::JsStringLiteralExpression(string_literal) => {
                return Some(string_literal.inner_string_text().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsNumberLiteralExpression(number_literal) => {
                return Some(number_literal.value_token().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsUnaryExpression(unary_expression) => {
                if unary_expression.is_signed_numeric_literal().ok()? {
                    return Some(unary_expression.to_trimmed_string());
                }
            }
        }
        None
    }
}
