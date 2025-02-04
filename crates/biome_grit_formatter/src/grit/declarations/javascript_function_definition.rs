use crate::prelude::*;
use biome_formatter::write;
use biome_grit_syntax::{GritJavascriptFunctionDefinition, GritJavascriptFunctionDefinitionFields};
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatGritJavascriptFunctionDefinition;
impl FormatNodeRule<GritJavascriptFunctionDefinition> for FormatGritJavascriptFunctionDefinition {
    fn fmt_fields(
        &self,
        node: &GritJavascriptFunctionDefinition,
        f: &mut GritFormatter,
    ) -> FormatResult<()> {
        let GritJavascriptFunctionDefinitionFields {
            function_token,
            name,
            l_paren_token,
            args,
            r_paren_token,
            js_token,
            grit_javascript_body_wrapper,
        } = node.as_fields();

        write!(
            f,
            [
                function_token.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                group(&args.format()),
                r_paren_token.format(),
                space(),
                js_token.format(),
                space(),
                grit_javascript_body_wrapper.format(),
            ]
        )
    }
}
