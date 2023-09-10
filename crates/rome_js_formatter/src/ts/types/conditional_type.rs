use crate::prelude::*;
use crate::utils::AnyJsConditional;

use crate::parentheses::{
    is_check_type, is_in_many_type_union_or_intersection_list,
    operator_type_or_higher_needs_parens, NeedsParentheses,
};
use biome_js_syntax::{JsSyntaxKind, JsSyntaxNode, TsConditionalType};
use biome_rowan::AstNode;

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

impl NeedsParentheses for TsConditionalType {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match parent.kind() {
            JsSyntaxKind::TS_CONDITIONAL_TYPE => {
                let conditional = TsConditionalType::unwrap_cast(parent.clone());

                let is_extends_type = conditional
                    .extends_type()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(self.syntax());

                is_check_type(self.syntax(), parent) || is_extends_type
            }

            _ => {
                is_in_many_type_union_or_intersection_list(self.syntax(), parent)
                    || operator_type_or_higher_needs_parens(self.syntax(), parent)
            }
        }
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
