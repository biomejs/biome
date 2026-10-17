use crate::prelude::*;
use biome_css_syntax::{AnyCssAttrType, CssAttrFunction, CssAttrFunctionFields};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttrFunction;

impl FormatNodeRule<CssAttrFunction> for FormatCssAttrFunction {
    fn fmt_fields(&self, node: &CssAttrFunction, f: &mut CssFormatter) -> FormatResult<()> {
        let CssAttrFunctionFields {
            name_token,
            l_paren_token,
            attr_name,
            attr_type,
            fallback_value,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();

        write!(
            f,
            [
                name_token.format()?.with_text_case(CssCase::Preserve),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &format_with(|f| {
                            write!(f, [group(&attr_name.format())])?;

                            if let Some(attr_type) = &attr_type {
                                write!(f, [soft_line_break_or_space(), attr_type.format()])?;
                                if let Some(fallback) = &fallback_value {
                                    // A suppressed comma-prefixed fallback needs its own line
                                    // so the directive stays leading after reparsing.
                                    let has_suppressed_dynamic_fallback =
                                        matches!(
                                            attr_type,
                                            AnyCssAttrType::ScssVariable(_)
                                                | AnyCssAttrType::ScssNamespacedVariable(_)
                                                | AnyCssAttrType::ScssInterpolatedIdentifier(_)
                                        ) && f.comments().is_suppressed(fallback.syntax());
                                    if has_suppressed_dynamic_fallback
                                        || f.comments()
                                            .has_trailing_line_comment(attr_type.syntax())
                                    {
                                        write!(f, [hard_line_break()])?;
                                    }
                                }
                            }

                            write!(f, [fallback_value.format()])?;

                            Ok(())
                        }),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
