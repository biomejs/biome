mod quotes;
mod raw_interpolation;

use self::raw_interpolation::FormatRawScssStringInterpolation;
use crate::prelude::*;
use biome_css_syntax::{
    AnyScssInterpolatedStringPart, ScssInterpolatedString, ScssInterpolatedStringFields,
};

pub(crate) use self::quotes::ScssInterpolatedStringQuotes;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssInterpolatedString;

impl FormatNodeRule<ScssInterpolatedString> for FormatScssInterpolatedString {
    fn fmt_fields(&self, node: &ScssInterpolatedString, f: &mut CssFormatter) -> FormatResult<()> {
        let ScssInterpolatedStringFields {
            opening_quote_token,
            parts,
            closing_quote_token,
        } = node.as_fields();
        let opening_quote_token = opening_quote_token?;
        let closing_quote_token = closing_quote_token?;
        let quotes = ScssInterpolatedStringQuotes::new(
            &opening_quote_token,
            &parts,
            f.options().quote_style(),
        );

        quotes.write_quote_token(&opening_quote_token, f)?;

        for part in parts {
            match part {
                AnyScssInterpolatedStringPart::ScssInterpolation(interpolation) => {
                    FormatRawScssStringInterpolation::new(&interpolation, quotes).fmt(f)?;
                }
                AnyScssInterpolatedStringPart::ScssStringText(text) => {
                    text.format().with_options(quotes).fmt(f)?;
                }
            }
        }

        quotes.write_quote_token(&closing_quote_token, f)
    }
}
