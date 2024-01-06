use crate::context::{JsFormatOptions, QuoteProperties};
use crate::prelude::*;
use biome_formatter::token::string::normalize_string;
use biome_formatter::QuoteStyle;
use biome_js_syntax::JsSyntaxKind::{JSX_STRING_LITERAL, JS_STRING_LITERAL};
use biome_js_syntax::{JsFileSource, JsSyntaxToken};
use biome_unicode_table::is_js_ident;
use std::borrow::Cow;
use unicode_width::UnicodeWidthStr;

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variant to track tokens that are inside an expression
    Expression,
    /// Variant to track tokens that are inside a member
    Member,
    /// Variant used when the string literal is inside a directive. This will apply
    /// a simplified logic of normalisation
    Directive,
}

/// Data structure of convenience to format string literals
pub(crate) struct FormatLiteralStringToken<'token> {
    /// The current token
    token: &'token JsSyntaxToken,

    /// The parent that holds the token
    parent_kind: StringLiteralParentKind,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token JsSyntaxToken, parent_kind: StringLiteralParentKind) -> Self {
        Self { token, parent_kind }
    }

    fn token(&self) -> &'token JsSyntaxToken {
        self.token
    }

    pub fn clean_text(&self, options: &JsFormatOptions) -> CleanedStringLiteralText {
        let token = self.token();
        debug_assert!(
            matches!(token.kind(), JS_STRING_LITERAL | JSX_STRING_LITERAL),
            "Found kind {:?}",
            token.kind()
        );

        let chosen_quote_style = match token.kind() {
            JSX_STRING_LITERAL => options.jsx_quote_style(),
            _ => options.quote_style(),
        };
        let chosen_quote_properties = options.quote_properties();

        let mut string_cleaner =
            LiteralStringNormaliser::new(self, chosen_quote_style, chosen_quote_properties);

        let content = string_cleaner.normalise_text(options.source_type().into());
        let normalized_text_width = content.width();

        CleanedStringLiteralText {
            text: content,
            width: normalized_text_width,
            token,
        }
    }
}

pub(crate) struct CleanedStringLiteralText<'a> {
    token: &'a JsSyntaxToken,
    text: Cow<'a, str>,
    width: usize,
}

impl CleanedStringLiteralText<'_> {
    pub fn width(&self) -> usize {
        self.width
    }
}

impl Format<JsFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        format_replaced(
            self.token,
            &syntax_token_cow_slice(
                self.text.clone(),
                self.token,
                self.token.text_trimmed_range().start(),
            ),
        )
        .fmt(f)
    }
}

impl Format<JsFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let cleaned = self.clean_text(f.options());

        cleaned.fmt(f)
    }
}

/// Data structure of convenience to store some information about the
/// string that has been processed
struct StringInformation {
    /// This is the quote that the is calculated and eventually used inside the string.
    /// It could be different from the one inside the formatter options
    preferred_quote: QuoteStyle,
    /// It flags if the raw content has quotes (single or double). The raw content is the
    /// content of a string literal without the quotes
    raw_content_has_quotes: bool,
}

