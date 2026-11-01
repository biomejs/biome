use biome_rowan::{SyntaxResult, declare_node_union};

use crate::static_value::StaticValue;
use crate::{AnyJsExpression, JsComputedMemberAssignment, JsStaticMemberAssignment};

declare_node_union! {
    pub AnyJsMemberAssignment = JsComputedMemberAssignment | JsStaticMemberAssignment
}

impl AnyJsMemberAssignment {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            Self::JsComputedMemberAssignment(assignment) => assignment.object(),
            Self::JsStaticMemberAssignment(assignment) => assignment.object(),
        }
    }

    /// Returns the name of the assigned member when it's known without running the code:
    /// `a.b`, `a["b"]`, and ``a[`b`]`` all return `b`.
    ///
    /// Returns `None` for private names such as `a.#b`, and for computed members
    /// that aren't a string, such as `a[b]` or `a[0]`.
    pub fn member_name(&self) -> Option<StaticValue> {
        let value = match self {
            Self::JsStaticMemberAssignment(assignment) => {
                StaticValue::String(assignment.member().ok()?.as_js_name()?.value_token().ok()?)
            }
            Self::JsComputedMemberAssignment(assignment) => {
                let member = assignment.member().ok()?.omit_parentheses();
                let result = member.as_static_value()?;
                if !matches!(result, StaticValue::String(_) | StaticValue::EmptyString(_)) {
                    return None;
                }
                result
            }
        };
        Some(value)
    }
}
