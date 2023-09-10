use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use crate::ts::types::function_type::function_like_type_needs_parentheses;
use biome_js_syntax::TsConstructorTypeFields;
use biome_js_syntax::{JsSyntaxNode, TsConstructorType};
use rome_formatter::write;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConstructorType;

impl FormatNodeRule<TsConstructorType> for FormatTsConstructorType {
    fn fmt_fields(&self, node: &TsConstructorType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsConstructorTypeFields {
            abstract_token,
            new_token,
            type_parameters,
            parameters,
            fat_arrow_token,
            return_type,
        } = node.as_fields();

        if let Some(abstract_token) = abstract_token {
            write!(f, [abstract_token.format(), space()])?;
        }

        write![
            f,
            [
                new_token.format(),
                space(),
                type_parameters.format(),
                parameters.format(),
                space(),
                fat_arrow_token.format(),
                space(),
                return_type.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsConstructorType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsConstructorType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        function_like_type_needs_parentheses(self.syntax(), parent)
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsConstructorType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("type s = (new () => string)[]", TsConstructorType);

        assert_needs_parentheses!("type s = unique (new () => string);", TsConstructorType);

        assert_needs_parentheses!(
            "type s = [number, ...(new () => string)]",
            TsConstructorType
        );
        assert_needs_parentheses!("type s = [(new () => string)?]", TsConstructorType);

        assert_needs_parentheses!("type s = (new () => string)[a]", TsConstructorType);
        assert_not_needs_parentheses!("type s = a[new () => string]", TsConstructorType);

        assert_needs_parentheses!("type s = (new () => string) & b", TsConstructorType);
        assert_needs_parentheses!("type s = a & (new () => string)", TsConstructorType);

        // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
        // thus, no parentheses are required
        assert_not_needs_parentheses!("type s = &(new () => string)", TsConstructorType);

        assert_needs_parentheses!("type s = (new () => string) | b", TsConstructorType);
        assert_needs_parentheses!("type s = a | (new () => string)", TsConstructorType);
        assert_not_needs_parentheses!("type s = |(new () => string)", TsConstructorType);

        assert_needs_parentheses!(
            "type s = (new () => string) extends string ? string : number",
            TsConstructorType
        );
        assert_not_needs_parentheses!(
            "type s = A extends string ? (new () => string) : number",
            TsConstructorType
        );
        assert_not_needs_parentheses!(
            "type s = A extends string ? string : (new () => string)",
            TsConstructorType
        )
    }
}
