use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsTypeAssertionAssignmentFields;

use biome_js_syntax::TsTypeAssertionAssignment;

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAssertionAssignment;

impl FormatNodeRule<TsTypeAssertionAssignment> for FormatTsTypeAssertionAssignment {
    fn fmt_fields(
        &self,
        node: &TsTypeAssertionAssignment,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let TsTypeAssertionAssignmentFields {
            l_angle_token,
            ty,
            r_angle_token,
            assignment,
        } = node.as_fields();

        write![
            f,
            [
                l_angle_token.format(),
                group(&soft_block_indent(&ty.format())),
                r_angle_token.format(),
                assignment.format()
            ]
        ]
    }

    fn needs_parentheses(&self, item: &TsTypeAssertionAssignment) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsTypeAssertionAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("(<number>a) = 'test'", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>a)! = 'test'", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>(<any>a)) = 'test'", TsTypeAssertionAssignment[0]);
        assert_needs_parentheses!("(<number>(<any>a)) = 'test'", TsTypeAssertionAssignment[1]);
        assert_needs_parentheses!("++(<number>a)", TsTypeAssertionAssignment);
        assert_needs_parentheses!("(<number>a)--", TsTypeAssertionAssignment);
        assert_not_needs_parentheses!("({ a: <number>a } = { a: 5 })", TsTypeAssertionAssignment);
    }
}
