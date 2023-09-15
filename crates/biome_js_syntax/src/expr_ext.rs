//! Extensions for things which are not easily generated in ast expr nodes
use crate::numbers::parse_js_number;
use crate::static_value::StaticValue;
use crate::{
    inner_string_text, AnyJsCallArgument, AnyJsClassMemberName, AnyJsExpression,
    AnyJsLiteralExpression, AnyJsObjectMemberName, AnyJsTemplateElement, JsArrayExpression,
    JsArrayHole, JsAssignmentExpression, JsBinaryExpression, JsCallExpression,
    JsComputedMemberAssignment, JsComputedMemberExpression, JsLiteralMemberName,
    JsLogicalExpression, JsNewExpression, JsNumberLiteralExpression, JsObjectExpression,
    JsPostUpdateExpression, JsReferenceIdentifier, JsRegexLiteralExpression,
    JsStaticMemberExpression, JsStringLiteralExpression, JsSyntaxKind, JsSyntaxToken,
    JsTemplateChunkElement, JsTemplateExpression, JsUnaryExpression, OperatorPrecedence, T,
};
use crate::{JsPreUpdateExpression, JsSyntaxKind::*};
use biome_rowan::{
    declare_node_union, AstNode, AstNodeList, AstSeparatedList, NodeOrToken, SyntaxResult,
    TextRange, TokenText,
};
use core::iter;

const GLOBAL_THIS: &str = "globalThis";
const UNDEFINED: &str = "undefined";
const WINDOW: &str = "window";

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
            .map(|token| token.text_trimmed() == name)
            .unwrap_or_default()
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
                .map_or(false, |x| x.is_null_or_undefined()))
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
    pub fn as_number(&self) -> Option<f64> {
        parse_js_number(self.value_token().unwrap().text())
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
            .filter(|t| t.kind() == TEMPLATE_CHUNK)
    }

    pub fn template_range(&self) -> Option<TextRange> {
        let start = self
            .syntax()
            .children_with_tokens()
            .filter_map(|x| x.into_token())
            .find(|tok| tok.kind() == BACKTICK)?;
        Some(TextRange::new(
            start.text_range().start(),
            self.syntax().text_range().end(),
        ))
    }
}

impl JsRegexLiteralExpression {
    pub fn pattern(&self) -> SyntaxResult<String> {
        let token = self.value_token()?;
        let text_trimmed = token.text_trimmed();

        // SAFETY: a valid regex literal must have a end slash
        let end_slash_pos = text_trimmed
            .rfind('/')
            .expect("regex literal must have an end slash");

        Ok(String::from(&text_trimmed[1..end_slash_pos]))
    }

    pub fn flags(&self) -> SyntaxResult<String> {
        let token = self.value_token()?;
        let text_trimmed = token.text_trimmed();

        // SAFETY: a valid regex literal must have a end slash
        let end_slash_pos = text_trimmed
            .rfind('/')
            .expect("regex literal must have an end slash");

        Ok(String::from(&text_trimmed[end_slash_pos..]))
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

            AnyJsExpression::JsBogusExpression(_) => OperatorPrecedence::lowest(),
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
        self.operator_token()
            .map_or(false, |token| token.kind() == JsSyntaxKind::QUESTIONDOT)
    }

    /// Returns true if this member has an optional token or any member expression on the left side.
    ///
    /// ```javascript
    /// a.b -> false
    /// a?.b-> true
    /// a?.[b][c][d].e -> true
    /// ```
    pub fn is_optional_chain(&self) -> bool {
        is_optional_chain(self.clone().into())
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
        is_optional_chain(self.clone().into())
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
        };
        Some(inner_string_text(&token))
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
    pub fn name(&self) -> Option<TokenText> {
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
            AnyJsClassMemberName::JsPrivateClassMemberName(expr) => expr.id_token().ok()?,
        };
        Some(inner_string_text(&token))
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
        is_optional_chain(self.clone().into())
    }

    /// Get [AnyJsCallArgument] by it index inside the [JsCallExpression] argument list.
    ///
    /// Each index inside "indices" should be unique.
    /// "indices" must be sorted.
    ///
    /// Supports maximum of 16 indices to avoid stack overflow. Each argument will consume:
    ///
    /// - 8 bytes for the `Option<AnyJsCallArgument>` result;
    /// - 8 bytes for the [usize] argument.
    pub fn get_arguments_by_index<const N: usize>(
        &self,
        indices: [usize; N],
    ) -> [Option<AnyJsCallArgument>; N] {
        debug_assert!(N <= 16);
        // assert there are no duplicates and they are in-order
        debug_assert!(indices.windows(2).all(|vs| vs[0] < vs[1]));

        const INIT: Option<AnyJsCallArgument> = None;
        let mut results = [INIT; N];
        let mut next = 0;
        for (i, arg) in self
            .arguments()
            .ok()
            .map(|x| x.args().into_iter())
            .into_iter()
            .flatten()
            .enumerate()
        {
            if i == indices[next] {
                results[next] = arg.ok();
                next += 1;
                if next == N {
                    break;
                }
            }
        }
        results
    }

    pub fn has_callee(&self, name: &str) -> bool {
        self.callee().map_or(false, |it| {
            it.as_js_reference_identifier()
                .map_or(false, |it| it.has_name(name))
        })
    }
}

impl JsNewExpression {
    pub fn has_callee(&self, name: &str) -> bool {
        self.callee().map_or(false, |it| {
            it.as_js_reference_identifier()
                .map_or(false, |it| it.has_name(name))
        })
    }
}

fn is_optional_chain(start: AnyJsExpression) -> bool {
    let mut current = Some(start);

    while let Some(node) = current {
        current = match node {
            AnyJsExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.expression().ok()
            }

            AnyJsExpression::JsCallExpression(call) => {
                if call.is_optional() {
                    return true;
                }
                call.callee().ok()
            }

            AnyJsExpression::JsStaticMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object().ok()
            }

            AnyJsExpression::JsComputedMemberExpression(member) => {
                if member.is_optional() {
                    return true;
                }
                member.object().ok()
            }
            _ => return false,
        }
    }

    false
}
