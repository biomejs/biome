use super::compilation_context::CompilationContext;
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{GritMapAccessor, GritSyntaxKind};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{Accessor, AccessorKey, AccessorMap};

pub(crate) struct MapAccessorCompiler;

impl MapAccessorCompiler {
    pub(crate) fn from_node(
        node: &GritMapAccessor,
        context: &mut CompilationContext,
    ) -> Result<Accessor<GritQueryContext>, CompileError> {
        let map = node.map()?;
        let map = if map.syntax().kind() == GritSyntaxKind::GRIT_MAP {
            AccessorMap::Map(MapCompiler::from_node(&map, context)?)
        } else {
            AccessorMap::Container(ContainerCompiler::from_node(&map, context)?)
        };

        let key = node.key()?;
        let key = if key.syntax().kind() == GritSyntaxKind::GRIT_VARIABLE {
            AccessorKey::Variable(VariableCompiler::from_node(&key, context)?)
        } else {
            AccessorKey::String(key.syntax().text_trimmed().to_string())
        };

        Ok(Accessor::new(map, key))
    }
}
