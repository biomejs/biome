use crate::CssFormatter;
use crate::prelude::*;
use biome_css_syntax::{AnyCssImportLayer, AnyCssImportUrl, CssImportSupports, CssMediaQueryList};
use biome_formatter::{FormatResult, write};
use biome_rowan::SyntaxResult;

/// Formats the shared `@import` payload used by CSS imports and SCSS plain imports.
pub(crate) fn write_import_payload(
    f: &mut CssFormatter,
    url: &SyntaxResult<AnyCssImportUrl>,
    layer: Option<&AnyCssImportLayer>,
    supports: Option<&CssImportSupports>,
    media: &CssMediaQueryList,
) -> FormatResult<()> {
    let has_any_modifiers = layer.is_some() || supports.is_some() || !media.is_empty();

    if has_any_modifiers {
        let modifiers = format_once(|f| {
            let separator = soft_line_break_or_space();
            let mut fill = f.fill();

            fill.entry(&separator, &url.format());

            if let Some(layer) = layer {
                fill.entry(&separator, &layer.format());
            }

            if let Some(supports) = supports {
                fill.entry(&separator, &supports.format());
            }

            if !media.is_empty() {
                fill.entry(&separator, &media.format());
            }

            fill.finish()
        });

        write!(f, [group(&indent(&modifiers))])
    } else {
        write!(f, [url.format()])
    }
}
