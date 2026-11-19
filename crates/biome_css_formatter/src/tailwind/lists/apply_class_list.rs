use crate::prelude::*;
use biome_css_syntax::TwApplyClassList;
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTwApplyClassList;
impl FormatRule<TwApplyClassList> for FormatTwApplyClassList {
    type Context = CssFormatContext;
    fn fmt(&self, node: &TwApplyClassList, f: &mut CssFormatter) -> FormatResult<()> {
        let mut classes = node.iter();

        let Some(first) = classes.next() else {
            return Ok(());
        };

        write!(f, [first.format().with_text_case(CssCase::Preserve)])?;

        for class in classes {
            write!(
                f,
                [space(), class.format().with_text_case(CssCase::Preserve)]
            )?;
        }

        Ok(())
    }
}
