use biome_rowan::{declare_node_union, SyntaxResult};

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
