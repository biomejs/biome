use crate::prelude::*;
use biome_formatter::write;

use crate::utils::FormatInitializerClause;

use crate::js::bindings::parameters::{
    should_hug_function_parameters, AnyJsParameters, FormatAnyJsParameters,
    FormatJsParametersOptions,
};
use biome_js_syntax::JsFormalParameterFields;
use biome_js_syntax::{JsFormalParameter, JsParameters};

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
            .and_then(JsParameters::cast)
            .map_or(false, |parameters| {
                should_hug_function_parameters(
                    &FormatAnyJsParameters::new(
                        AnyJsParameters::JsParameters(parameters),
                        FormatJsParametersOptions::default(),
                    ),
                    f.comments(),
                )
                .unwrap_or(false)
            });

        if is_hug_parameter && decorators.is_empty() {
            write![f, [decorators.format(), content]]?;
        } else if decorators.is_empty() {
            write![f, [decorators.format(), content]]?;
        } else {
            write![f, [group(&decorators.format()), group(&content)]]?;
        }

        write![f, [FormatInitializerClause::new(initializer.as_ref())]]
    }
}
