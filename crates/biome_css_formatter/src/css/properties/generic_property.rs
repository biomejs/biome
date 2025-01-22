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

        let is_comma: bool = value.iter().any(|v| v.text().eq(","));

        if is_comma {
            write!(
                f,
                [
                    name.format(),
                    colon_token.format(),
                    &soft_line_indent_or_hard_space(&format_once(|f| {
                        for (idx, v) in value.iter().enumerate() {
                            let is_last = idx == value.len() - 1;

                            if is_last {
                                write!(f, [v.format()])?;
                                break;
                            }

                            let Some(next) = value.iter().nth(idx + 1) else {
                                continue;
                            };

                            let next_is_comma = next.text().eq(",");

                            if v.text().eq(",") {
                                write!(f, [v.format(), hard_line_break()])?;
                            } else if next_is_comma {
                                write!(f, [v.format()])?;
                            } else {
                                write!(f, [v.format(), space()])?;
                            }
                        }

                        Ok(())
                    }))
                ]
            )
        } else {
            write!(
                f,
                [name.format(), colon_token.format(), space(), value.format()]
            )
        }
    }
}
