//! JSON value types based on `biome_json_syntax`.
//!
//! Compared to the CST types defined in `biome_json_syntax`, these have the
//! following characteristics:
//!
//! - Values can be constructed from [`biome_json_syntax::AnyJsonValue`] with
//!   few allocations. The only types that require allocation are arrays and
//!   objects, as well as strings that contain escape sequences.
//! - Objects, strings, and arrays can be mutated.
//! - String values have escape sequences processed, so that their values
//!   correspond to runtime values and not just lexical values.
//! - [`oxc_resolver::ImportsExportsEntry`], [`oxc_resolver::ImportsExportsArray`],
//!   and [`oxc_resolver::ImportsExportsMap`] are implemented for [`JsonValue`],
//!   [`JsonArray`] and [`JsonObject`], respectively. This functionality can be
//!   enabled through the `oxc_resolver` feature flag.
//! - They are both [`Send`] and [`Sync`], so they can be shared across threads.
use std::{
    borrow::Borrow,
    cmp::Ordering,
    ops::{Deref, DerefMut},
};

use biome_deserialize::{
    json::unescape_json_string, DeserializableType, DeserializableValue, DeserializationContext,
    Text,
};
use biome_deserialize_macros::Deserializable;
use biome_json_syntax::{AnyJsonValue, JsonArrayValue, JsonObjectValue, JsonStringValue};
use biome_rowan::TokenText;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

/// JSON value based on types from `biome_json_syntax`.
///
/// See the [module-level documentation](self) for more info.
#[derive(Clone, Debug, PartialEq)]
pub enum JsonValue {
    Array(JsonArray),
    Bool(bool),
    Null,
    Number(f64),
    Object(JsonObject),
    String(JsonString),
    Bogus,
}

static_assertions::assert_impl_all!(JsonValue: Send, Sync);

impl JsonValue {
    pub const fn as_array(&self) -> Option<&JsonArray> {
        match self {
            Self::Array(array) => Some(array),
            _ => None,
        }
    }

    pub const fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(bool) => Some(*bool),
            _ => None,
        }
    }

    pub const fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number(number) => Some(*number),
            _ => None,
        }
    }

    pub const fn as_object(&self) -> Option<&JsonObject> {
        match self {
            Self::Object(object) => Some(object),
            _ => None,
        }
    }

    pub const fn as_string(&self) -> Option<&JsonString> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub const fn is_bogus(&self) -> bool {
        matches!(self, Self::Bogus)
    }

    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

impl biome_deserialize::Deserializable for JsonValue {
    fn deserialize(
        ctx: &mut impl DeserializationContext,
        value: &impl DeserializableValue,
        name: &str,
    ) -> Option<Self> {
        match value.visitable_type()? {
            DeserializableType::Array => JsonArray::deserialize(ctx, value, name).map(Self::Array),
            DeserializableType::Bool => bool::deserialize(ctx, value, name).map(Self::Bool),
            DeserializableType::Map => JsonObject::deserialize(ctx, value, name).map(Self::Object),
            DeserializableType::Null => Some(Self::Null),
            DeserializableType::Number => f64::deserialize(ctx, value, name).map(Self::Number),
            DeserializableType::Str => JsonString::deserialize(ctx, value, name).map(Self::String),
        }
    }
}

impl From<AnyJsonValue> for JsonValue {
    fn from(value: AnyJsonValue) -> Self {
        match value {
            AnyJsonValue::JsonArrayValue(array_value) => Self::Array(array_value.into()),
            AnyJsonValue::JsonBogusValue(_) => Self::Bogus,
            AnyJsonValue::JsonBooleanValue(boolean_value) => match boolean_value.value_token() {
                Ok(value) => Self::Bool(value.text_trimmed() == "true"),
                Err(_) => Self::Bogus,
            },
            AnyJsonValue::JsonNullValue(_) => Self::Null,
            AnyJsonValue::JsonNumberValue(number_value) => match number_value.value_token() {
                Ok(value) => match value.text_trimmed().parse() {
                    Ok(number) => Self::Number(number),
                    Err(_) => Self::Bogus,
                },
                Err(_) => Self::Bogus,
            },
            AnyJsonValue::JsonObjectValue(object_value) => Self::Object(object_value.into()),
            AnyJsonValue::JsonStringValue(string_value) => Self::String(string_value.into()),
        }
    }
}

impl From<JsonArray> for JsonValue {
    fn from(value: JsonArray) -> Self {
        Self::Array(value)
    }
}

impl From<JsonObject> for JsonValue {
    fn from(value: JsonObject) -> Self {
        Self::Object(value)
    }
}

impl From<JsonString> for JsonValue {
    fn from(value: JsonString) -> Self {
        Self::String(value)
    }
}

