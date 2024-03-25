use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::JsObjectBindingPatternProperty;
use biome_js_syntax::JsObjectBindingPatternPropertyFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsObjectBindingPatternProperty;

impl FormatNodeRule<JsObjectBindingPatternProperty> for FormatJsObjectBindingPatternProperty {
    fn fmt_fields(
        &self,
        node: &JsObjectBindingPatternProperty,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsObjectBindingPatternPropertyFields {
            member,
            colon_token,
            pattern,
            init,
        } = node.as_fields();

        let group_id = f.group_id("assignment");

        write![
            f,
            [group(&format_args![
                member.format(),
                colon_token.format(),
                group(&indent(&soft_line_break_or_space())).with_group_id(Some(group_id)),
                line_suffix_boundary(),
                indent_if_group_breaks(&pattern.format(), group_id)
            ])]
        ]?;

        if let Some(init) = init {
            write!(f, [space(), init.format()])?;
        }

        Ok(())
    }
}
