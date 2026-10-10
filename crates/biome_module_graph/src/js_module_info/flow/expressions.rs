use biome_js_syntax::{
    AnyJsCallArgument, JsAwaitExpression, JsBinaryExpression, JsBinaryOperator, JsCallArgumentList,
    JsCallArguments, JsCallExpression, JsComputedMemberExpression, JsConditionalExpression,
    JsIdentifierExpression, JsLogicalExpression, JsParenthesizedExpression, JsSequenceExpression,
    JsStaticMemberExpression,
};
use biome_rowan::{AstNode, AstSeparatedList, declare_node_union};

declare_node_union! {
    /// Expressions whose type at a source occurrence can follow a refined
    /// operand.
    ///
    /// Flow evaluation refines exactly the members for which
    /// [`Self::is_evaluated`] returns true; every other expression keeps its
    /// raw type.
    pub(crate) AnyFlowExpression = JsIdentifierExpression
        | JsParenthesizedExpression
        | JsStaticMemberExpression
        | JsComputedMemberExpression
        | JsLogicalExpression
        | JsConditionalExpression
        | JsCallExpression
        | JsAwaitExpression
        | JsSequenceExpression
        | JsBinaryExpression
}

impl AnyFlowExpression {
    /// Returns whether flow evaluation supports this expression's shape.
    ///
    /// Among binary expressions only addition is evaluated, and calls are
    /// evaluated only without explicit type arguments or spread arguments.
    pub(crate) fn is_evaluated(&self) -> bool {
        match self {
            Self::JsBinaryExpression(binary) => {
                binary.operator().ok() == Some(JsBinaryOperator::Plus)
            }
            Self::JsCallExpression(call) => {
                call.type_arguments().is_none()
                    && call.arguments().is_ok_and(|arguments| {
                        arguments.args().iter().all(|argument| {
                            matches!(argument, Ok(AnyJsCallArgument::AnyJsExpression(_)))
                        })
                    })
            }
            Self::JsIdentifierExpression(_)
            | Self::JsParenthesizedExpression(_)
            | Self::JsStaticMemberExpression(_)
            | Self::JsComputedMemberExpression(_)
            | Self::JsLogicalExpression(_)
            | Self::JsConditionalExpression(_)
            | Self::JsAwaitExpression(_)
            | Self::JsSequenceExpression(_) => true,
        }
    }

    /// Returns the enclosing expressions whose evaluation queries this one,
    /// directly or through other operands, innermost first.
    ///
    /// A refined type can only propagate along this chain, and its length
    /// bounds the operand recursion that can reach this expression.
    pub(crate) fn evaluated_ancestors(&self) -> impl Iterator<Item = Self> + use<> {
        std::iter::successors(self.evaluated_parent(), Self::evaluated_parent)
    }

    /// Returns the parent expression whose evaluation queries this one as an
    /// operand.
    ///
    /// An operand is a direct child or a call argument. A sequence reads only
    /// its right side; every other evaluated expression reads all of its operands.
    fn evaluated_parent(&self) -> Option<Self> {
        let mut parent = self.syntax().parent()?;
        while JsCallArguments::can_cast(parent.kind())
            || JsCallArgumentList::can_cast(parent.kind())
        {
            parent = parent.parent()?;
        }
        let parent = Self::cast(parent)?;
        let reads_operand = match &parent {
            Self::JsSequenceExpression(sequence) => sequence
                .right()
                .is_ok_and(|right| right.syntax() == self.syntax()),
            Self::JsIdentifierExpression(_)
            | Self::JsParenthesizedExpression(_)
            | Self::JsStaticMemberExpression(_)
            | Self::JsComputedMemberExpression(_)
            | Self::JsLogicalExpression(_)
            | Self::JsConditionalExpression(_)
            | Self::JsCallExpression(_)
            | Self::JsAwaitExpression(_)
            | Self::JsBinaryExpression(_) => true,
        };
        (reads_operand && parent.is_evaluated()).then_some(parent)
    }
}
