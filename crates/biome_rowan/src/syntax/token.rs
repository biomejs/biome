use crate::green::{GreenToken, GreenTrivia};
use crate::syntax::element::SyntaxElementKey;
use crate::syntax::SyntaxTrivia;
use crate::token_text::TokenText;
use crate::{
    chain_trivia_pieces, cursor, Direction, Language, NodeOrToken, SyntaxElement, SyntaxKind,
    SyntaxNode, SyntaxTriviaPiece, TriviaPiece, TriviaPieceKind,
};
use biome_text_size::{TextLen, TextRange, TextSize};
use std::fmt;
use std::marker::PhantomData;

use super::trivia::{trim_leading_trivia_pieces, trim_trailing_trivia_pieces};

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken<L: Language> {
    raw: cursor::SyntaxToken,
    _p: PhantomData<L>,
}

impl<L: Language> SyntaxToken<L> {
    /// Create a new token detached from any tree
    ///
    /// This is mainly useful for creating a small number of individual tokens
    /// when mutating an existing tree, the bulk of the tokens in a given file
    /// should be created through the [crate::TreeBuilder] abstraction instead
    /// as it will efficiently cache and reuse the created tokens
    pub fn new_detached<Leading, Trailing>(
        kind: L::Kind,
        text: &str,
        leading: Leading,
        trailing: Trailing,
    ) -> Self
    where
        Leading: IntoIterator<Item = TriviaPiece>,
        Leading::IntoIter: ExactSizeIterator,
        Trailing: IntoIterator<Item = TriviaPiece>,
        Trailing::IntoIter: ExactSizeIterator,
    {
        Self {
            raw: cursor::SyntaxToken::new_detached(GreenToken::with_trivia(
                kind.to_raw(),
                text,
                GreenTrivia::new(leading),
                GreenTrivia::new(trailing),
            )),
            _p: PhantomData,
        }
    }

    pub(super) fn green_token(&self) -> GreenToken {
        self.raw.green().to_owned()
    }

