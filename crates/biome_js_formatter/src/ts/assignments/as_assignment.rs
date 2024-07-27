use crate::prelude::*;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{AnyJsAssignment, AnyTsType, JsLanguage, TsAsAssignment};
use biome_rowan::SyntaxToken;

#[derive(Debug, Clone, Default)]
pub struct FormatTsAsAssignment;

impl FormatNodeRule<TsAsAssignment> for FormatTsAsAssignment {
    fn fmt_fields(&self, node: &TsAsAssignment, f: &mut JsFormatter) -> FormatResult<()> {
        format_as_or_satisfies_assignment(f, node.assignment()?, node.as_token()?, node.ty()?)
    }

    fn needs_parentheses(&self, item: &TsAsAssignment) -> bool {
        item.needs_parentheses()
    }
}

pub(crate) fn format_as_or_satisfies_assignment(
    f: &mut Formatter<JsFormatContext>,
    assignment: AnyJsAssignment,
    operation_token: SyntaxToken<JsLanguage>,
    ty: AnyTsType,
) -> FormatResult<()> {
    write![f, [assignment.format(), space(), operation_token.format()]]?;

    if f.comments().has_leading_own_line_comment(ty.syntax()) {
        write!(f, [indent(&format_args![hard_line_break(), ty.format()])])
    } else {
        write!(f, [space(), ty.format()])
    }
}

#[cfg(test)]
mod tests {

    use crate::assert_needs_parentheses;
    use biome_js_syntax::TsAsAssignment;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("a as number = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(a as number)! = 'test'", TsAsAssignment);
        assert_needs_parentheses!("(<number>(a as number)) = 'test'", TsAsAssignment);
        assert_needs_parentheses!("++(a as number)", TsAsAssignment);
        assert_needs_parentheses!("(a as number)--", TsAsAssignment);
        assert_needs_parentheses!("({ a: a as number } = { a: 5 })", TsAsAssignment);
    }
}
