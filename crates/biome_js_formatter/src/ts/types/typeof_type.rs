use crate::prelude::*;

use crate::parentheses::NeedsParentheses;
use biome_formatter::write;
use biome_js_syntax::{
    JsSyntaxKind, JsSyntaxNode, TsIndexedAccessType, TsTypeofType, TsTypeofTypeFields,
};
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeofType;

impl FormatNodeRule<TsTypeofType> for FormatTsTypeofType {
    fn fmt_fields(&self, node: &TsTypeofType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeofTypeFields {
            type_arguments,
            typeof_token,
            expression_name,
        } = node.as_fields();

        write![
            f,
            [
                typeof_token.format(),
                space(),
                expression_name.format(),
                type_arguments.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeofType) -> bool {
        item.needs_parentheses()
    }
}

impl NeedsParentheses for TsTypeofType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::TS_ARRAY_TYPE => true,
            // Typeof operators are parenthesized when used as an object type in an indexed access
            // to avoid ambiguity of precedence, as it's higher than the JS equivalent:
            // ```typescript
            // const array = [1, 2, 3]
            // type T = typeof array[0]; // => number
            // type T2 = (typeof array)[0]; // => number
            // const J1 = typeof array[0]; // => 'number'
            // const J2 = (typeof array)[0]; // => 'o', because `typeof array` is 'object'
            // ```
            JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
                let indexed = TsIndexedAccessType::unwrap_cast(parent.clone());
                // The typeof operator only needs parens if it's the object of the indexed access.
                // If it's the index_type, then the braces already act as the visual precedence.
                indexed.object_type().map(AstNode::into_syntax).as_ref() == Ok(self.syntax())
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::{TsTypeOperatorType, TsTypeofType};

    #[test]
    fn needs_parentheses() {
        assert_not_needs_parentheses!("let s: typeof obj;", TsTypeofType);

        assert_needs_parentheses!("let s: typeof obj[number];", TsTypeofType);
        assert_needs_parentheses!("let s: keyof (typeof obj)[number];", TsTypeofType);

        // Disambiguates to `keyof ((typeof obj)[number])`, so the outer `keyof` doesn't need
        // parentheses as it's written here, but the inner `typeof obj` does, to clarify precedence.
        assert_not_needs_parentheses!("let s: keyof (typeof obj[number]);", TsTypeOperatorType);
        assert_needs_parentheses!("let s: keyof (typeof obj[number]);", TsTypeofType);

        // Forced precedence change with added parentheses, the `typeof` is no longer an indexed
        // access type, so no parentheses are needed on it directly.
        assert_not_needs_parentheses!("let s: (keyof typeof obj)[number];", TsTypeofType);
        assert_needs_parentheses!("let s: (keyof typeof obj)[number];", TsTypeOperatorType);

        assert_not_needs_parentheses!("let s: number[typeof obj];", TsTypeofType);
    }
}
