use crate::prelude::*;
use biome_formatter::{format_args, write};
use biome_js_syntax::{TsTypeEmptyParameters, TsTypeEmptyParametersFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatTsTypeEmptyParameters;
impl FormatNodeRule<TsTypeEmptyParameters> for FormatTsTypeEmptyParameters {
    fn fmt_fields(&self, node: &TsTypeEmptyParameters, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeEmptyParametersFields {
            l_angle_token,
            r_angle_token,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args![
                l_angle_token.format(),
                r_angle_token.format()
            ])]
        )
    }
}
