use biome_css_syntax::{CssFunctionParameter, CssFunctionParameterFields};
use biome_formatter::write;

use crate::prelude::*;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssFunctionParameter;

impl FormatNodeRule<CssFunctionParameter> for FormatCssFunctionParameter {
    fn fmt_fields(&self, node: &CssFunctionParameter, f: &mut CssFormatter) -> FormatResult<()> {
        let CssFunctionParameterFields {
            name,
            ty,
            default_value,
        } = node.as_fields();

        write!(
            f,
            [&format_with(|f| {
                write!(f, [name.format()])?;

                if let Some(ty) = &ty {
                    write!(f, [space(), ty.format()])?;
                }

                write!(f, [default_value.format()])
            })]
        )
    }
}
