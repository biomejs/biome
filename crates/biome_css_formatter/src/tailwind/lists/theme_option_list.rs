use crate::prelude::*;
use biome_css_syntax::{AnyTwThemeOption, TwThemeOptionList};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwThemeOptionList;
impl FormatRule<TwThemeOptionList> for FormatTwThemeOptionList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &TwThemeOptionList, f: &mut CssFormatter) -> FormatResult<()> {
        for option in node {
            write!(f, [space()])?;
            match option {
                AnyTwThemeOption::CssIdentifier(identifier) => {
                    write!(f, [identifier.format().with_text_case(CssCase::Preserve)])?;
                }
                option => write!(f, [option.format()])?,
            }
        }

        Ok(())
    }
}
