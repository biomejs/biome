//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_grit_syntax::AnyGritDefinition;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyGritDefinition;
impl FormatRule<AnyGritDefinition> for FormatAnyGritDefinition {
    type Context = GritFormatContext;
    fn fmt(&self, node: &AnyGritDefinition, f: &mut GritFormatter) -> FormatResult<()> {
        match node {
            AnyGritDefinition::AnyGritPattern(node) => node.format().fmt(f),
            AnyGritDefinition::GritBogusDefinition(node) => node.format().fmt(f),
            AnyGritDefinition::GritFunctionDefinition(node) => node.format().fmt(f),
            AnyGritDefinition::GritJavascriptFunctionDefinition(node) => node.format().fmt(f),
            AnyGritDefinition::GritPatternDefinition(node) => node.format().fmt(f),
            AnyGritDefinition::GritPredicateDefinition(node) => node.format().fmt(f),
        }
    }
}
