use crate::prelude::*;
use crate::utils::AnyJsConditional;

use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::TsConditionalType;

#[derive(Debug, Clone, Default)]
pub struct FormatTsConditionalType;

impl FormatNodeRule<TsConditionalType> for FormatTsConditionalType {
    fn fmt_fields(
        &self,
        node: &TsConditionalType,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        AnyJsConditional::from(node.clone()).format().fmt(formatter)
    }

    fn needs_parentheses(&self, item: &TsConditionalType) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {
    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsConditionalType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!("type s = (A extends B ? C : D)[]", TsConditionalType);

        assert_needs_parentheses!("type s = unique (A extends B ? C : D);", TsConditionalType);

        assert_needs_parentheses!(
            "type s = [number, ...(A extends B ? C : D)]",
            TsConditionalType
        );
        assert_needs_parentheses!("type s = [(A extends B ? C : D)?]", TsConditionalType);

        assert_needs_parentheses!("type s = (A extends B ? C : D)[a]", TsConditionalType);
        assert_not_needs_parentheses!("type s = a[A extends B ? C : D]", TsConditionalType);

        assert_needs_parentheses!("type s = (A extends B ? C : D) & b", TsConditionalType);
        assert_needs_parentheses!("type s = a & (A extends B ? C : D)", TsConditionalType);

        // This does require parentheses but the formatter will strip the leading `&`, leaving only the inner type
        // thus, no parentheses are required
        assert_not_needs_parentheses!("type s = &(A extends B ? C : D)", TsConditionalType);

        assert_needs_parentheses!("type s = (A extends B ? C : D) | b", TsConditionalType);
        assert_needs_parentheses!("type s = a | (A extends B ? C : D)", TsConditionalType);
        assert_not_needs_parentheses!("type s = |(A extends B ? C : D)", TsConditionalType);

        assert_needs_parentheses!(
            "type s = (A extends B ? C : D) extends E ? F : G",
            TsConditionalType[1]
        );
        assert_not_needs_parentheses!(
            "type s = (A extends B ? C : D) extends E ? F : G",
            TsConditionalType[0]
        );

        assert_needs_parentheses!(
            "type s = A extends (B extends C ? D : E) ? F : G",
            TsConditionalType[1]
        );
        assert_not_needs_parentheses!(
            "type s = A extends (B extends C ? D : E) ? F : G",
            TsConditionalType[0]
        );

        assert_not_needs_parentheses!(
            "type s = A extends B ? (C extends D ? E : F) : G",
            TsConditionalType[0]
        );
        assert_not_needs_parentheses!(
            "type s = A extends B ? (C extends D ? E : F) : G",
            TsConditionalType[1]
        );

        assert_not_needs_parentheses!(
            "type s = A extends B ? C : (D extends E ? F : G)",
            TsConditionalType[0]
        );
        assert_not_needs_parentheses!(
            "type s = A extends B ? C : (D extends E ? F : G)",
            TsConditionalType[1]
        );
    }
}
