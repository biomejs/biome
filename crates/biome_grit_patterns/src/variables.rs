use crate::grit_context::GritQueryContext;
use grit_pattern_matcher::pattern::{VariableContent, VariableSource};
use grit_util::VariableBinding;

/// List of all variable locations in a query.
///
/// Variables are stored in a vector of vectors, where the outer vector is used
/// to separate scopes, while the inner vector contains the variables. For each
/// variable, we track the separate locations (plural) where that variable
/// occurs.
#[derive(Clone, Debug, Default)]
pub struct VariableLocations(Vec<Vec<VariableSource>>);

impl VariableLocations {
    pub(crate) fn new(locations: Vec<Vec<VariableSource>>) -> Self {
        Self(locations)
    }

    #[expect(dead_code)]
    pub(crate) fn compiled_vars(&self) -> Vec<VariableBinding> {
        let mut variables = Vec::new();
        for (i, scope) in self.0.iter().enumerate() {
            for (j, var) in scope.iter().enumerate() {
                if let VariableSource::Compiled {
                    name, locations, ..
                } = var
                {
                    variables.push(VariableBinding {
                        name: name.to_owned(),
                        scoped_name: format!("{i}_{j}_{name}"),
                        ranges: locations.iter().copied().collect(),
                    });
                }
            }
        }
        variables
    }
}

/// Registry containing all variables.
///
/// Variables are stored here in a three-dimensional vector, where the outer
/// vector is used to separate scopes, the second vector is used to
/// differentiate scope instances across calls, and the inner vector contains
/// the variable contents.
pub(crate) struct VarRegistry<'a>(VarRegistryVector<'a>);

impl VarRegistry<'_> {
    pub(crate) fn from_locations(locations: &VariableLocations) -> Self {
        let vector = locations
            .0
            .iter()
            .map(|scope| {
                vec![scope
                    .iter()
                    .map(|s| Box::new(VariableContent::new(s.name().to_owned())))
                    .collect()]
            })
            .collect();

        Self(vector)
    }
}

pub(crate) type VarRegistryVector<'a> = Vec<Vec<Vec<Box<VariableContent<'a, GritQueryContext>>>>>;

impl<'a> From<VarRegistry<'a>> for VarRegistryVector<'a> {
    fn from(value: VarRegistry<'a>) -> Self {
        value.0
    }
}
