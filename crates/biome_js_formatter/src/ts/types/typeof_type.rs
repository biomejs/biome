use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{TsTypeofType, TsTypeofTypeFields};

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