    pub fn key(&self) -> SyntaxElementKey {
        let (node_data, offset) = self.raw.key();
        SyntaxElementKey::new(node_data, offset)
    }

    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.raw.kind())
    }

    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }

    pub fn text_trimmed_range(&self) -> TextRange {
        self.raw.text_trimmed_range()
    }

    pub fn index(&self) -> usize {
        self.raw.index()
    }

    /// Returns the text of the token, including all trivia.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// })
    /// .first_token()
    /// .unwrap();
    /// assert_eq!("\n\t let \t\t", token.text());
    /// ```
    pub fn text(&self) -> &str {
        self.raw.text()
    }

    /// Returns the text of a token, including all trivia as an owned value.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// })
    /// .first_token()
    /// .unwrap();
    /// assert_eq!("\n\t let \t\t", token.token_text());
    /// assert_eq!(
    ///     format!("{}", "\n\t let \t\t"),
    ///     format!("{}", token.token_text())
    /// );
    /// assert_eq!(
    ///     format!("{:?}", "\n\t let \t\t"),
    ///     format!("{:?}", token.token_text())
    /// );
    /// ```
    pub fn token_text(&self) -> TokenText {
        self.raw.token_text()
    }

    pub fn token_text_trimmed(&self) -> TokenText {
        self.raw.token_text_trimmed()
    }

    /// Returns the text of the token, excluding all trivia.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// })
    /// .first_token()
    /// .unwrap();
    /// assert_eq!("let", token.text_trimmed());
    /// ```
    pub fn text_trimmed(&self) -> &str {
        self.raw.text_trimmed()
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        self.raw.parent().map(SyntaxNode::from)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.ancestors().map(SyntaxNode::from)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.next_sibling_or_token().map(NodeOrToken::from)
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.prev_sibling_or_token().map(NodeOrToken::from)
    }

    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw
            .siblings_with_tokens(direction)
            .map(SyntaxElement::from)
    }

    /// Next token in the tree (i.e, not necessary a sibling).
    pub fn next_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.next_token().map(SyntaxToken::from)
    }
    /// Previous token in the tree (i.e, not necessary a sibling).
    pub fn prev_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.prev_token().map(SyntaxToken::from)
    }

    /// Return a new version of this token detached from its parent node
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn detach(self) -> Self {
        Self {
            raw: self.raw.detach(),
            _p: PhantomData,
        }
    }

    /// Return a new version of this token with its leading trivia replaced with `trivia`
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_leading_trivia<'a, I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = (TriviaPieceKind, &'a str)>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut token_text = String::new();
        let trivia = trivia.into_iter().map(|(kind, text)| {
            token_text.push_str(text);
            TriviaPiece::new(kind, TextSize::of(text))
        });

        let leading = GreenTrivia::new(trivia);

        // Copy over token text and trailing trivia
        let leading_len = self.raw.green().leading_trivia().text_len();
        token_text.push_str(&self.text()[usize::from(leading_len)..]);

        Self {
            raw: cursor::SyntaxToken::new_detached(GreenToken::with_trivia(
                self.kind().to_raw(),
                &token_text,
                leading,
                self.green_token().trailing_trivia().clone(),
            )),
            _p: PhantomData,
        }
    }

    /// Return a new version of this token with its leading trivia replaced with `trivia`
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_leading_trivia_pieces<I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut token_text = String::new();
        let trivia = trivia.into_iter().map(|piece| {
            token_text.push_str(piece.text());
            piece.into_raw_piece()
        });

        let leading = GreenTrivia::new(trivia);

        // Copy over token text and trailing trivia
        let leading_len = self.raw.green().leading_trivia().text_len();
        token_text.push_str(&self.text()[usize::from(leading_len)..]);

        Self {
            raw: cursor::SyntaxToken::new_detached(GreenToken::with_trivia(
                self.kind().to_raw(),
                &token_text,
                leading,
                self.green_token().trailing_trivia().clone(),
            )),
            _p: PhantomData,
        }
    }

    /// Return a new version of this token with its trailing trivia replaced with `trivia`
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_trailing_trivia<'a, I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = (TriviaPieceKind, &'a str)>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut token_text = String::new();

        // copy over leading trivia and token text
        let trailing_len = self.green_token().trailing_trivia().text_len();
        token_text.push_str(&self.text()[..usize::from(self.text().text_len() - trailing_len)]);

        let trivia = trivia.into_iter().map(|(kind, text)| {
            token_text.push_str(text);
            TriviaPiece::new(kind, TextSize::of(text))
        });

        let trailing = GreenTrivia::new(trivia);

        Self {
            raw: cursor::SyntaxToken::new_detached(GreenToken::with_trivia(
                self.kind().to_raw(),
                &token_text,
                self.green_token().leading_trivia().clone(),
                trailing,
            )),
            _p: PhantomData,
        }
    }

    /// Return a new version of this token with its trailing trivia replaced with `trivia`
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn with_trailing_trivia_pieces<I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        let mut token_text = String::new();

        // copy over leading trivia and token text
        let trailing_len = self.green_token().trailing_trivia().text_len();
        token_text.push_str(&self.text()[..usize::from(self.text().text_len() - trailing_len)]);

        let trivia = trivia.into_iter().map(|piece| {
            token_text.push_str(piece.text());
            piece.into_raw_piece()
        });

        let trailing = GreenTrivia::new(trivia);

        Self {
            raw: cursor::SyntaxToken::new_detached(GreenToken::with_trivia(
                self.kind().to_raw(),
                &token_text,
                self.green_token().leading_trivia().clone(),
                trailing,
            )),
            _p: PhantomData,
        }
    }

    // Return a new version of this token with `trivia` prepended to its leading trivia.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind};
    /// use biome_rowan::{RawSyntaxToken, SyntaxToken, TriviaPiece};
    ///
    /// let token = SyntaxToken::<RawLanguage>::new_detached(
    ///     RawLanguageKind::LET_TOKEN,
    ///     "/*c*/ let \t",
    ///     [TriviaPiece::multi_line_comment(5), TriviaPiece::whitespace(1)],
    ///     [TriviaPiece::whitespace(2)]
    /// );
    /// let new_token = token.prepend_trivia_pieces(token.trailing_trivia().pieces());
    ///
    /// assert_eq!(
    ///     format!("{:?}", new_token),
    ///     "LET_TOKEN@0..13 \"let\" [Whitespace(\" \\t\"), Comments(\"/*c*/\"), Whitespace(\" \")] [Whitespace(\" \\t\")]"
    /// );
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn prepend_trivia_pieces<I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        self.with_leading_trivia_pieces(chain_trivia_pieces(
            trivia.into_iter(),
            self.leading_trivia().pieces(),
        ))
    }

    // Return a new version of this token with `trivia` appended to its trailing trivia.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind};
    /// use biome_rowan::{RawSyntaxToken, SyntaxToken, TriviaPiece};
    ///
    /// let token = SyntaxToken::<RawLanguage>::new_detached(
    ///     RawLanguageKind::LET_TOKEN,
    ///     "\t let /*c*/",
    ///     [TriviaPiece::whitespace(2)],
    ///     [TriviaPiece::whitespace(1), TriviaPiece::multi_line_comment(5)],
    /// );
    /// let new_token = token.append_trivia_pieces(token.leading_trivia().pieces());
    ///
    /// assert_eq!(
    ///     format!("{:?}", new_token),
    ///     "LET_TOKEN@0..13 \"let\" [Whitespace(\"\\t \")] [Whitespace(\" \"), Comments(\"/*c*/\"), Whitespace(\"\\t \")]"
    /// );
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn append_trivia_pieces<I>(&self, trivia: I) -> Self
    where
        I: IntoIterator<Item = SyntaxTriviaPiece<L>>,
        I::IntoIter: ExactSizeIterator,
    {
        self.with_trailing_trivia_pieces(chain_trivia_pieces(
            self.trailing_trivia().pieces(),
            trivia.into_iter(),
        ))
    }

    /// Return a new version of this token without leading newlines and whitespaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind};
    /// use biome_rowan::{RawSyntaxToken, SyntaxToken, TriviaPiece};
    ///
    /// let token = SyntaxToken::<RawLanguage>::new_detached(
    ///     RawLanguageKind::LET_TOKEN,
    ///     "\n\t /*c*/ let \t",
    ///     [TriviaPiece::newline(1), TriviaPiece::whitespace(2), TriviaPiece::multi_line_comment(5), TriviaPiece::whitespace(1)],
    ///     [TriviaPiece::whitespace(2)]
    /// );
    /// let new_token = token.trim_leading_trivia();
    ///
    /// assert_eq!(
    ///     format!("{:?}", new_token),
    ///     "LET_TOKEN@0..11 \"let\" [Comments(\"/*c*/\"), Whitespace(\" \")] [Whitespace(\" \\t\")]"
    /// );
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn trim_leading_trivia(&self) -> Self {
        self.with_leading_trivia_pieces(trim_leading_trivia_pieces(self.leading_trivia().pieces()))
    }

    /// Return a new version of this token without trailing whitespaces.
    ///
    /// # Examples
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind};
    /// use biome_rowan::{RawSyntaxToken, SyntaxToken, TriviaPiece};
    ///
    /// let token = SyntaxToken::<RawLanguage>::new_detached(
    ///     RawLanguageKind::LET_TOKEN,
    ///     "\n let /*c*/\t ",
    ///     [TriviaPiece::newline(1), TriviaPiece::whitespace(1)],
    ///     [TriviaPiece::whitespace(1), TriviaPiece::multi_line_comment(5), TriviaPiece::whitespace(2)]
    /// );
    /// let new_token = token.trim_trailing_trivia();
    ///
    /// assert_eq!(
    ///     format!("{:?}", new_token),
    ///     "LET_TOKEN@0..11 \"let\" [Newline(\"\\n\"), Whitespace(\" \")] [Whitespace(\" \"), Comments(\"/*c*/\")]"
    /// );
    /// ```
    #[must_use = "syntax elements are immutable, the result of update methods must be propagated to have any effect"]
    pub fn trim_trailing_trivia(&self) -> Self {
        self.with_trailing_trivia_pieces(trim_trailing_trivia_pieces(
            self.trailing_trivia().pieces(),
        ))
    }

    /// Return whitespace that juxtapose the token until the first non-whitespace item.
    pub fn indentation_trivia_pieces(&self) -> impl ExactSizeIterator<Item = SyntaxTriviaPiece<L>> {
        let leading_trivia = self.leading_trivia().pieces();
        let skip_count = leading_trivia.len()
            - leading_trivia
                .rev()
                .position(|x| !x.is_whitespace())
                .map(|pos| pos + 1)
                .unwrap_or(0);
        self.leading_trivia().pieces().skip(skip_count)
    }

    /// Returns the token's leading trivia.
    ///
    /// Looking backward in the text, a token owns all of its preceding trivia up to and including the first newline character.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// })
    /// .first_token()
    /// .unwrap();
    /// assert_eq!("\n\t ", token.leading_trivia().text());
    /// ```
    #[inline]
    pub fn leading_trivia(&self) -> SyntaxTrivia<L> {
        SyntaxTrivia::new(self.raw.leading_trivia())
    }

    /// Returns the token's trailing trivia.
    ///
    /// A token owns all of its following trivia up to, but not including, the next newline character.
    ///
    /// ```
    /// use biome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use biome_rowan::*;
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// })
    /// .first_token()
    /// .unwrap();
    /// assert_eq!(" \t\t", token.trailing_trivia().text());
    /// ```
    #[inline]
    pub fn trailing_trivia(&self) -> SyntaxTrivia<L> {
        SyntaxTrivia::new(self.raw.trailing_trivia())
    }

    /// Checks if the current token has trailing comments
    pub fn has_trailing_comments(&self) -> bool {
        self.trailing_trivia()
            .pieces()
            .any(|piece| piece.is_comments())
    }

    /// Checks if the token has any trailing trivia that is a whitespace
    pub fn has_trailing_whitespace(&self) -> bool {
        self.trailing_trivia()
            .pieces()
            .any(|piece| piece.is_whitespace())
    }

    /// Checks if the current token has leading comments
    pub fn has_leading_comments(&self) -> bool {
        self.leading_trivia()
            .pieces()
            .any(|piece| piece.is_comments())
    }

    /// Checks if the token has any leading trivia that is a whitespace or a line break
    pub fn has_leading_whitespace_or_newline(&self) -> bool {
        self.leading_trivia()
            .pieces()
            .any(|piece| piece.is_whitespace() || piece.is_newline())
    }

    /// Checks if the current token has leading newline
    pub fn has_leading_newline(&self) -> bool {
        self.leading_trivia()
            .pieces()
            .any(|piece| piece.is_newline())
    }
}

impl<L: Language> fmt::Debug for SyntaxToken<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}@{:?} {:?} ",
            self.kind(),
            self.text_range(),
            self.text_trimmed()
        )?;

        self.leading_trivia().fmt(f)?;
        write!(f, " ")?;
        self.trailing_trivia().fmt(f)
    }
}

impl<L: Language> fmt::Display for SyntaxToken<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<L: Language> From<SyntaxToken<L>> for cursor::SyntaxToken {
    fn from(token: SyntaxToken<L>) -> cursor::SyntaxToken {
        token.raw
    }
}

impl<L: Language> From<cursor::SyntaxToken> for SyntaxToken<L> {
    fn from(raw: cursor::SyntaxToken) -> SyntaxToken<L> {
        SyntaxToken {
            raw,
            _p: PhantomData,
        }
    }
}
