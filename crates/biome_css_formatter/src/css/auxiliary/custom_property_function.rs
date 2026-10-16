use crate::prelude::*;
use crate::utils::custom_property::CustomPropertyContainer;
use crate::verbatim::format_css_verbatim_fields;
use biome_css_syntax::{
    AnyCssCustomPropertyComponent, CssCustomPropertyFunction, CssCustomPropertyFunctionFields,
    CssFunctionParameterDefaultValue, decode_css_identifier,
};
use biome_formatter::write;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssCustomPropertyFunction;
impl FormatNodeRule<CssCustomPropertyFunction> for FormatCssCustomPropertyFunction {
    fn fmt_fields(
        &self,
        node: &CssCustomPropertyFunction,
        f: &mut CssFormatter,
    ) -> FormatResult<()> {
        let CssCustomPropertyFunctionFields {
            name,
            l_paren_token: _,
            components,
            r_paren_token: _,
        } = node.as_fields();
        let name = name?;

        // Native defaults keep unquoted URL commas and comment-shaped text as
        // payload, including trivia that ordinary component formatting moves.
        if decode_css_identifier(name.value_token()?.text_trimmed()).eq_ignore_ascii_case("url")
            && !matches!(
                components.first(),
                Some(
                    AnyCssCustomPropertyComponent::CssString(_)
                        | AnyCssCustomPropertyComponent::ScssInterpolatedString(_)
                )
            )
            && node
                .syntax()
                .ancestors()
                .any(|ancestor| CssFunctionParameterDefaultValue::can_cast(ancestor.kind()))
        {
            return format_css_verbatim_fields(node.syntax()).fmt(f);
        }

        write!(
            f,
            [name.format(), CustomPropertyContainer::from(node.clone())]
        )
    }
}
