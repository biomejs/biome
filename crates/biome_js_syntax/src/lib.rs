//! A crate for generated Syntax node definitions and utility macros.
//! Both rome_js_lexer and biome_js_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#[macro_use]
mod generated;
pub mod assign_ext;
pub mod binary_like_expression;
pub mod binding_ext;
pub mod declaration_ext;
pub mod directive_ext;
pub mod export_ext;
pub mod expr_ext;
pub mod expression_left_side;
pub mod file_source;
pub mod function_ext;
pub mod identifier_ext;
pub mod import_ext;
pub mod jsx_ext;
pub mod misc_ext;
pub mod modifier_ext;
pub mod numbers;
pub mod parameter_ext;
pub mod parentheses;
pub mod static_value;
pub mod stmt_ext;
mod syntax_node;
pub mod type_ext;
mod union_ext;

pub use self::generated::*;
pub use biome_rowan::{
    SyntaxNodeText, TextLen, TextRange, TextSize, TokenAtOffset, TokenText, TriviaPieceKind,
    WalkEvent,
};
pub use expr_ext::*;
pub use file_source::*;
pub use function_ext::*;
pub use identifier_ext::*;
pub use import_ext::*;
pub use modifier_ext::*;
pub use stmt_ext::*;
pub use syntax_node::*;

use crate::JsSyntaxKind::*;
use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind, SyntaxResult};

impl From<u16> for JsSyntaxKind {
    fn from(d: u16) -> JsSyntaxKind {
        assert!(d <= (JsSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, JsSyntaxKind>(d) }
    }
}

impl From<JsSyntaxKind> for u16 {
    fn from(k: JsSyntaxKind) -> u16 {
        k as u16
    }
}

impl JsSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (JsSyntaxKind::USING_KW as u16)
            && (self as u16) >= (JsSyntaxKind::BREAK_KW as u16)
    }

    /// Returns `true` for any kind representing a Grit metavariable.
    #[inline]
    pub fn is_metavariable(&self) -> bool {
        matches!(
            self,
            JsSyntaxKind::GRIT_METAVARIABLE | JsSyntaxKind::JS_METAVARIABLE
        )
    }

    /// Returns `true` for contextual keywords (excluding strict mode contextual keywords)
    #[inline]
    pub const fn is_contextual_keyword(self) -> bool {
        (self as u16) >= (JsSyntaxKind::ABSTRACT_KW as u16)
            && (self as u16) <= (JsSyntaxKind::USING_KW as u16)
    }

    /// Returns true for all non-contextual keywords (includes future reserved keywords)
    #[inline]
    pub const fn is_non_contextual_keyword(self) -> bool {
        self.is_keyword() && !self.is_contextual_keyword()
    }

    #[inline]
    pub const fn is_future_reserved_keyword(self) -> bool {
        (self as u16) >= (JsSyntaxKind::IMPLEMENTS_KW as u16)
            && (self as u16) <= (JsSyntaxKind::YIELD_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for JsSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            JS_BOGUS
                | JS_BOGUS_STATEMENT
                | JS_BOGUS_PARAMETER
                | JS_BOGUS_BINDING
                | JS_BOGUS_MEMBER
                | JS_BOGUS_EXPRESSION
                | JS_BOGUS_IMPORT_ASSERTION_ENTRY
                | JS_BOGUS_NAMED_IMPORT_SPECIFIER
                | JS_BOGUS_ASSIGNMENT
                | TS_BOGUS_TYPE
        )
    }

    fn to_bogus(&self) -> JsSyntaxKind {
        match self {
            kind if AnyJsModuleItem::can_cast(*kind) => JS_BOGUS_STATEMENT,
            kind if AnyJsExpression::can_cast(*kind) => JS_BOGUS_EXPRESSION,
            kind if AnyJsBinding::can_cast(*kind) => JS_BOGUS_BINDING,
            kind if AnyJsClassMember::can_cast(*kind) || AnyJsObjectMember::can_cast(*kind) => {
                JS_BOGUS_MEMBER
            }
            kind if AnyJsAssignment::can_cast(*kind) => JS_BOGUS_ASSIGNMENT,
            kind if AnyJsNamedImportSpecifier::can_cast(*kind) => JS_BOGUS_NAMED_IMPORT_SPECIFIER,
            kind if AnyJsImportAssertionEntry::can_cast(*kind) => JS_BOGUS_IMPORT_ASSERTION_ENTRY,
            kind if AnyJsParameter::can_cast(*kind) => JS_BOGUS_PARAMETER,
            kind if AnyTsType::can_cast(*kind) => TS_BOGUS_TYPE,

            _ => JS_BOGUS,
        }
    }

    #[inline]
    fn to_raw(&self) -> RawSyntaxKind {
        RawSyntaxKind(*self as u16)
    }

    #[inline]
    fn from_raw(raw: RawSyntaxKind) -> Self {
        Self::from(raw.0)
    }

    fn is_root(&self) -> bool {
        AnyJsRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        JsSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            JsSyntaxKind::NEWLINE
                | JsSyntaxKind::WHITESPACE
                | JsSyntaxKind::COMMENT
                | JsSyntaxKind::MULTILINE_COMMENT
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        JsSyntaxKind::to_string(self)
    }
}

impl TryFrom<JsSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: JsSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                JsSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                JsSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                JsSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                JsSyntaxKind::MULTILINE_COMMENT => Ok(TriviaPieceKind::MultiLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}

