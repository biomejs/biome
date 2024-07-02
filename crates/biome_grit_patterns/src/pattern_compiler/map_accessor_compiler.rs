use super::{
    compilation_context::NodeCompilationContext, container_compiler::ContainerCompiler,
    map_compiler::MapCompiler, variable_compiler::VariableCompiler,
};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritMapAccessorSubject, AnyGritMapKey, GritMapAccessor};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Accessor, AccessorKey, AccessorMap};

pub(crate) struct MapAccessorCompiler;

impl MapAccessorCompiler {
    pub(crate) fn from_node(
        node: &GritMapAccessor,
        context: &mut NodeCompilationContext,
    ) -> Result<Accessor<GritQueryContext>, CompileError> {
        let map = match node.map()? {
            AnyGritMapAccessorSubject::AnyGritContainer(container) => {
                AccessorMap::Container(ContainerCompiler::from_node(&container, context)?)
            }
            AnyGritMapAccessorSubject::GritMap(map) => {
                AccessorMap::Map(MapCompiler::from_node(&map, context)?)
            }
        };

        let key = match node.key()? {
            AnyGritMapKey::GritName(name) => {
                AccessorKey::String(name.syntax().text_trimmed().to_string())
            }
            AnyGritMapKey::GritVariable(variable) => {
                AccessorKey::Variable(VariableCompiler::from_node(&variable, context))
            }
        };

        Ok(Accessor::new(map, key))
    }
}
