use crate::prelude::*;

use biome_formatter::write;
use biome_js_syntax::parentheses::NeedsParentheses;
use biome_js_syntax::{TsInferType, TsInferTypeFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInferType;

impl FormatNodeRule<TsInferType> for FormatTsInferType {
    fn fmt_fields(&self, node: &TsInferType, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInferTypeFields {
            infer_token,
            name,
            constraint,
        } = node.as_fields();

        write!(f, [infer_token.format(), space(), name.format()])?;

        if let Some(constraint) = constraint {
            write!(f, [space(), constraint.format()])?;
        }

        Ok(())
    }

    fn needs_parentheses(&self, item: &TsInferType) -> bool {
        item.needs_parentheses()
    }
}

#[cfg(test)]
mod tests {

    use crate::{assert_needs_parentheses, assert_not_needs_parentheses};
    use biome_js_syntax::TsInferType;

    #[test]
    fn needs_parentheses() {
        assert_needs_parentheses!(
            "type A<T> = T extends (infer string)[] ? string : never",
            TsInferType
        );
        assert_needs_parentheses!(
            "type A<T> = T extends unique (infer string) ? string : never",
            TsInferType
        );

        assert_not_needs_parentheses!(
            "type A<T> = T extends [number, ...infer string] ? string : never",
            TsInferType
        );
        assert_needs_parentheses!(
            "type A = T extends [(infer string)?] ? string : never",
            TsInferType
        );
        assert_needs_parentheses!(
            "type A<T> = [T] extends [(infer S extends string) | undefined] ? S : T",
            TsInferType
        );

        assert_needs_parentheses!(
            "type A<T> = T extends (infer string)[a] ? string : never",
            TsInferType
        );
        assert_not_needs_parentheses!(
            "type A<T> = T extends a[(infer string)] ? string : never",
            TsInferType
        );
        assert_not_needs_parentheses!(
            "type A = T extends () => infer R | B ? R : never",
            TsInferType
        );
    }
}
