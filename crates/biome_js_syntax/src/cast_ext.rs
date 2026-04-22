use crate::{
    AnyJsExpression, AnyTsType, TsAsExpression, TsSatisfiesExpression, TsTypeAssertionExpression,
};
use biome_rowan::declare_node_union;

declare_node_union! {
    /// Any TypeScript expression that asserts its operand to a specific type:
    /// `expr as T`, `expr satisfies T`, or the legacy `<T>expr`.
    pub AnyTsCastExpression =
        TsAsExpression
        | TsSatisfiesExpression
        | TsTypeAssertionExpression
}

impl AnyTsCastExpression {
    /// Type expression the assertion casts to.
    pub fn cast_type(&self) -> Option<AnyTsType> {
        match self {
            Self::TsAsExpression(expression) => expression.ty().ok(),
            Self::TsSatisfiesExpression(expression) => expression.ty().ok(),
            Self::TsTypeAssertionExpression(expression) => expression.ty().ok(),
        }
    }

    /// Underlying expression whose type is being asserted.
    pub fn inner_expression(&self) -> Option<AnyJsExpression> {
        match self {
            Self::TsAsExpression(expression) => expression.expression().ok(),
            Self::TsSatisfiesExpression(expression) => expression.expression().ok(),
            Self::TsTypeAssertionExpression(expression) => expression.expression().ok(),
        }
    }
}
