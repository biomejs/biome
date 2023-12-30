use std::borrow::Cow;

use crate::prelude::*;
use biome_css_syntax::CssLanguage;
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
