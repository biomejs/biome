use crate::CssFormatter;
use crate::prelude::*;
use crate::utils::media_query_comments::fill_media_queries;
use biome_css_syntax::{AnyCssImportLayer, AnyCssImportUrl, CssImportSupports, CssMediaQueryList};
use biome_formatter::{Format, FormatResult, write};
use biome_rowan::SyntaxResult;

/// Formats the shared import clause used by CSS imports and SCSS plain imports.
///
/// ```css
/// @import "a.css" layer(base) screen;
/// ```
pub(crate) struct FormatImportClause {
    url: SyntaxResult<AnyCssImportUrl>,
    layer: Option<AnyCssImportLayer>,
    supports: Option<CssImportSupports>,
    media: CssMediaQueryList,
}

impl FormatImportClause {
    /// Creates an import clause formatter.
    ///
    /// ```css
    /// @import "a.css" layer(base) screen;
    /// ```
    pub(crate) fn new(
        url: SyntaxResult<AnyCssImportUrl>,
        layer: Option<AnyCssImportLayer>,
        supports: Option<CssImportSupports>,
        media: CssMediaQueryList,
    ) -> Self {
        Self {
            url,
            layer,
            supports,
            media,
        }
    }
}

impl Format<CssFormatContext> for FormatImportClause {
    fn fmt(&self, f: &mut CssFormatter) -> FormatResult<()> {
        let has_any_modifiers =
            self.layer.is_some() || self.supports.is_some() || !self.media.is_empty();

        if has_any_modifiers {
            let separator = soft_line_break_or_space();
            let mut fill = f.fill();

            fill.entry(&separator, &self.url.format());

            if let Some(layer) = &self.layer {
                fill.entry(&separator, &layer.format());
            }

            if let Some(supports) = &self.supports {
                fill.entry(&separator, &supports.format());
            }

            if !self.media.is_empty() {
                fill_media_queries(&self.media, |separator, formatted| {
                    fill.entry(separator, formatted);
                });
            }

            fill.finish()
        } else {
            write!(f, [self.url.format()])
        }
    }
}
