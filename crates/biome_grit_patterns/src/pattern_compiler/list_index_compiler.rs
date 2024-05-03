use super::{
    compilation_context::NodeCompilationContext, container_compiler::ContainerCompiler,
    list_compiler::ListCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritListAccessorSubject, AnyGritListIndex, GritListAccessor};
use grit_pattern_matcher::pattern::{ContainerOrIndex, ListIndex, ListOrContainer};

pub(crate) struct ListIndexCompiler;

impl ListIndexCompiler {
    pub(crate) fn from_node(
        node: &GritListAccessor,
        context: &mut NodeCompilationContext,
    ) -> Result<ListIndex<GritQueryContext>, CompileError> {
        let list = match node.list()? {
            AnyGritListAccessorSubject::AnyGritContainer(container) => {
                ListOrContainer::Container(ContainerCompiler::from_node(&container, context)?)
            }
            AnyGritListAccessorSubject::GritList(list) => {
                ListOrContainer::List(ListCompiler::from_node(&list, context)?)
            }
        };

        let index = match node.index()? {
            AnyGritListIndex::AnyGritContainer(container) => {
                ContainerOrIndex::Container(ContainerCompiler::from_node(&container, context)?)
            }
            AnyGritListIndex::GritIntLiteral(int) => ContainerOrIndex::Index(
                int.value_token()?.text_trimmed().parse().map_err(|err| {
                    CompileError::LiteralOutOfRange(format!("Error parsing list index: {err}"))
                })?,
            ),
            AnyGritListIndex::GritNegativeIntLiteral(int) => ContainerOrIndex::Index(
                int.value_token()?.text_trimmed().parse().map_err(|err| {
                    CompileError::LiteralOutOfRange(format!("Error parsing list index: {err}"))
                })?,
            ),
        };

        Ok(ListIndex { list, index })
    }
}
