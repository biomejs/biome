use crate::prelude::*;
use biome_css_syntax::ScssModuleMemberList;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatScssModuleMemberList;

impl FormatRule<ScssModuleMemberList> for FormatScssModuleMemberList {
    type Context = CssFormatContext;

    fn fmt(&self, node: &ScssModuleMemberList, f: &mut CssFormatter) -> FormatResult<()> {
        let separator = soft_line_break_or_space();
        let mut members = node.elements();

        let Some(first) = members.next() else {
            return Ok(());
        };

        write!(
            f,
            [
                first.node()?.format().with_text_case(CssCase::Preserve),
                first.trailing_separator()?.format()
            ]
        )?;

        for element in members {
            write!(
                f,
                [
                    &separator,
                    element.node()?.format().with_text_case(CssCase::Preserve),
                    element.trailing_separator()?.format()
                ]
            )?;
        }

        Ok(())
    }
}
