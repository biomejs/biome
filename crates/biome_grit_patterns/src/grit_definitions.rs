use std::collections::BTreeMap;

use crate::{
    grit_context::GritQueryContext,
    pattern_compiler::{
        compilation_context::{DefinitionInfo, NodeCompilationContext},
        FunctionDefinitionCompiler, PatternDefinitionCompiler, PredicateDefinitionCompiler,
    },
    util::TextRangeGritExt,
    CompileError,
};
use biome_grit_syntax::{AnyGritDefinition, GritDefinitionList, GritVariableList};
use biome_rowan::AstNode;
use grit_pattern_matcher::pattern::{
    GritFunctionDefinition, PatternDefinition, PredicateDefinition,
};
use grit_util::ByteRange;

#[derive(Clone, Debug)]
pub struct Definitions {
    pub patterns: Vec<PatternDefinition<GritQueryContext>>,
    pub predicates: Vec<PredicateDefinition<GritQueryContext>>,
    pub functions: Vec<GritFunctionDefinition<GritQueryContext>>,
}

/// Compiles all definitions.
///
/// Must be called after [scan_definitions()].
pub fn compile_definitions(
    definitions: GritDefinitionList,
    context: &mut NodeCompilationContext,
) -> Result<Definitions, CompileError> {
    let mut patterns = Vec::new();
    let mut predicates = Vec::new();
    let mut functions = Vec::new();
    for definition in definitions {
        match definition? {
            AnyGritDefinition::AnyGritPattern(_) => continue, // Handled separately.
            AnyGritDefinition::GritPatternDefinition(node) => {
                patterns.push(PatternDefinitionCompiler::from_node(node, context)?);
            }
            AnyGritDefinition::GritPredicateDefinition(node) => {
                predicates.push(PredicateDefinitionCompiler::from_node(node, context)?);
            }
            AnyGritDefinition::GritFunctionDefinition(node) => {
                functions.push(FunctionDefinitionCompiler::from_node(node, context)?);
            }
            AnyGritDefinition::GritJavascriptFunctionDefinition(_)
            | AnyGritDefinition::GritBogusDefinition(_) => {
                unreachable!(); // Should be handled in `scan_definitions()`.
            }
        }
    }

    Ok(Definitions {
        patterns,
        predicates,
        functions,
    })
}

pub struct ScannedDefinitionInfo {
    pub pattern_definition_info: BTreeMap<String, DefinitionInfo>,
    pub predicate_definition_info: BTreeMap<String, DefinitionInfo>,
    pub function_definition_info: BTreeMap<String, DefinitionInfo>,
}

/// Finds all definitions so that we can allocate their scopes in preparation
/// for the compilation phase.
pub fn scan_definitions(
    definitions: GritDefinitionList,
) -> Result<ScannedDefinitionInfo, CompileError> {
    let mut pattern_definition_info = BTreeMap::new();
    let mut pattern_index = 0;

    let mut predicate_definition_info = BTreeMap::new();
    let mut predicate_index = 0;

    let mut function_definition_info = BTreeMap::new();
    let mut function_index = 0;

    for definition in definitions {
        match definition? {
            AnyGritDefinition::AnyGritPattern(_) => continue, // Handled separately.
            AnyGritDefinition::GritPatternDefinition(node) => {
                let name = node.name()?.to_trimmed_string();
                let name = name.trim();
                if pattern_definition_info.contains_key(name) {
                    return Err(CompileError::DuplicatePatternDefinition(name.to_owned()));
                }

                pattern_definition_info.insert(
                    name.to_owned(),
                    DefinitionInfo {
                        index: pattern_index,
                        parameters: collect_variables(node.args())?,
                    },
                );

                pattern_index += 1;
            }
            AnyGritDefinition::GritPredicateDefinition(node) => {
                let name = node.name()?.to_trimmed_string();
                let name = name.trim();
                if predicate_definition_info.contains_key(name) {
                    return Err(CompileError::DuplicatePredicateDefinition(name.to_owned()));
                }

                predicate_definition_info.insert(
                    name.to_owned(),
                    DefinitionInfo {
                        index: predicate_index,
                        parameters: collect_variables(node.args())?,
                    },
                );

                predicate_index += 1;
            }
            AnyGritDefinition::GritFunctionDefinition(node) => {
                let name = node.name()?.to_trimmed_string();
                let name = name.trim();
                if function_definition_info.contains_key(name) {
                    return Err(CompileError::DuplicateFunctionDefinition(name.to_owned()));
                }

                function_definition_info.insert(
                    name.to_owned(),
                    DefinitionInfo {
                        index: function_index,
                        parameters: collect_variables(node.args())?,
                    },
                );

                function_index += 1;
            }
            AnyGritDefinition::GritJavascriptFunctionDefinition(func) => {
                return Err(CompileError::UnsupportedFunctionDefinition(
                    func.name()?.to_trimmed_string().trim().to_owned(),
                ));
            }
            AnyGritDefinition::GritBogusDefinition(bogus) => {
                return Err(CompileError::UnexpectedKind(
                    bogus
                        .items()
                        .next()
                        .map(|item| item.kind().into())
                        .unwrap_or_default(),
                ));
            }
        }
    }

    Ok(ScannedDefinitionInfo {
        pattern_definition_info,
        predicate_definition_info,
        function_definition_info,
    })
}

fn collect_variables(
    variables: GritVariableList,
) -> Result<Vec<(String, ByteRange)>, CompileError> {
    variables
        .into_iter()
        .map(|var| {
            let token = var?.value_token()?;
            Ok((
                token.text_trimmed().to_string(),
                token.text_trimmed_range().to_byte_range(),
            ))
        })
        .collect()
}
