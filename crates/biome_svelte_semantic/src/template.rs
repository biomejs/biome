use biome_rowan::{Text, TextRange, TextSize};
use rustc_hash::{FxHashMap, FxHashSet};
use std::cmp::Reverse;

#[derive(Debug, PartialEq, Eq)]
struct Binding {
    declaration: TextRange,
    visibility: TextRange,
    parent: Option<usize>,
    constant: bool,
}

/// Template binding visibility ranges are nested or disjoint within each namespace.
#[derive(Default, Debug, PartialEq, Eq)]
pub struct TemplateBindings {
    values: FxHashMap<Text, Vec<Binding>>,
    types: FxHashMap<Text, Vec<Binding>>,
}

impl TemplateBindings {
    pub fn insert(
        &mut self,
        name: Text,
        declaration: TextRange,
        visibility: TextRange,
        type_only: bool,
        constant: bool,
    ) {
        let bindings = if type_only {
            &mut self.types
        } else {
            &mut self.values
        };
        bindings.entry(name).or_default().push(Binding {
            declaration,
            visibility,
            parent: None,
            constant,
        });
    }

    /// Marks the innermost bindings visible at any of the sorted positions.
    /// Returns names that shadow script declarations at every position.
    pub(super) fn possible_uses<'a>(
        &'a self,
        positions: &[TextSize],
        uses: &mut FxHashSet<TextRange>,
    ) -> [FxHashSet<&'a Text>; 2] {
        [&self.values, &self.types].map(|namespace| {
            let mut hidden = FxHashSet::default();
            for (name, bindings) in namespace {
                let mut visible = Vec::with_capacity(bindings.len());
                let mut covered = 0;
                for binding in bindings {
                    let start = positions.partition_point(|&p| p < binding.visibility.start());
                    let end = positions.partition_point(|&p| p < binding.visibility.end());
                    let count = end - start;
                    visible.push(count);
                    if let Some(parent) = binding.parent {
                        visible[parent] -= count;
                    } else {
                        covered += count;
                    }
                }
                for (binding, count) in bindings.iter().zip(visible) {
                    if count > 0 {
                        uses.insert(binding.declaration);
                    }
                }
                if covered == positions.len() {
                    hidden.insert(name);
                }
            }
            hidden
        })
    }

    pub(super) fn index(&mut self) {
        for bindings in self.values.values_mut().chain(self.types.values_mut()) {
            bindings.sort_by_key(|binding| {
                (
                    binding.visibility.start(),
                    Reverse(binding.visibility.end()),
                )
            });
            let mut parent: Option<usize> = None;
            for index in 0..bindings.len() {
                while let Some(candidate) = parent {
                    if bindings[candidate]
                        .visibility
                        .contains_range(bindings[index].visibility)
                    {
                        break;
                    }
                    parent = bindings[candidate].parent;
                }
                bindings[index].parent = parent;
                parent = Some(index);
            }
        }
    }

    pub(super) fn lookup(
        &self,
        name: &str,
        position: TextSize,
        type_only: bool,
    ) -> Option<(TextRange, TextRange, bool)> {
        let bindings = (if type_only { &self.types } else { &self.values }).get(name)?;
        let mut index = bindings
            .partition_point(|binding| binding.visibility.start() <= position)
            .checked_sub(1)?;
        loop {
            let binding = &bindings[index];
            if binding.visibility.contains(position) {
                return Some((binding.declaration, binding.visibility, binding.constant));
            }
            index = binding.parent?;
        }
    }
}
