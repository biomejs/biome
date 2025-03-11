use biome_rowan::{SyntaxResult, declare_node_union};

use crate::{AnyJsExpression, JsComputedMemberAssignment, JsStaticMemberAssignment};

declare_node_union! {
    pub AnyJsMemberAssignment = JsComputedMemberAssignment | JsStaticMemberAssignment
}

impl AnyJsMemberAssignment {
    pub fn object(&self) -> SyntaxResult<AnyJsExpression> {
        match self {
            AnyJsMemberAssignment::JsComputedMemberAssignment(assignment) => assignment.object(),
            AnyJsMemberAssignment::JsStaticMemberAssignment(assignment) => assignment.object(),
        }
    }
}
