use crate::prelude::*;
use crate::utils::FormatTypeMemberSeparator;

use biome_formatter::{format_args, write};
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{TsIntersectionType, TsIntersectionTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsIntersectionType;

impl FormatNodeRule<TsIntersectionType> for FormatTsIntersectionType {
    fn fmt_fields(&self, node: &TsIntersectionType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsIntersectionTypeFields {
            leading_separator_token,
            types,
        } = node.as_fields();
        write!(
            f,
            [group(&format_args!(
                FormatTypeMemberSeparator::new(leading_separator_token.as_ref()),
                types.format()
            ))]
        )
    }

    fn needs_parentheses(&self, item: &TsIntersectionType) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsIntersectionType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("let s: (string & number)[] = symbol();", TsIntersectionType);

        assert_needs_parentheses!("let s: unique (string & number);", TsIntersectionType);

        assert_needs_parentheses!("let s: [number, ...(string & number)]", TsIntersectionType);
        assert_needs_parentheses!("let s: [(string & number)?]", TsIntersectionType);

        assert_needs_parentheses!("let s: (string & number)[a]", TsIntersectionType);
        assert_not_needs_parentheses!("let s: a[(string & number)]", TsIntersectionType);

        assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[1]);
        assert_not_needs_parentheses!("let s: (&a) & (&b)", TsIntersectionType[2]);

        assert_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[1]);
        assert_not_needs_parentheses!("let s: (a & b) & (&c)", TsIntersectionType[2]);
    }
}
