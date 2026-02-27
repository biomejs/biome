use crate::{HtmlString, inner_string_text};
use biome_rowan::{SyntaxResult, TokenText};

impl HtmlString {
    /// Returns the inner text of a string not including the quotes.
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_html_factory::make;
    /// use biome_rowan::TriviaPieceKind;
    ///
    ///let string = make::html_string(make::html_string_literal("button")
    ///     .with_leading_trivia(vec![(TriviaPieceKind::Whitespace, " ")]));
    /// assert_eq!(string.inner_string_text().unwrap().text(), "button");
    /// ```
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}
