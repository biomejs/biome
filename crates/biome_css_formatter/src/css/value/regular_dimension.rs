use crate::{prelude::*, utils::string_utils::FormatTokenAsLowercase};
use biome_css_syntax::{CssRegularDimension, CssRegularDimensionFields};
use biome_formatter::{
    token::number::{format_number_token, NumberFormatOptions},
    write,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssRegularDimension;
impl FormatNodeRule<CssRegularDimension> for FormatCssRegularDimension {
    fn fmt_fields(&self, node: &CssRegularDimension, f: &mut CssFormatter) -> FormatResult<()> {
        let CssRegularDimensionFields {
            value_token,
            unit_token,
        } = node.as_fields();

        write!(
            f,
            [
                format_number_token(&value_token?, NumberFormatOptions::default()),
                FormatTokenAsLowercase::from(unit_token?),
            ]
        )
    }
}
