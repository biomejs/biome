use crate::{HtmlString, inner_string_text, is_quoted};
use biome_rowan::{SyntaxResult, TextLen, TextRange, TextSize, TokenText};

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

    /// Returns the range of the string contents in the source file.
    pub fn inner_string_range(&self) -> SyntaxResult<TextRange> {
        let token = self.value_token()?;
        let start = token.text_trimmed_range().start()
            + if is_quoted(token.text_trimmed()) {
                TextSize::from(1)
            } else {
                TextSize::from(0)
            };
        Ok(TextRange::at(start, self.inner_string_text()?.text_len()))
    }
}

#[cfg(test)]
mod tests {
    use biome_html_factory::syntax::HtmlString;
    use biome_html_parser::{HtmlParserOptions, parse_html};
    use biome_rowan::{AstNode, TextRange};

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

    #[test]
    fn inner_string_range_skips_quotes() {
        let string = first_string(r#"<textarea rows="4"></textarea>"#);

        assert_eq!(
            string.inner_string_range().unwrap(),
            TextRange::new(16.into(), 17.into())
        );
    }

    #[test]
    fn inner_string_range_keeps_unquoted_start() {
        let string = first_string("<textarea rows=4></textarea>");

        assert_eq!(
            string.inner_string_range().unwrap(),
            TextRange::new(15.into(), 16.into())
        );
    }
}
