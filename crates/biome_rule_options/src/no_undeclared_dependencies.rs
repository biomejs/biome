use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
};
use biome_deserialize_macros::Deserializable;
use camino::Utf8Path;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoUndeclaredDependenciesOptions {
    /// If set to `false`, then the rule will show an error when `devDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    pub dev_dependencies: DependencyAvailability,

    /// If set to `false`, then the rule will show an error when `peerDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    pub peer_dependencies: DependencyAvailability,

    /// If set to `false`, then the rule will show an error when `optionalDependencies` are imported. Defaults to `true`.
    #[serde(default)]
    pub optional_dependencies: DependencyAvailability,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
pub enum DependencyAvailability {
    /// Dependencies are always available or unavailable.
    Bool(bool),

    /// Dependencies are available in files that matches any of the globs.
    Patterns(Box<[biome_glob::Glob]>),
}

impl Default for DependencyAvailability {
    fn default() -> Self {
        Self::Bool(true)
    }
}

impl Deserializable for DependencyAvailability {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        Some(if value.visitable_type()? == DeserializableType::Bool {
            Self::Bool(<bool as Deserializable>::deserialize(ctx, value, name)?)
        } else {
            Self::Patterns(Deserializable::deserialize(ctx, value, name)?)
        })
    }
}

#[cfg(feature = "schema")]
impl schemars::JsonSchema for DependencyAvailability {
    fn schema_name() -> String {
        "DependencyAvailability".to_owned()
    }

    fn json_schema(_generator: &mut schemars::r#gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;

        Schema::Object(SchemaObject {
            subschemas: Some(Box::new(SubschemaValidation {
                one_of: Some(vec![
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Boolean.into()),
                        metadata: Some(Box::new(Metadata {
                            description: Some("This type of dependency will be always available or unavailable.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }),
                    Schema::Object(SchemaObject {
                        instance_type: Some(InstanceType::Array.into()),
                        array: Some(Box::new(ArrayValidation {
                            items: Some(SingleOrVec::Single(Box::new(Schema::Object(SchemaObject {
                                instance_type: Some(InstanceType::String.into()),
                                ..Default::default()
                            })))),
                            min_items: Some(1),
                            ..Default::default()
                        })),
                        metadata: Some(Box::new(Metadata {
                            description: Some("This type of dependency will be available only if the linted file matches any of the globs.".to_owned()),
                            ..Default::default()
                        })),
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}

impl DependencyAvailability {
    pub fn is_available(&self, path: &Utf8Path) -> bool {
        match self {
            Self::Bool(b) => *b,
            Self::Patterns(globs) => {
                biome_glob::CandidatePath::new(&path).matches_with_exceptions(globs)
            }
        }
    }
}
