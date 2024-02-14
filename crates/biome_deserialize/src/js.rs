//! Implementation of [DeserializableValue] for the JS engine.
use crate::{DeserializationDiagnostic, Deserialized};
use biome_console::markup;
use biome_diagnostics::Error;
use boa_engine::{Context, JsValue};
use serde::de::DeserializeOwned;

/// Attempts to interpret a value from the JS runtime and deserialize it following the semantics
/// defined by JSON. As such, JS types that cannot be serialized to JSON, such as functions or
/// Promises are not supported by this deserializer.
///
/// The data structures that need to be deserialized have to implement the [Deserializable] trait.
/// For most data structures, this can be achieved using the
/// [biome_deserialize_macros::Deserializable] derive.
///
/// `name` corresponds to the name used in a diagnostic to designate the deserialized value.
///
/// ## Examples
///
/// ```
/// use biome_deserialize::js::deserialize_from_js_value;
/// use biome_deserialize_macros::Deserializable;
/// use boa_engine::{Context, Source};
///
/// #[derive(Debug, Default, Deserializable, Eq, PartialEq)]
/// struct NewConfiguration {
///     lorem: String
/// }
///
/// let source = r#"{ lorem: "ipsum" }"#;
///
/// let mut context = Context::default();
/// let value = context.eval(Source::from_bytes(source)).unwrap();
///
/// let deserialized = deserialize_from_js_value::<NewConfiguration>(&mut context, value);
/// assert!(!deserialized.has_errors());
/// assert_eq!(deserialized.into_deserialized().unwrap(), NewConfiguration { lorem: "ipsum".to_string() });
/// ```
pub fn deserialize_from_js_value<Output: DeserializeOwned>(
    context: &mut Context,
    value: JsValue,
) -> Deserialized<Output> {
    let mut diagnostics: Vec<DeserializationDiagnostic> = Vec::new();

    // TODO: We probably want to improve our diagnostics somewhat.
    let deserialized = match value.to_json(context) {
        Ok(json) => match serde_json::from_value(json) {
            Ok(value) => Some(value),
            Err(err) => {
                diagnostics.push(DeserializationDiagnostic::new(
                    markup!("Value could not be exported to JSON: "{err.to_string()}),
                ));
                None
            }
        },
        Err(err) => {
            diagnostics.push(DeserializationDiagnostic::new(
                markup!("Value could not be parsed as JSON: "{err.to_string()}),
            ));
            None
        }
    };

    Deserialized {
        diagnostics: diagnostics.into_iter().map(Error::from).collect::<Vec<_>>(),
        deserialized,
    }
}
