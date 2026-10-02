use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationDiagnostic,
    ErasedDeserializationVisitor, TextRange,
};
use biome_deserialize_macros::Deserializable;

#[derive(Default)]
struct Context {
    visits: usize,
    diagnostics: Vec<DeserializationDiagnostic>,
}

impl biome_deserialize::DeserializationContext for Context {
    fn id(&self) -> Option<&str> {
        Some("root")
    }

    fn report(&mut self, diagnostic: DeserializationDiagnostic) {
        self.diagnostics.push(diagnostic);
    }
}

#[derive(Clone)]
enum Value {
    Str(String),
    Array(Vec<Self>),
    Map(Vec<(String, Self)>),
}

impl DeserializableValue for Value {
    type Context = Context;

    fn range(&self) -> TextRange {
        TextRange::default()
    }

    fn deserialize_erased(
        &self,
        ctx: &mut Context,
        visitor: &mut dyn ErasedDeserializationVisitor<Context>,
        name: &str,
    ) {
        ctx.visits += 1;
        match self {
            Self::Str(text) => visitor.visit_str(ctx, text.clone().into(), self.range(), name),
            Self::Array(values) => {
                let mut values = values.iter().cloned().map(|value| {
                    Some(Box::new(value) as Box<dyn DeserializableValue<Context = Context>>)
                });
                visitor.visit_array(ctx, &mut values, self.range(), name);
            }
            Self::Map(members) => {
                let mut members = members.iter().cloned().map(|(key, value)| {
                    Some((
                        Box::new(Self::Str(key)) as Box<dyn DeserializableValue<Context = Context>>,
                        Box::new(value) as Box<dyn DeserializableValue<Context = Context>>,
                    ))
                });
                visitor.visit_map(ctx, &mut members, self.range(), name);
            }
        }
    }

    fn visitable_type(&self, _ctx: &mut Context) -> Option<DeserializableType> {
        Some(match self {
            Self::Str(_) => DeserializableType::Str,
            Self::Array(_) => DeserializableType::Array,
            Self::Map(_) => DeserializableType::Map,
        })
    }
}

#[derive(Debug, Eq, PartialEq, Deserializable)]
enum Mode {
    Yes,
    No,
}

#[derive(Debug, Default, Eq, PartialEq, Deserializable)]
#[deserializable(with_validator)]
struct Configuration {
    modes: Vec<Mode>,
}

impl biome_deserialize::DeserializableValidator for Configuration {
    fn validate(
        &mut self,
        ctx: &mut dyn biome_deserialize::DeserializationContext,
        name: &str,
        range: TextRange,
    ) -> bool {
        assert_eq!(ctx.id(), Some("root"));
        biome_deserialize::non_empty(ctx, &self.modes, name, range)
    }
}

#[test]
fn associated_context_is_shared_with_erased_children_and_validators() {
    let value = Value::Map(vec![(
        "modes".into(),
        Value::Array(vec![Value::Str("yes".into()), Value::Str("no".into())]),
    )]);
    let mut ctx = Context::default();
    assert_eq!(
        Configuration::deserialize(&mut ctx, &value, ""),
        Some(Configuration {
            modes: vec![Mode::Yes, Mode::No],
        })
    );
    assert_eq!(ctx.visits, 5);
    assert!(ctx.diagnostics.is_empty());

    let unknown = Value::Str("unknown".into());
    assert_eq!(Mode::deserialize(&mut ctx, &unknown, ""), None);
    assert_eq!(ctx.visits, 6);
    assert_eq!(ctx.diagnostics.len(), 1);

    ctx.diagnostics.clear();
    let empty = Value::Map(vec![("modes".into(), Value::Array(vec![]))]);
    assert_eq!(Configuration::deserialize(&mut ctx, &empty, ""), None);
    assert_eq!(ctx.diagnostics.len(), 1);
}

#[test]
fn json_context_preserves_root_identifier_for_validators() {
    let (value, diagnostics) = biome_deserialize::json::deserialize_from_json_str::<Configuration>(
        r#"{"modes": ["yes", "no"]}"#,
        biome_json_parser::JsonParserOptions::default(),
        "root",
    )
    .consume();
    assert_eq!(
        value,
        Some(Configuration {
            modes: vec![Mode::Yes, Mode::No],
        })
    );
    assert!(diagnostics.is_empty());
}

#[test]
#[cfg(feature = "yaml")]
fn yaml_context_preserves_root_identifier_for_validators_and_aliases() {
    let (value, diagnostics) = biome_deserialize::yaml::deserialize_from_yaml_str::<Configuration>(
        "modes: [&mode yes, *mode]",
        "root",
    )
    .consume();
    assert_eq!(
        value,
        Some(Configuration {
            modes: vec![Mode::Yes, Mode::Yes],
        })
    );
    assert!(diagnostics.is_empty());
}
