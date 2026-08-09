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

#[cfg(test)]
mod tests {
    use biome_html_factory::syntax::HtmlString;
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_rowan::AstNode;

    fn first_string(html: &str) -> HtmlString {
        parse_html(html, HtmlParserOptions::default())
            .tree()
            .syntax()
            .descendants()
            .find_map(HtmlString::cast)
            .unwrap()
    }

    #[test]
    fn inner_string_text_strips_quoted_attribute_value() {
        let string = first_string(r#"<textarea rows="4"></textarea>"#);

        assert_eq!(string.inner_string_text().unwrap().text(), "4");
    }

    #[test]
    fn inner_string_text_keeps_unquoted_attribute_value() {
        let string = first_string("<textarea rows=4></textarea>");

        assert_eq!(string.inner_string_text().unwrap().text(), "4");
    }
}
