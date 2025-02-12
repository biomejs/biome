use super::compilation_context::NodeCompilationContext;
use crate::util::TextRangeGritExt;
use biome_grit_syntax::GritVariable;
use biome_rowan::AstNode;
use camino::Utf8Path;
use grit_pattern_matcher::constants::GLOBAL_VARS_SCOPE_INDEX;
use grit_pattern_matcher::pattern::{Variable, VariableSource};
use grit_util::ByteRange;
use std::collections::BTreeSet;

pub(crate) struct VariableCompiler;

impl VariableCompiler {
    pub(crate) fn from_node(node: &GritVariable, context: &mut NodeCompilationContext) -> Variable {
        let name = node.syntax().text_trimmed().to_string();
        let range = node.range().to_byte_range();
        context.register_variable(name, range)
    }
}

impl NodeCompilationContext<'_> {
    pub(super) fn variable_from_name(&mut self, name: String) -> Variable {
        self.register_variable_with_optional_range(name, None)
    }

    pub(super) fn get_variables(
        &mut self,
        params: &[(String, ByteRange)],
    ) -> Vec<(String, Variable)> {
        params
            .iter()
            .map(|(name, range)| {
                let index = self.register_variable(name.clone(), *range);
                (name.to_owned(), index)
            })
            .collect()
    }

    pub fn register_variable(&mut self, name: String, range: ByteRange) -> Variable {
        self.register_variable_with_optional_range(
            name,
            Some(FileLocation {
                range,
                path: self.compilation.source_path,
            }),
        )
    }

    pub fn register_variable_with_optional_range(
        &mut self,
        name: String,
        location: Option<FileLocation>,
    ) -> Variable {
        let Self {
            vars,
            vars_array,
            global_vars,
            scope_index,
            ..
        } = self;

        if let Some(i) = vars.get(&name) {
            if let Some(FileLocation { range, .. }) = location {
                if let VariableSource::Compiled { locations, .. } =
                    &mut vars_array[*scope_index][*i]
                {
                    locations.insert(range);
                }
            }
            return Variable::new(*scope_index, *i);
        }

        if let Some(i) = global_vars.get(&name) {
            if let Some(FileLocation { path, range }) = location {
                if path.is_none() {
                    if let VariableSource::Compiled { locations, .. } =
                        &mut vars_array[GLOBAL_VARS_SCOPE_INDEX as usize][*i]
                    {
                        locations.insert(range);
                    }
                }
            }
            return Variable::new(GLOBAL_VARS_SCOPE_INDEX.into(), *i);
        }
        let (name_map, scope_index) = if name.starts_with("$GLOBAL_") {
            (global_vars, GLOBAL_VARS_SCOPE_INDEX.into())
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

        scope.push(VariableSource::Compiled {
            name,
            file: path.map(|p| p.to_string()).unwrap_or_default(),
            locations,
        });
        Variable::new(scope_index, index)
    }
}

pub struct FileLocation<'a> {
    path: Option<&'a Utf8Path>,
    range: ByteRange,
}
