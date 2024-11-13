use biome_rowan::TextRange;

use crate::{JsSyntaxKind, JsSyntaxToken};

#[derive(Debug, Clone, Eq, PartialEq)]
/// static values defined in JavaScript's expressions
pub enum StaticValue {
    Boolean(JsSyntaxToken),
    Null(JsSyntaxToken),
    Undefined(JsSyntaxToken),
    Number(JsSyntaxToken),
    BigInt(JsSyntaxToken),
    // The string can be empty.
    String(JsSyntaxToken),
    /// This is used to represent the empty template string.
    EmptyString(TextRange),
}

impl StaticValue {
    /// Return `true` if the value is falsy
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{T, static_value::StaticValue};
    /// use biome_js_factory::make;
    ///
    /// let bool = make::token(T![false]);
    /// assert!(StaticValue::Boolean(bool).is_falsy());
    /// ```
    pub fn is_falsy(&self) -> bool {
        match self {
            StaticValue::Boolean(token) => token.text_trimmed() == "false",
            StaticValue::Null(_) | StaticValue::Undefined(_) | StaticValue::EmptyString(_) => true,
            StaticValue::Number(token) => token.text_trimmed() == "0",
            StaticValue::BigInt(token) => token.text_trimmed() == "0n",
            StaticValue::String(_) => self.text().is_empty(),
        }
    }

    /// Return a string of the static value
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{T, static_value::StaticValue};
    /// use biome_js_factory::make;
    ///
    /// let bool = make::token(T![false]);
    /// assert_eq!(StaticValue::Boolean(bool).text(), "false");
    /// ```
    pub fn text(&self) -> &str {
        match self {
            StaticValue::Boolean(token)
            | StaticValue::Null(token)
            | StaticValue::Undefined(token)
            | StaticValue::Number(token)
            | StaticValue::BigInt(token) => token.text_trimmed(),
            StaticValue::String(token) => {
                let text = token.text_trimmed();
                if matches!(
                    token.kind(),
                    JsSyntaxKind::JS_STRING_LITERAL | JsSyntaxKind::JSX_STRING_LITERAL
                ) {
                    // SAFETY: string literal token have a delimiters at the start and the end of the string
                    return &text[1..text.len() - 1];
                }
                text
            }
            StaticValue::EmptyString(_) => "",
        }
    }

    /// Return teh range of the static value.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{T, static_value::StaticValue};
    /// use biome_js_factory::make;
    ///
    /// let bool = make::token(T![false]);
    /// assert_eq!(StaticValue::Boolean(bool.clone()).range(), bool.text_trimmed_range());
    /// ```
    pub fn range(&self) -> TextRange {
        match self {
            StaticValue::Boolean(token)
            | StaticValue::Null(token)
            | StaticValue::Undefined(token)
            | StaticValue::Number(token)
            | StaticValue::BigInt(token)
            | StaticValue::String(token) => token.text_trimmed_range(),
            StaticValue::EmptyString(range) => *range,
        }
    }

    /// Return `true` if the static value doesn't match the given string value and it is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::static_value::StaticValue;
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let str_literal = make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// assert!(StaticValue::String(str_literal).is_not_string_constant("bar"));
    /// ```
    pub fn is_not_string_constant(&self, text: &str) -> bool {
        match self {
            StaticValue::String(_) | StaticValue::EmptyString(_) => self.text() != text,
            _ => false,
        }
    }

    /// Return a string if the static value is
    /// 1. A string literal
    /// 2. A template literal with no substitutions
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::static_value::StaticValue;
    /// use biome_js_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    /// let str_literal = make::js_string_literal("foo")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]);
    /// assert_eq!(StaticValue::String(str_literal).as_string_constant().unwrap(), "foo");
    /// ```
    pub fn as_string_constant(&self) -> Option<&str> {
        match self {
            StaticValue::String(_) | StaticValue::EmptyString(_) => Some(self.text()),
            _ => None,
        }
    }

    /// Return `true` if the value is null or undefined
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_js_syntax::{T, static_value::StaticValue};
    /// use biome_js_factory::make::{js_null_literal_expression, token};
    ///
    /// let null = js_null_literal_expression(token(T![null]));
    /// assert!(StaticValue::Null(null.value_token().ok().unwrap()).is_null_or_undefined());
    /// ```
    pub fn is_null_or_undefined(&self) -> bool {
        matches!(self, StaticValue::Null(_) | StaticValue::Undefined(_))
    }
}

impl AsRef<str> for StaticValue {
    fn as_ref(&self) -> &str {
        self.text()
    }
}
