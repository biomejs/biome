use crate::{css::auxiliary::identifier::FormatCssIdentifierOptions, prelude::*};
use biome_css_syntax::{CssRegularDimension, CssRegularDimensionFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields { value, unit } = node.as_fields();

        write!(f, [value.format()])?;

        if let Ok(unit) = unit {
            write!(
                f,
                [unit.format().with_options(FormatCssIdentifierOptions {
                    forced_lowercase: true
                })]
            )?;
        }

        Ok(())
    }
}
