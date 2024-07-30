use crate::js::declarations::function_declaration::should_group_function_parameters;
use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsFunctionType;
use biome_js_syntax::TsFunctionTypeFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsFunctionType;

impl FormatNodeRule<TsFunctionType> for FormatTsFunctionType {
    fn fmt_fields(&self, node: &TsFunctionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsFunctionTypeFields {
            parameters,
            fat_arrow_token,
            type_parameters,
            return_type,
        } = node.as_fields();

        let format_inner = format_with(|f| {
            write![f, [type_parameters.format()]]?;

            let mut format_return_type = return_type.format().memoized();
            let should_group_parameters = should_group_function_parameters(
                type_parameters.as_ref(),
                parameters.as_ref()?.items().len(),
                Some(return_type.clone()),
                &mut format_return_type,
                f,
            )?;

            if should_group_parameters {
                write!(f, [group(&parameters.format())])?;
            } else {
                write!(f, [parameters.format()])?;
            }

            write![
                f,
                [
                    space(),
                    fat_arrow_token.format(),
                    space(),
                    format_return_type
                ]
            ]
        });

        write!(f, [group(&format_inner)])
    }

    fn needs_parentheses(&self, item: &TsFunctionType) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsFunctionType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("type s = (() => string)[]", TsFunctionType);

        assert_needs_parentheses!("type s = unique (() => string);", TsFunctionType);

        assert_needs_parentheses!("type s = [number, ...(() => string)]", TsFunctionType);
        assert_needs_parentheses!("type s = [(() => string)?]", TsFunctionType);

        assert_needs_parentheses!("type s = (() => string)[a]", TsFunctionType);
        assert_not_needs_parentheses!("type s = a[() => string]", TsFunctionType);

        assert_needs_parentheses!("type s = (() => string) & b", TsFunctionType);
        assert_needs_parentheses!("type s = a & (() => string)", TsFunctionType);

        // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
        // thus, no parentheses are required
        assert_not_needs_parentheses!("type s = &(() => string)", TsFunctionType);

        assert_needs_parentheses!("type s = (() => string) | b", TsFunctionType);
        assert_needs_parentheses!("type s = a | (() => string)", TsFunctionType);
        assert_not_needs_parentheses!("type s = |(() => string)", TsFunctionType);

        assert_needs_parentheses!(
            "type s = (() => string) extends string ? string : number",
            TsFunctionType
        );
        assert_not_needs_parentheses!(
            "type s = A extends string ? (() => string) : number",
            TsFunctionType
        );
        assert_not_needs_parentheses!(
            "type s = A extends string ? string : (() => string)",
            TsFunctionType
        )
    }
}
