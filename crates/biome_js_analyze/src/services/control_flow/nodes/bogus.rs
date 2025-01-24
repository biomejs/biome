use biome_js_syntax::AnyJsBogusNode;
use biome_rowan::{SyntaxError, SyntaxResult};

use crate::services::control_flow::{
    visitor::{NodeVisitor, StatementStack},
    FunctionBuilder,
};

/// Bogus visitor.
///
/// The bogus visitor merely acts to abort control flow analysis inside broken
/// code, which could otherwise mess with assumptions made inside other
/// visitors.
pub(in crate::services::control_flow) struct BogusVisitor;

impl NodeVisitor for BogusVisitor {
    type Node = AnyJsBogusNode;

    fn enter(_: Self::Node, _: &mut FunctionBuilder, _: StatementStack) -> SyntaxResult<Self> {
        Err(SyntaxError::UnexpectedBogusNode)
    }
}
