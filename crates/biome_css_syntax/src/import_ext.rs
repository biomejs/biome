use crate::{AnyCssImportUrl, AnyCssUrlValue};
use biome_rowan::TokenText;

impl AnyCssImportUrl {
    /// Returns the inner text of specifier:
    ///
    /// ## Examples
    ///
    /// ```
    /// use biome_css_factory::make;
    /// use biome_css_syntax::AnyCssImportUrl;
    /// ```
    pub fn inner_string_text(&self) -> Option<TokenText> {
        match self {
            Self::CssString(css) => css.inner_string_text().ok(),
            Self::CssUrlFunction(url_function) => {
                url_function.value().and_then(|value| match value {
                    AnyCssUrlValue::CssString(css) => css.inner_string_text().ok(),
                    AnyCssUrlValue::CssUrlValueRaw(raw) => raw
                        .value_token()
                        .ok()
                        .map(|token| token.token_text_trimmed()),
                })
            }
        }
    }
}