#[cfg(feature = "oxc_resolver")]
impl<'a> oxc_resolver::ImportsExportsEntry<'a> for &'a JsonValue {
    type Array = &'a JsonArray;
    type Map = &'a JsonObject;

    fn kind(&self) -> oxc_resolver::ImportsExportsKind {
        match self {
            JsonValue::Array(_) => oxc_resolver::ImportsExportsKind::Array,
            JsonValue::Object(_) => oxc_resolver::ImportsExportsKind::Map,
            JsonValue::String(_) => oxc_resolver::ImportsExportsKind::String,
            _ => oxc_resolver::ImportsExportsKind::Invalid,
        }
    }

    fn as_string(&self) -> Option<&'a str> {
        match self {
            JsonValue::String(string) => Some(string.as_ref()),
            _ => None,
        }
    }

    fn as_array(&self) -> Option<Self::Array> {
        match self {
            JsonValue::Array(array) => Some(array),
            _ => None,
        }
    }

    fn as_map(&self) -> Option<Self::Map> {
        match self {
            JsonValue::Object(map) => Some(map),
            _ => None,
        }
    }
}

/// JSON array to be used with [JsonValue].
///
/// See the [module-level documentation](self) for more info.
#[derive(Clone, Debug, Default, Deserializable, PartialEq)]
pub struct JsonArray(Vec<JsonValue>);

impl Deref for JsonArray {
    type Target = Vec<JsonValue>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsonArray {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<JsonArrayValue> for JsonArray {
    fn from(value: JsonArrayValue) -> Self {
        let vec = value
            .elements()
            .into_iter()
            .map(|element| match element {
                Ok(element) => element.into(),
                Err(_) => JsonValue::Bogus,
            })
            .collect();
        Self(vec)
    }
}

#[cfg(feature = "oxc_resolver")]
impl<'a> oxc_resolver::ImportsExportsArray<'a> for &'a JsonArray {
    type Entry = &'a JsonValue;

    fn len(&self) -> usize {
        self.0.len()
    }

    fn iter(&self) -> impl Iterator<Item = Self::Entry> {
        self.0.iter()
    }
}

/// JSON object to be used with [JsonValue].
///
/// See the [module-level documentation](self) for more info.
#[derive(Clone, Debug, Default, Deserializable, PartialEq)]
pub struct JsonObject(IndexMap<JsonString, JsonValue, FxBuildHasher>);

impl Deref for JsonObject {
    type Target = IndexMap<JsonString, JsonValue, FxBuildHasher>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for JsonObject {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<const N: usize> From<[(JsonString, JsonValue); N]> for JsonObject {
    fn from(value: [(JsonString, JsonValue); N]) -> Self {
        Self(IndexMap::from_iter(value))
    }
}

impl From<JsonObjectValue> for JsonObject {
    fn from(value: JsonObjectValue) -> Self {
        let map = value
            .json_member_list()
            .into_iter()
            .filter_map(|member| match member {
                Ok(member) => {
                    let key = member.name().ok()?;
                    let value = member.value().ok()?;
                    Some((key.inner_string_text().ok()?.into(), value.into()))
                }
                Err(_) => None,
            })
            .collect();
        Self(map)
    }
}

#[cfg(feature = "oxc_resolver")]
impl<'a> oxc_resolver::ImportsExportsMap<'a> for &'a JsonObject {
    type Entry = &'a JsonValue;

    fn get(&self, key: &str) -> Option<Self::Entry> {
        self.0.get(key)
    }

    fn iter(&self) -> impl Iterator<Item = (&'a str, Self::Entry)> {
        self.0.iter().map(|(key, value)| (key.as_ref(), value))
    }

    fn keys(&self) -> impl Iterator<Item = &'a str> {
        self.0.keys().map(JsonString::as_ref)
    }
}

/// JSON string to be used with [JsonValue].
///
/// This type can be constructed from [TokenText], in which case it will process
/// any embedded escape sequences. Allocation is only required if escape
/// sequences are present.
///
/// See the [module-level documentation](self) for more info.
#[derive(Clone, Debug, Default, Deserializable, Eq, Hash, PartialEq)]
pub struct JsonString(Text);

impl JsonString {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl AsRef<str> for JsonString {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl Borrow<str> for JsonString {
    fn borrow(&self) -> &str {
        self.0.as_ref()
    }
}

impl From<&str> for JsonString {
    fn from(value: &str) -> Self {
        Self(Text::Owned(value.to_string()))
    }
}

impl From<JsonStringValue> for JsonString {
    fn from(value: JsonStringValue) -> Self {
        match value.inner_string_text() {
            Ok(text) => text.into(),
            Err(_) => Self(Text::Owned(String::new())),
        }
    }
}

impl From<String> for JsonString {
    fn from(value: String) -> Self {
        Self(Text::Owned(value))
    }
}

impl From<TokenText> for JsonString {
    fn from(text: TokenText) -> Self {
        Self(unescape_json_string(text))
    }
}

impl Ord for JsonString {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialOrd for JsonString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
#[path = "json_value.tests.rs"]
mod tests;
