use crate::html::auxiliary::text_expression::FormatHtmlTextExpressionOptions;
use crate::prelude::*;
use crate::vue::value::v_for_value::keeps_source_quotes;
use biome_formatter::write;
use biome_html_syntax::{VueVForDefaultBinding, VueVForDefaultBindingFields, VueVForValue};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatVueVForDefaultBinding;
impl FormatNodeRule<VueVForDefaultBinding> for FormatVueVForDefaultBinding {
    fn fmt_fields(&self, node: &VueVForDefaultBinding, f: &mut HtmlFormatter) -> FormatResult<()> {
        let VueVForDefaultBindingFields {
            binding,
            eq_token,
            default,
        } = node.as_fields();
        let default = default?;
        let escape_double_quotes = node
            .syntax()
            .ancestors()
            .find_map(VueVForValue::cast)
            .is_some_and(|value| !keeps_source_quotes(&value, f));

        write!(
            f,
            [
                binding.format(),
                space(),
                eq_token.format(),
                space(),
                default
                    .format()
                    .with_options(FormatHtmlTextExpressionOptions {
                        escape_double_quotes,
                        ..Default::default()
                    })
            ]
        )
    }
}
