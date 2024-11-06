use crate::prelude::*;
use biome_css_syntax::{CssGenericProperty, CssGenericPropertyFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssGenericProperty;
impl FormatNodeRule<CssGenericProperty> for FormatCssGenericProperty {
    fn fmt_fields(&self, node: &CssGenericProperty, f: &mut CssFormatter) -> FormatResult<()> {
        let CssGenericPropertyFields {
            name,
            colon_token,
            value,
        } = node.as_fields();

        let is_dash_identity = name.clone().unwrap().as_css_dashed_identifier().is_some();

        let has_comma = value.iter().any(|v| v.text().eq(","));

        if has_comma && !is_dash_identity {
            write!(
                f,
                [
                    name.format(),
                    colon_token.format(),
                    indent(&format_with(|f| {
                        let last = value.last().unwrap();

                        for (idx, v) in value.iter().enumerate() {
                            if idx == 0 {
                                write!(f, [soft_line_break_or_space()])?;
                            }

                            if v.text().eq(",") {
                                write!(f, [v.format(), hard_line_break()])?;
                            } else if let Some(next) = value.iter().nth(idx + 1) {
                                if next.text().eq(",") || v == last {
                                    write!(f, [v.format()])?;
                                } else {
                                    write!(f, [v.format(), space()])?;
                                }
                            } else {
                                write!(f, [v.format()])?;
                            }
                        }

                        Ok(())
                    })),
                ]
            )?;

            return Ok(());
        }

        write!(
            f,
            [name.format(), colon_token.format(), space(), value.format()]
        )
    }
}
