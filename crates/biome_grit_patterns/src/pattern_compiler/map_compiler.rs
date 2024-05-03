use super::{compilation_context::NodeCompilationContext, PatternCompiler};
use crate::{grit_context::GritQueryContext, CompileError};
use biome_grit_syntax::{AnyGritMapElement, GritMap, GritSyntaxKind};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{GritMap as Map, Pattern};
use std::collections::BTreeMap;

pub(crate) struct MapCompiler;

impl MapCompiler {
    pub(crate) fn from_node(
        node: &GritMap,
        context: &mut NodeCompilationContext,
    ) -> Result<Map<GritQueryContext>, CompileError> {
        Self::from_node_with_rhs(node, context, false)
    }

    pub(crate) fn from_node_with_rhs(
        node: &GritMap,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Map<GritQueryContext>, CompileError> {
        let elements = node
            .elements()
            .into_iter()
            .map(|element| compile_map_element(&element?, context, is_rhs))
            .collect::<Result<BTreeMap<_, _>, CompileError>>()?;
        Ok(Map::new(elements))
    }
}

fn compile_map_element(
    node: &AnyGritMapElement,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<(String, Pattern<GritQueryContext>), CompileError> {
    match node {
        AnyGritMapElement::GritMapElement(element) => {
            let key = element.key()?.syntax().text_trimmed().to_string();
            let pattern = PatternCompiler::from_node_with_rhs(&element.value()?, context, is_rhs)?;
            Ok((key, pattern))
        }
        AnyGritMapElement::GritBogusMapElement(_) => Err(CompileError::UnexpectedKind(
            GritSyntaxKind::GRIT_BOGUS_MAP_ELEMENT.into(),
        )),
    }
}
