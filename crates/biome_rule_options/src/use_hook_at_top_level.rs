use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableTypes, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic, DeserializationVisitor, TextRange,
};
use biome_rowan::Text;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct UseHookAtTopLevelOptions {}

impl Deserializable for UseHookAtTopLevelOptions {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        value.deserialize(ctx, DeprecatedHooksOptionsVisitor, name)
    }
}

// TODO: remove in Biome 2.0
struct DeprecatedHooksOptionsVisitor;
impl DeserializationVisitor for DeprecatedHooksOptionsVisitor {
    type Output = UseHookAtTopLevelOptions;

    const EXPECTED_TYPE: DeserializableTypes = DeserializableTypes::MAP;

    fn visit_map(
        self,
        ctx: &mut impl DeserializationContext,
        members: impl Iterator<Item = Option<(impl DeserializableValue, impl DeserializableValue)>>,
        _range: TextRange,
        _name: &str,
    ) -> Option<Self::Output> {
        const ALLOWED_KEYS: &[&str] = &["hooks"];
        for (key, value) in members.flatten() {
            let Some(key_text) = Text::deserialize(ctx, &key, "") else {
                continue;
            };
            match key_text.text() {
                "hooks" => {
                    ctx.report(
                        DeserializationDiagnostic::new_deprecated(
                            key_text.text(),
                            value.range()
                        ).with_note(
                            markup! {
                            <Emphasis>"useHookAtTopLevel"</Emphasis>" now uses the React hook naming convention to determine hook calls."
                        })
                    );
                }
                text => ctx.report(DeserializationDiagnostic::new_unknown_key(
                    text,
                    key.range(),
                    ALLOWED_KEYS,
                )),
            }
        }
        Some(Self::Output::default())
    }
}
