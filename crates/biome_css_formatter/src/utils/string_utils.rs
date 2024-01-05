use std::borrow::Cow;

use crate::context::CssFormatOptions;
use crate::prelude::*;
use biome_css_syntax::CssLanguage;
use biome_css_syntax::CssSyntaxKind::{CSS_STRING_LITERAL, CSS_URL_VALUE_RAW_LITERAL, IDENT};
use biome_css_syntax::CssSyntaxToken;
use biome_formatter::token::string::normalize_string;
use biome_formatter::QuoteStyle;
use biome_formatter::{
    prelude::{dynamic_text, write},
    token::string::ToAsciiLowercaseCow,
    trivia::format_replaced,
    Format, FormatResult,
};
use biome_rowan::SyntaxToken;

use crate::{prelude::CssFormatContext, AsFormat, CssFormatter};

pub(crate) struct FormatTokenAsLowercase {
    token: SyntaxToken<CssLanguage>,
}

impl From<SyntaxToken<CssLanguage>> for FormatTokenAsLowercase {
    fn from(value: SyntaxToken<CssLanguage>) -> Self {
        Self { token: value }
    }
}

impl Format<CssFormatContext> for FormatTokenAsLowercase {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let original = self.token.text_trimmed();
        match original.to_ascii_lowercase_cow() {
            Cow::Borrowed(_) => write!(f, [self.token.format()]),
            Cow::Owned(lowercase) => write!(
                f,
                [format_replaced(
                    &self.token,
                    &dynamic_text(&lowercase, self.token.text_trimmed_range().start()),
                )]
            ),
        }
    }
}

/// Data structure of convenience to format string literals. This is copied
/// from the JS formatter, but should eventually have the logic made generic
/// and reusable since many languages will have the same needs.
pub(crate) struct FormatLiteralStringToken<'token> {
    /// The current token
    token: &'token CssSyntaxToken,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token CssSyntaxToken) -> Self {
        Self { token }
    }

    fn token(&self) -> &'token CssSyntaxToken {
        self.token
    }

    pub fn clean_text(&self, options: &CssFormatOptions) -> CleanedStringLiteralText {
        let token = self.token();
        debug_assert!(
            matches!(
                token.kind(),
                CSS_STRING_LITERAL | CSS_URL_VALUE_RAW_LITERAL | IDENT
            ),
            "Found kind {:?}",
            token.kind()
        );

        let chosen_quote_style = options.quote_style();
        let mut string_cleaner = LiteralStringNormaliser::new(self, chosen_quote_style);

        let content = string_cleaner.normalise_text();

        CleanedStringLiteralText {
            text: content,
            token,
        }
    }
}

pub(crate) struct CleanedStringLiteralText<'a> {
    token: &'a CssSyntaxToken,
    text: Cow<'a, str>,
}

impl Format<CssFormatContext> for CleanedStringLiteralText<'_> {
    fn fmt(&self, f: &mut Formatter<CssFormatContext>) -> FormatResult<()> {
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

impl Format<CssFormatContext> for FormatLiteralStringToken<'_> {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
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
        // For anything other than string literals, the token won't have
        // pre-existing quotes, so we can just immediately, safely use the
        // preferred quote style without having to check the content.
        if !matches!(self.token().kind(), CSS_STRING_LITERAL) {
            return StringInformation {
                raw_content_has_quotes: false,
                preferred_quote: chosen_quote,
            };
        }

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
}

impl<'token> LiteralStringNormaliser<'token> {
    pub fn new(
        token: &'token FormatLiteralStringToken<'_>,
        chosen_quote_style: QuoteStyle,
    ) -> Self {
        Self {
            token,
            chosen_quote_style,
        }
    }

    fn normalise_text(&mut self) -> Cow<'token, str> {
        let string_information = self
            .token
            .compute_string_information(self.chosen_quote_style);

        match self.token.token.kind() {
            CSS_STRING_LITERAL => self.normalise_string_literal(string_information),
            _ => self.normalise_non_string_token(string_information),
        }
    }

    fn get_token(&self) -> &'token CssSyntaxToken {
        self.token.token()
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

    /// Add the chosen quotes to any other kind of token to normalize it into a string.
    ///
    /// CSS has various places where "string-like" tokens can be used without quotes, but the
    /// semantics aren't affected by whether they are present or not. This function lets those
    /// tokens become string literals by safely adding quotes around them.
    fn normalise_non_string_token(
        &self,
        string_information: StringInformation,
    ) -> Cow<'token, str> {
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

        normalize_string(raw_content, string_information.preferred_quote.into(), true)
    }

    fn raw_content(&self) -> &'token str {
        let token = self.get_token();
        match token.kind() {
            CSS_STRING_LITERAL => {
                let content = token.text_trimmed();
                &content[1..content.len() - 1]
            }
            _ => token.text_trimmed(),
        }
    }

    fn swap_quotes(
        &self,
        content_to_use: &'token str,
        string_information: &StringInformation,
    ) -> Cow<'token, str> {
        let original_content = self.get_token().text_trimmed();
        let preferred_quote = string_information.preferred_quote;

        let raw_content_has_quotes = string_information.raw_content_has_quotes;

        if raw_content_has_quotes {
            Cow::Borrowed(original_content)
        } else if !original_content.starts_with(preferred_quote.as_char()) {
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
