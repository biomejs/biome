use crate::prelude::*;
use biome_css_syntax::{CssImportAtRule, CssImportAtRuleFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportAtRule;
impl FormatNodeRule<CssImportAtRule> for FormatCssImportAtRule {
    fn fmt_fields(&self, node: &CssImportAtRule, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportAtRuleFields {
            import_token,
            url,
            layer,
            supports,
            media,
            semicolon_token,
        } = node.as_fields();

        write!(f, [import_token.format(), space()])?;

        // Determine if there are any modifiers present.
        let has_any_modifiers = layer.is_some() || supports.is_some() || media.len() > 0;

        if has_any_modifiers {
            // If there are, we need to group them together and try to fill them.
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

                if media.len() > 0 {
                    fill.entry(&separator, &media.format());
                }

                fill.finish()
            });
            write!(f, [group(&indent(&modifiers))])?;
        } else {
            // If there are no modifiers, simply write the formatted `url` to the formatter `f`.
            write!(f, [url.format()])?;
        }

        write!(f, [semicolon_token.format()])
    }
}
