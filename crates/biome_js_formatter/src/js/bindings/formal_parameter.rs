use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatInitializerClause;

use crate::js::bindings::parameters::{should_hug_function_parameters, FormatAnyJsParameters};
use biome_js_syntax::JsFormalParameter;
use biome_js_syntax::JsFormalParameterFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsFormalParameter;

impl FormatNodeRule<JsFormalParameter> for FormatJsFormalParameter {
    fn fmt_fields(&self, node: &JsFormalParameter, f: &mut JsFormatter) -> FormatResult<()> {
        let JsFormalParameterFields {
            decorators,
            binding,
            question_mark_token,
            type_annotation,
            initializer,
        } = node.as_fields();

        let content = format_with(|f| {
            write![
                f,
                [
                    binding.format(),
                    question_mark_token.format(),
                    type_annotation.format()
                ]
            ]
        });

        let is_hug_parameter = node
            .syntax()
            .grand_parent()
            .and_then(FormatAnyJsParameters::cast)
            .is_some_and(|parameters| {
                should_hug_function_parameters(&parameters, f.comments(), false).unwrap_or(false)
            });

        if is_hug_parameter && decorators.is_empty() {
            write![f, [decorators.format(), content]]?;
        } else if decorators.is_empty() {
            write![f, [decorators.format(), group(&content)]]?;
        } else {
            write![f, [group(&decorators.format()), group(&content)]]?;
        }

        write![f, [FormatInitializerClause::new(initializer.as_ref())]]
    }
}
