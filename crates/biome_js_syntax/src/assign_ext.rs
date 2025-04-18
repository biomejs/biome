use biome_rowan::{SyntaxResult, declare_node_union};

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
}
