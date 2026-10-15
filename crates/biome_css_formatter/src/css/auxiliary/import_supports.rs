use crate::prelude::*;
use biome_css_syntax::{
    AnyCssImportSupportsCondition, CssImportSupports, CssImportSupportsFields, ScssPlainImport,
};
use biome_formatter::{format_args, write};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssImportSupports;
impl FormatNodeRule<CssImportSupports> for FormatCssImportSupports {
    fn fmt_fields(&self, node: &CssImportSupports, f: &mut CssFormatter) -> FormatResult<()> {
        let CssImportSupportsFields {
            supports_token,
            l_paren_token,
            condition,
            r_paren_token,
        } = node.as_fields();

        let should_insert_space = f.options().delimiter_spacing().value();
        let condition = condition?;
        let has_scss_condition = node.parent::<ScssPlainImport>().is_some()
            && matches!(
                condition,
                AnyCssImportSupportsCondition::AnyCssSupportsCondition(_)
            );

        if has_scss_condition {
            // Sass rejects trailing whitespace in `supports(at-rule(@property))`.
            let separator = if should_insert_space {
                soft_line_break_or_space()
            } else {
                soft_line_break()
            };
            return write!(
                f,
                [
                    supports_token.format()?.with_text_case(CssCase::Preserve),
                    group(&format_args![
                        l_paren_token.format(),
                        indent(&format_args![
                            separator,
                            condition.format().with_text_case(CssCase::Preserve)
                        ]),
                        r_paren_token.format(),
                    ])
                ]
            );
        }

        write!(
            f,
            [
                supports_token.format()?.with_text_case(CssCase::Preserve),
                group(&format_args![
                    l_paren_token.format(),
                    soft_block_indent_with_maybe_space(
                        &condition.format().with_text_case(CssCase::Preserve),
                        should_insert_space
                    ),
                    r_paren_token.format()
                ])
            ]
        )
    }
}