impl FormatLiteralStringToken<'_> {
    /// This function determines which quotes should be used inside to enclose the string.
    /// The function take as a input the string **without quotes**.
    ///
    /// # How it works
    ///
    /// The function determines the preferred quote and alternate quote.
    /// The preferred quote is the one that comes from the formatter options. The alternate quote is the other one.
    ///
    /// We check how many preferred quotes we have inside the content. If this number is greater then the
    /// number alternate quotes that we have inside the content,
    /// then we swap them, so we can reduce the number of escaped quotes.
    ///
    /// For example, let's suppose that the preferred quote is double, and we have a string like this:
    /// ```js
    /// (" content \"\"\" don't ")
    /// ```
    /// Excluding the quotes at the start and beginning, we have three double quote and one single quote.
    /// If we decided to keep them like this, we would have three escaped quotes.
    ///
    /// But then, we choose the single quote as preferred quote and we would have only one quote that is escaped,
    /// resulting into a string like this:
    /// ```js
    /// (' content """ dont\'t ')
    /// ```
    /// Like this, we reduced the number of escaped quotes.
    fn compute_string_information(&self, chosen_quote: QuoteStyle) -> StringInformation {
        let literal = self.token().text_trimmed();
        let alternate = chosen_quote.other();

        let char_count = literal.chars().count();

        let (preferred_quotes_count, alternate_quotes_count) = literal.chars().enumerate().fold(
            (0, 0),
            |(preferred_quotes_counter, alternate_quotes_counter), (index, current_character)| {
                if index == 0 || index == char_count - 1 {
                    (preferred_quotes_counter, alternate_quotes_counter)
                } else if current_character == chosen_quote.as_char() {
                    (preferred_quotes_counter + 1, alternate_quotes_counter)
                } else if current_character == alternate.as_char() {
                    (preferred_quotes_counter, alternate_quotes_counter + 1)
                } else {
                    (preferred_quotes_counter, alternate_quotes_counter)
                }
            },
        );

        StringInformation {
            raw_content_has_quotes: preferred_quotes_count > 0 || alternate_quotes_count > 0,
            preferred_quote: if preferred_quotes_count > alternate_quotes_count {
                alternate
            } else {
                chosen_quote
            },
        }
    }
}

/// Struct of convenience used to manipulate the string. It saves some state in order to apply
/// the normalise process.
struct LiteralStringNormaliser<'token> {
    /// The current token
    token: &'token FormatLiteralStringToken<'token>,
    /// The quote that was set inside the configuration
    chosen_quote_style: QuoteStyle,
    /// When properties in objects are quoted that was set inside the configuration
    chosen_quote_properties: QuoteProperties,
}

/// Convenience enum to map [biome_js_syntax::JsFileSource] by just reading
/// the type of file
#[derive(Eq, PartialEq)]
pub(crate) enum SourceFileKind {
    TypeScript,
    JavaScript,
}

impl From<JsFileSource> for SourceFileKind {
    fn from(st: JsFileSource) -> Self {
        if st.language().is_typescript() {
            Self::TypeScript
        } else {
            Self::JavaScript
        }
    }
}

impl<'token> LiteralStringNormaliser<'token> {
    pub fn new(
        token: &'token FormatLiteralStringToken<'_>,
        chosen_quote_style: QuoteStyle,
        chosen_quote_properties: QuoteProperties,
    ) -> Self {
        Self {
            token,
            chosen_quote_style,
            chosen_quote_properties,
        }
    }

