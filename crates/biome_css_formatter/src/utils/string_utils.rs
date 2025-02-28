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
    trivia::format_replaced,
    Format, FormatResult,
};
use biome_rowan::SyntaxToken;
use biome_string_case::StrLikeExtension;

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

#[derive(Eq, PartialEq, Debug)]
pub(crate) enum StringLiteralParentKind {
    /// Variants to track tokens that are inside a CssCharasetRule
    /// @charset must always have double quotes: https://www.w3.org/TR/css-syntax-3/#determine-the-fallback-encoding
    CharsetAtRule,
    /// other types, will add more later
    Others,
}

/// Data structure of convenience to format string literals. This is copied
/// from the JS formatter, but should eventually have the logic made generic
/// and reusable since many languages will have the same needs.
pub(crate) struct FormatLiteralStringToken<'token> {
    /// The current token
    token: &'token CssSyntaxToken,

    // The parent that holds the token
    parent_kind: StringLiteralParentKind,
}

impl<'token> FormatLiteralStringToken<'token> {
    pub fn new(token: &'token CssSyntaxToken, parent_kind: StringLiteralParentKind) -> Self {
        Self { token, parent_kind }
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
                preferred_quote: chosen_quote,
            };
        }

        let literal = self.token().text_trimmed();
        let alternate_quote = chosen_quote.other();
        let chosen_quote_byte = chosen_quote.as_byte();
        let alternate_quote_byte = alternate_quote.as_byte();

        let quoteless = &literal[1..literal.len() - 1];
        let (chosen_quote_count, alternate_quote_count) = quoteless.bytes().fold(
            (0u32, 0u32),
            |(chosen_quote_count, alternate_quote_count), current_character| {
                if current_character == chosen_quote_byte {
                    (chosen_quote_count + 1, alternate_quote_count)
                } else if current_character == alternate_quote_byte {
                    (chosen_quote_count, alternate_quote_count + 1)
                } else {
                    (chosen_quote_count, alternate_quote_count)
                }
            },
        );

        StringInformation {
            preferred_quote: if chosen_quote_count > alternate_quote_count {
                alternate_quote
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
        match self.token.parent_kind {
            StringLiteralParentKind::CharsetAtRule => {
                let string_information = StringInformation {
                    preferred_quote: QuoteStyle::Double,
                };
                self.normalise_tokens(string_information)
            }
            StringLiteralParentKind::Others => {
                let string_information = self
                    .token
                    .compute_string_information(self.chosen_quote_style);

                // Normalize string token and non-string token.
                //
                // Add the chosen quotes to any non-string tokensto normalize them into strings.
                //
                // CSS has various places where "string-like" tokens can be used without quotes, but the
                // semantics aren't affected by whether they are present or not. This function lets those
                // tokens become string literals by safely adding quotes around them.
                self.normalise_tokens(string_information)
            }
        }
    }

    fn get_token(&self) -> &'token CssSyntaxToken {
        self.token.token()
    }

    fn normalise_tokens(&self, string_information: StringInformation) -> Cow<'token, str> {
        let preferred_quote = string_information.preferred_quote;
        let polished_raw_content = self.normalize_string(&string_information);

        match polished_raw_content {
            Cow::Borrowed(raw_content) => self.swap_quotes(raw_content, &string_information),
            Cow::Owned(mut s) => {
                // content is owned, meaning we allocated a new string,
                // so we force replacing quotes, regardless
                s.insert(0, preferred_quote.as_char());
                s.push(preferred_quote.as_char());
                Cow::Owned(s)
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
        let preferred_quote = string_information.preferred_quote.as_char();
        let original = self.get_token().text_trimmed();

        if original.starts_with(preferred_quote) {
            Cow::Borrowed(original)
        } else {
            Cow::Owned(std::format!(
                "{preferred_quote}{content_to_use}{preferred_quote}",
            ))
        }
    }
}
