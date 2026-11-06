use crate::{CssString, inner_string_text};
use biome_rowan::{SyntaxResult, TokenText};

impl CssString {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}

#[cfg(test)]
mod tests {
    use biome_css_factory::syntax::CssString;
    use biome_css_parser::{CssParserOptions, parse_css};
    use biome_languages::{CssFileSource, css::CssEmbeddingKind};
    use biome_rowan::AstNode;

    #[test]
    fn inner_string_text_removes_html_quote_references() {
        let source =
            r#"content: "a", &quot;b&quot;, &#39;c', "d&#x22;, "&amp;&quot;, &quot;&quot;"#;
        let source_type =
            CssFileSource::css().with_embedding_kind(CssEmbeddingKind::HtmlStyleAttribute);
        let parsed = parse_css(source, source_type, CssParserOptions::default());
        let texts: Vec<_> = parsed
            .syntax()
            .descendants()
            .filter_map(CssString::cast)
            .map(|string| string.inner_string_text().unwrap().to_string())
            .collect();

        assert_eq!(texts, ["a", "b", "c", "d", "&amp;", ""]);
    }
}