    fn normalise_text(&mut self, file_source: SourceFileKind) -> Cow<'token, str> {
        let string_information = self
            .token
            .compute_string_information(self.chosen_quote_style);
        match self.token.parent_kind {
            StringLiteralParentKind::Expression => {
                self.normalise_string_literal(string_information)
            }
            StringLiteralParentKind::Directive => self.normalise_directive(&string_information),
            StringLiteralParentKind::Member => {
                self.normalise_type_member(string_information, file_source)
            }
        }
    }

    fn get_token(&self) -> &'token JsSyntaxToken {
        self.token.token()
    }

    fn normalise_directive(&mut self, string_information: &StringInformation) -> Cow<'token, str> {
        let content = self.normalize_string(string_information);
        match content {
            Cow::Borrowed(content) => self.swap_quotes(content, string_information),
            Cow::Owned(content) => Cow::Owned(
                self.swap_quotes(content.as_ref(), string_information)
                    .into_owned(),
            ),
        }
    }

    /// We can change the text only if there are alphanumeric or alphabetic characters, depending on the file source
    fn can_remove_quotes(&self, file_source: SourceFileKind) -> bool {
        if self.chosen_quote_properties == QuoteProperties::Preserve {
            return false;
        }

        let text_to_check = self.raw_content();

        if text_to_check
            .chars()
            .next()
            .map_or(false, |c| c.is_ascii_digit())
        {
            if let Ok(parsed) = text_to_check.parse::<f64>() {
                // In TypeScript, numbers like members have different meaning from numbers.
                // Hence, if we see a number, we bail straightaway
                if file_source == SourceFileKind::TypeScript {
                    return false;
                }

                // Rule out inexact floats and octal literals
                return parsed.to_string() == text_to_check;
            }

            return false;
        }

        is_js_ident(text_to_check)
    }

    fn normalise_type_member(
        &mut self,
        string_information: StringInformation,
        file_source: SourceFileKind,
    ) -> Cow<'token, str> {
        if self.can_remove_quotes(file_source) {
            return Cow::Owned(self.raw_content().to_string());
        }
        self.normalise_string_literal(string_information)
    }

    fn normalise_string_literal(&self, string_information: StringInformation) -> Cow<'token, str> {
        let preferred_quote = string_information.preferred_quote;
        let polished_raw_content = self.normalize_string(&string_information);

        match polished_raw_content {
            Cow::Borrowed(raw_content) => {
                let final_content = self.swap_quotes(raw_content, &string_information);
                match final_content {
                    Cow::Borrowed(final_content) => Cow::Borrowed(final_content),
                    Cow::Owned(final_content) => Cow::Owned(final_content),
                }
            }
            Cow::Owned(s) => {
                // content is owned, meaning we allocated a new string,
                // so we force replacing quotes, regardless
                let final_content = std::format!(
                    "{}{}{}",
                    preferred_quote.as_char(),
                    s.as_str(),
                    preferred_quote.as_char()
                );

                Cow::Owned(final_content)
            }
        }
    }

    fn normalize_string(&self, string_information: &StringInformation) -> Cow<'token, str> {
        let raw_content = self.raw_content();
        let is_escape_preserved = matches!(self.token.token.kind(), JSX_STRING_LITERAL);

        if matches!(self.token.parent_kind, StringLiteralParentKind::Directive) {
            return Cow::Borrowed(raw_content);
        }
        normalize_string(
            raw_content,
            string_information.preferred_quote.into(),
            is_escape_preserved,
        )
    }

    fn raw_content(&self) -> &'token str {
        let content = self.get_token().text_trimmed();
        &content[1..content.len() - 1]
    }

    fn swap_quotes(
        &self,
        content_to_use: &'token str,
        string_information: &StringInformation,
    ) -> Cow<'token, str> {
        let original_content = self.get_token().text_trimmed();
        let preferred_quote = string_information.preferred_quote;
        let other_quote = preferred_quote.other().as_char();

        let raw_content_has_quotes = string_information.raw_content_has_quotes;

        if raw_content_has_quotes {
            Cow::Borrowed(original_content)
        } else if original_content.starts_with(other_quote) {
            Cow::Owned(std::format!(
                "{}{}{}",
                preferred_quote.as_char(),
                content_to_use,
                preferred_quote.as_char()
            ))
        } else {
            Cow::Borrowed(original_content)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::quickcheck_utils::*;
    use crate::utils::FormatLiteralStringToken;
    use biome_formatter::token::string::ToAsciiLowercaseCow;
    use biome_formatter::QuoteStyle;
    use biome_js_factory::JsSyntaxTreeBuilder;
    use biome_js_syntax::JsSyntaxKind::{JS_STRING_LITERAL, JS_STRING_LITERAL_EXPRESSION};
    use biome_js_syntax::{JsStringLiteralExpression, JsSyntaxToken};
    use biome_rowan::AstNode;
    use quickcheck_macros::*;
    use std::borrow::Cow;

    #[quickcheck]
    fn to_ascii_lowercase_cow_always_returns_same_value_as_string_to_lowercase(txt: AsciiString) {
        assert_eq!(
            txt.to_lowercase(),
            txt.to_ascii_lowercase_cow().into_owned()
        );
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_borrowed_when_all_chars_are_lowercase(txt: AsciiString) {
        let txt = txt.to_lowercase();
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Borrowed(s) if s == txt));
    }

    #[quickcheck]
    fn to_ascii_lowercase_cow_returns_owned_when_some_chars_are_not_lowercase(txt: AsciiString) {
        let txt = std::format!("{}A", txt); //guarantees at least one uppercase letter
        assert!(matches!(txt.to_ascii_lowercase_cow(), Cow::Owned(s) if s == txt.to_lowercase()));
    }

    fn generate_syntax_token(input: &str) -> JsSyntaxToken {
        let mut tree_builder = JsSyntaxTreeBuilder::new();
        tree_builder.start_node(JS_STRING_LITERAL_EXPRESSION);
        tree_builder.token(JS_STRING_LITERAL, input);
        tree_builder.finish_node();

        let root = tree_builder.finish();

        JsStringLiteralExpression::cast(root)
            .unwrap()
            .value_token()
            .unwrap()
    }

    enum AsToken {
        Directive,
        String,
        Member,
    }

    impl AsToken {
        fn into_token(self, token: &JsSyntaxToken) -> FormatLiteralStringToken {
            match self {
                AsToken::Directive => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Directive)
                }
                AsToken::String => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Expression)
                }
                AsToken::Member => {
                    FormatLiteralStringToken::new(token, StringLiteralParentKind::Member)
                }
            }
        }
    }

    fn assert_borrowed_token(
        input: &str,
        quote: QuoteStyle,
        quote_properties: QuoteProperties,
        as_token: AsToken,
        source: SourceFileKind,
    ) {
        let token = generate_syntax_token(input);
        let string_token = as_token.into_token(&token);
        let mut string_cleaner =
            LiteralStringNormaliser::new(&string_token, quote, quote_properties);
        let content = string_cleaner.normalise_text(source);
        assert_eq!(content, Cow::Borrowed(input))
    }

    fn assert_owned_token(
        input: &str,
        output: &str,
        quote: QuoteStyle,
        quote_properties: QuoteProperties,
        as_token: AsToken,
        source: SourceFileKind,
    ) {
        let token = generate_syntax_token(input);
        let string_token = as_token.into_token(&token);
        let mut string_cleaner =
            LiteralStringNormaliser::new(&string_token, quote, quote_properties);
        let content = string_cleaner.normalise_text(source);
        let owned: Cow<str> = Cow::Owned(output.to_string());
        assert_eq!(content, owned)
    }

    #[test]
    fn string_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""content""#, r#""content with single ' quote ""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::String,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn string_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [
            (r#"" content '' \"\"\" ""#, r#"' content \'\' """ '"#),
            (r#"" content \"\"\"\" '' ""#, r#"' content """" \'\' '"#),
            (r#"" content ''''' \" ""#, r#"" content ''''' \" ""#),
            (r#"" content \'\' \" ""#, r#"" content '' \" ""#),
            (r#"" content \\' \" ""#, r#"" content \\' \" ""#),
        ];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::String,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn directive_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""use strict '""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::Directive,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn directive_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [(r#"' use strict '"#, r#"" use strict ""#)];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::Directive,
                SourceFileKind::JavaScript,
            )
        }
    }

    #[test]
    fn member_borrowed() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [r#""cant @ be moved""#, r#""1674""#, r#""33n""#];
        for input in inputs {
            assert_borrowed_token(
                input,
                quote,
                quote_properties,
                AsToken::Member,
                SourceFileKind::TypeScript,
            )
        }
    }

    #[test]
    fn member_owned() {
        let quote = QuoteStyle::Double;
        let quote_properties = QuoteProperties::AsNeeded;
        let inputs = [(r#""string""#, r#"string"#)];
        for (input, output) in inputs {
            assert_owned_token(
                input,
                output,
                quote,
                quote_properties,
                AsToken::Member,
                SourceFileKind::TypeScript,
            )
        }
    }
}
