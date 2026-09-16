use crate::prelude::*;
use crate::utils::case::query_feature_name_case;
use biome_css_syntax::{CssQueryFeaturePlain, CssQueryFeaturePlainFields};
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssQueryFeaturePlain;
impl FormatNodeRule<CssQueryFeaturePlain> for FormatCssQueryFeaturePlain {
    fn fmt_fields(&self, node: &CssQueryFeaturePlain, f: &mut CssFormatter) -> FormatResult<()> {
        let CssQueryFeaturePlainFields {
            name,
            colon_token,
            value,
        } = node.as_fields();
        let name = name?;
        let case = query_feature_name_case(&name);

        write!(
            f,
            [
                name.format().with_text_case(case),
                colon_token.format(),
                space(),
                value?.format().with_text_case(CssCase::Preserve)
            ]
        )
    }
}