/// See: [MDN Operator precedence](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Operators/Operator_Precedence#table)
#[derive(Debug, Eq, Ord, PartialOrd, PartialEq, Copy, Clone, Hash)]
pub enum OperatorPrecedence {
    Comma = 0,
    Yield = 1,
    Assignment = 2,
    Conditional = 3,
    Coalesce = 4,
    LogicalOr = 5,
    LogicalAnd = 6,
    BitwiseOr = 7,
    BitwiseXor = 8,
    BitwiseAnd = 9,
    Equality = 10,
    Relational = 11,
    Shift = 12,
    Additive = 13,
    Multiplicative = 14,
    Exponential = 15,
    Unary = 16,
    Update = 17,
    // `new` without arguments list
    NewWithoutArguments = 18,
    LeftHandSide = 19,
    Member = 20,
    Primary = 21,
    Group = 22,
}

impl OperatorPrecedence {
    /// Returns the operator with the lowest precedence
    pub fn lowest() -> Self {
        OperatorPrecedence::Comma
    }

    /// Returns the operator with the highest precedence
    pub fn highest() -> Self {
        OperatorPrecedence::Primary
    }

    /// Returns `true` if this operator has right to left associativity
    pub fn is_right_to_left(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::Yield
                | OperatorPrecedence::Assignment
                | OperatorPrecedence::Conditional
                | OperatorPrecedence::Exponential
                | OperatorPrecedence::Update
        )
    }

    /// Returns the precedence for a binary operator token or [None] if the token isn't a binary operator
    pub fn try_from_binary_operator(kind: JsSyntaxKind) -> Option<OperatorPrecedence> {
        Some(match kind {
            T![??] => OperatorPrecedence::Coalesce,
            T![||] => OperatorPrecedence::LogicalOr,
            T![&&] => OperatorPrecedence::LogicalAnd,
            T![|] => OperatorPrecedence::BitwiseOr,
            T![^] => OperatorPrecedence::BitwiseXor,
            T![&] => OperatorPrecedence::BitwiseAnd,
            T![==] | T![!=] | T![===] | T![!==] => OperatorPrecedence::Equality,
            T![<] | T![>] | T![<=] | T![>=] | T![instanceof] | T![in] | T![as] | T![satisfies] => {
                OperatorPrecedence::Relational
            }
            T![<<] | T![>>] | T![>>>] => OperatorPrecedence::Shift,
            T![+] | T![-] => OperatorPrecedence::Additive,
            T![*] | T![/] | T![%] => OperatorPrecedence::Multiplicative,
            T![**] => OperatorPrecedence::Exponential,
            _ => return None,
        })
    }

    pub const fn is_bitwise(&self) -> bool {
        matches!(
            self,
            OperatorPrecedence::BitwiseAnd
                | OperatorPrecedence::BitwiseOr
                | OperatorPrecedence::BitwiseXor
        )
    }

    pub const fn is_shift(&self) -> bool {
        matches!(self, OperatorPrecedence::Shift)
    }

    pub const fn is_additive(&self) -> bool {
        matches!(self, OperatorPrecedence::Additive)
    }

    pub const fn is_equality(&self) -> bool {
        matches!(self, OperatorPrecedence::Equality)
    }

    pub const fn is_multiplicative(&self) -> bool {
        matches!(self, OperatorPrecedence::Multiplicative)
    }

    pub const fn is_exponential(&self) -> bool {
        matches!(self, OperatorPrecedence::Exponential)
    }
}

/// Similar to [JsSyntaxToken::text_trimmed()], but removes the quotes of string literals.
///
/// ## Examples
///
/// ```
/// use biome_js_syntax::{JsSyntaxKind, JsSyntaxToken, inner_string_text};
///
/// let a = JsSyntaxToken::new_detached(JsSyntaxKind::JS_STRING_LITERAL, "'inner_string_text'", [], []);
/// let b = JsSyntaxToken::new_detached(JsSyntaxKind::JS_STRING_LITERAL, "\"inner_string_text\"", [], []);
/// assert_eq!(inner_string_text(&a), inner_string_text(&b));
///
/// let a = JsSyntaxToken::new_detached(JsSyntaxKind::LET_KW, "let", [], []);
/// let b = JsSyntaxToken::new_detached(JsSyntaxKind::LET_KW, "let", [], []);
/// assert_eq!(inner_string_text(&a), inner_string_text(&b));
///
/// let a = JsSyntaxToken::new_detached(JsSyntaxKind::LET_KW, "let", [], []);
/// let b = JsSyntaxToken::new_detached(JsSyntaxKind::CONST_KW, "const", [], []);
/// assert!(inner_string_text(&a) != inner_string_text(&b));
/// ```
pub fn inner_string_text(token: &JsSyntaxToken) -> TokenText {
    let mut text = token.token_text_trimmed();
    if matches!(
        token.kind(),
        JsSyntaxKind::JS_STRING_LITERAL | JsSyntaxKind::JSX_STRING_LITERAL
    ) {
        // remove string delimiters
        // SAFETY: string literal token have a delimiters at the start and the end of the string
        let range = TextRange::new(1.into(), text.len() - TextSize::from(1));
        text = text.slice(range);
    }
    text
}

/// Returns `Ok(true)` if `maybe_argument` is an argument of a [test call expression](JsCallExpression::is_test_call_expression).
pub fn is_test_call_argument(maybe_argument: &JsSyntaxNode) -> SyntaxResult<bool> {
    let call_expression = maybe_argument
        .parent()
        .and_then(JsCallArgumentList::cast)
        .and_then(|args| args.syntax().grand_parent())
        .and_then(JsCallExpression::cast);

    call_expression.map_or(Ok(false), |call| call.is_test_call_expression())
}
