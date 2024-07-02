use super::{
    compilation_context::NodeCompilationContext, list_index_compiler::ListIndexCompiler,
    map_accessor_compiler::MapAccessorCompiler, variable_compiler::VariableCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritContainer, GritSyntaxKind};
use grit_pattern_matcher::pattern::Container;

pub(crate) struct ContainerCompiler;

impl ContainerCompiler {
    pub(crate) fn from_node(
        node: &AnyGritContainer,
        context: &mut NodeCompilationContext,
    ) -> Result<Container<GritQueryContext>, CompileError> {
        match node {
            AnyGritContainer::GritListAccessor(accessor) => Ok(Container::ListIndex(Box::new(
                ListIndexCompiler::from_node(accessor, context)?,
            ))),
            AnyGritContainer::GritMapAccessor(accessor) => Ok(Container::Accessor(Box::new(
                MapAccessorCompiler::from_node(accessor, context)?,
            ))),
            AnyGritContainer::GritVariable(variable) => Ok(Container::Variable(
                VariableCompiler::from_node(variable, context),
            )),
            AnyGritContainer::GritBogusContainer(_) => Err(CompileError::UnexpectedKind(
                GritSyntaxKind::GRIT_BOGUS_CONTAINER.into(),
            )),
        }
    }
}
