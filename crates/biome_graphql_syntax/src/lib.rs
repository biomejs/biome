//! A crate for generated Syntax node definitions and utility macros.
//! Both rome_grit_lexer and biome_grit_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#![deny(clippy::use_self)]

#[macro_use]
mod generated;
mod file_source;
pub mod string_value_ext;
mod syntax_node;

use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind, TokenText};

pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::GraphqlFileSource;
pub use generated::*;
pub use syntax_node::*;

use GraphqlSyntaxKind::*;

impl From<u16> for GraphqlSyntaxKind {
    fn from(d: u16) -> Self {
        assert!(d <= (Self::__LAST as u16));
        unsafe { std::mem::transmute::<u16, Self>(d) }
    }
}

impl From<GraphqlSyntaxKind> for u16 {
    fn from(k: GraphqlSyntaxKind) -> Self {
        k as Self
    }
}

impl GraphqlSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (Self::INPUT_FIELD_DEFINITION_KW as u16)
            && (self as u16) >= (Self::TRUE_KW as u16)
    }
}

impl biome_rowan::SyntaxKind for GraphqlSyntaxKind {
    const TOMBSTONE: Self = TOMBSTONE;
    const EOF: Self = EOF;

    fn is_bogus(&self) -> bool {
        matches!(
            self,
            GRAPHQL_BOGUS
                | GRAPHQL_BOGUS_DEFINITION
                | GRAPHQL_BOGUS_SELECTION
                | GRAPHQL_BOGUS_VALUE
                | GRAPHQL_BOGUS_TYPE
        )
    }

    fn to_bogus(&self) -> Self {
        match self {
            kind if AnyGraphqlDefinition::can_cast(*kind) => GRAPHQL_BOGUS_DEFINITION,
            kind if AnyGraphqlSelection::can_cast(*kind) => GRAPHQL_BOGUS_SELECTION,
            kind if AnyGraphqlValue::can_cast(*kind) => GRAPHQL_BOGUS_VALUE,
            kind if AnyGraphqlType::can_cast(*kind) => GRAPHQL_BOGUS_TYPE,
            _ => GRAPHQL_BOGUS,
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
        GraphqlRoot::can_cast(*self)
    }

    fn is_list(&self) -> bool {
        Self::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            Self::NEWLINE | Self::WHITESPACE | Self::COMMENT | Self::COMMA
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        Self::to_string(self)
    }
}

impl TryFrom<GraphqlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: GraphqlSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                GraphqlSyntaxKind::NEWLINE => Ok(Self::Newline),
                GraphqlSyntaxKind::WHITESPACE => Ok(Self::Whitespace),
                // https://spec.graphql.org/October2021/#sec-Insignificant-Commas
                GraphqlSyntaxKind::COMMA => Ok(Self::Skipped),
                GraphqlSyntaxKind::COMMENT => Ok(Self::SingleLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}

/// Text of `token`, excluding all trivia and removing quotes if `token` is a string literal.
pub fn inner_string_text(token: &GraphqlSyntaxToken) -> TokenText {
    let text = token.token_text_trimmed();
    if token.kind() != GraphqlSyntaxKind::GRAPHQL_STRING_LITERAL {
        return text;
    }
    // remove string delimiters
    // SAFETY: string literal token have a delimiters at the start and the end of the string
    let slice = if text.starts_with("\"\"\"") && text.ends_with("\"\"\"") {
        TextRange::new(3.into(), text.len() - TextSize::from(3))
    } else {
        TextRange::new(1.into(), text.len() - TextSize::from(1))
    };
    text.slice(slice)
}
