//! A crate for generated Syntax node definitions and utility macros.
//! Both rome_grit_lexer and biome_grit_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#[macro_use]
mod generated;
mod file_source;
pub mod string_value_ext;
mod syntax_node;

use biome_rowan::{AstNode, RawSyntaxKind, SyntaxKind};

pub use biome_rowan::{TextLen, TextRange, TextSize, TokenAtOffset, TriviaPieceKind, WalkEvent};
pub use file_source::GraphqlFileSource;
pub use generated::*;
pub use syntax_node::*;

use GraphqlSyntaxKind::*;

impl From<u16> for GraphqlSyntaxKind {
    fn from(d: u16) -> GraphqlSyntaxKind {
        assert!(d <= (GraphqlSyntaxKind::__LAST as u16));
        unsafe { std::mem::transmute::<u16, GraphqlSyntaxKind>(d) }
    }
}

impl From<GraphqlSyntaxKind> for u16 {
    fn from(k: GraphqlSyntaxKind) -> u16 {
        k as u16
    }
}

impl GraphqlSyntaxKind {
    /// Returns `true` for any contextual (await) or non-contextual keyword
    #[inline]
    pub const fn is_keyword(self) -> bool {
        (self as u16) <= (GraphqlSyntaxKind::INPUT_FIELD_DEFINITION_KW as u16)
            && (self as u16) >= (GraphqlSyntaxKind::TRUE_KW as u16)
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

    fn to_bogus(&self) -> GraphqlSyntaxKind {
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
        GraphqlSyntaxKind::is_list(*self)
    }

    fn is_trivia(self) -> bool {
        matches!(
            self,
            GraphqlSyntaxKind::NEWLINE
                | GraphqlSyntaxKind::WHITESPACE
                | GraphqlSyntaxKind::COMMENT
                | GraphqlSyntaxKind::COMMA
        )
    }

    fn to_string(&self) -> Option<&'static str> {
        GraphqlSyntaxKind::to_string(self)
    }
}

impl TryFrom<GraphqlSyntaxKind> for TriviaPieceKind {
    type Error = ();

    fn try_from(value: GraphqlSyntaxKind) -> Result<Self, Self::Error> {
        if value.is_trivia() {
            match value {
                GraphqlSyntaxKind::NEWLINE => Ok(TriviaPieceKind::Newline),
                GraphqlSyntaxKind::WHITESPACE => Ok(TriviaPieceKind::Whitespace),
                // https://spec.graphql.org/October2021/#sec-Insignificant-Commas
                GraphqlSyntaxKind::COMMA => Ok(TriviaPieceKind::Skipped),
                GraphqlSyntaxKind::COMMENT => Ok(TriviaPieceKind::SingleLineComment),
                _ => unreachable!("Not Trivia"),
            }
        } else {
            Err(())
        }
    }
}
