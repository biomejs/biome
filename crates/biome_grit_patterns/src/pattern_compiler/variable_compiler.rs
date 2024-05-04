use super::compilation_context::NodeCompilationContext;
use crate::util::TextRangeGritExt;
use crate::CompileError;
use biome_grit_syntax::GritVariable;
use biome_rowan::AstNode;
use grit_pattern_matcher::constants::GLOBAL_VARS_SCOPE_INDEX;
use grit_pattern_matcher::pattern::{Variable, VariableSourceLocations};
use grit_util::ByteRange;
use std::collections::BTreeSet;
use std::path::Path;

pub(crate) struct VariableCompiler;

impl VariableCompiler {
    pub(crate) fn from_node(
        node: &GritVariable,
        context: &mut NodeCompilationContext,
    ) -> Result<Variable, CompileError> {
        let name = node.syntax().text_trimmed().to_string();
        let range = node.range().to_byte_range();
        context.register_variable(name, range)
    }
}

impl<'a> NodeCompilationContext<'a> {
    pub(super) fn variable_from_name(&mut self, name: String) -> Result<Variable, CompileError> {
        self.register_variable_with_optional_range(name, None)
    }

    pub(super) fn get_variables(
        &mut self,
        params: &[(String, ByteRange)],
    ) -> Result<Vec<(String, Variable)>, CompileError> {
        params
            .iter()
            .map(|(name, range)| {
                let index = self.register_variable(name.clone(), *range)?;
                Ok((name.to_owned(), index))
            })
            .collect()
    }

    pub(super) fn register_variable(
        &mut self,
        name: String,
        range: ByteRange,
    ) -> Result<Variable, CompileError> {
        self.register_variable_with_optional_range(
            name,
            Some(FileLocation {
                range,
                path: self.compilation.source_path,
            }),
        )
    }

    fn register_variable_with_optional_range(
        &mut self,
        name: String,
        location: Option<FileLocation>,
    ) -> Result<Variable, CompileError> {
        let Self {
            vars,
            vars_array,
            global_vars,
            scope_index,
            ..
        } = self;

        if let Some(i) = vars.get(&name) {
            if let Some(FileLocation { range, .. }) = location {
                vars_array[*scope_index][*i].locations.insert(range);
            }
            return Ok(Variable::new(*scope_index, *i));
        }

        if let Some(i) = global_vars.get(&name) {
            if let Some(FileLocation { path, range }) = location {
                if path.is_none() {
                    vars_array[GLOBAL_VARS_SCOPE_INDEX][*i]
                        .locations
                        .insert(range);
                }
            }
            return Ok(Variable::new(GLOBAL_VARS_SCOPE_INDEX, *i));
        }
        let (name_map, scope_index) = if name.starts_with("$GLOBAL_") {
            (global_vars, GLOBAL_VARS_SCOPE_INDEX)
        } else {
            (vars, *scope_index)
        };
        let scope = &mut vars_array[scope_index];
        let index = scope.len();
        name_map.insert(name.clone(), index);

        let (locations, path) = if let Some(FileLocation { path, range }) = location {
            (BTreeSet::from([range]), path)
        } else {
            // this currently only comes up with the $match variable which we autowrap, and is not
            // usually used by the user, but feels like this could potentially be a source of bugs
            (BTreeSet::new(), None)
        };

        scope.push(VariableSourceLocations {
            name,
            file: path
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_default(),
            locations,
        });
        Ok(Variable::new(scope_index, index))
    }
}

struct FileLocation<'a> {
    path: Option<&'a Path>,
    range: ByteRange,
}
