use biome_console::markup;
use biome_deserialize::{
    Deserializable, DeserializableType, DeserializableValue, DeserializationContext,
    DeserializationDiagnostic,
};
use biome_deserialize_macros::Deserializable;
use biome_rowan::TextRange;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use serde::{Deserialize, Serialize};

/// Restricted classes or variants, keyed by pattern, in the order they're configured.
pub type TailwindRestrictions = IndexMap<Box<str>, TailwindRestriction, FxBuildHasher>;

#[derive(Default, Clone, Debug, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct NoTailwindRestrictedClassesOptions {
    /// Classes to restrict.
    ///
    /// Each key is a class without variants or `!`, such as `text-green-500`.
    /// A `*` can stand for the whole utility name, as in `*-md`, the whole
    /// value, as in `bg-*`, or the value of an arbitrary property, as in
    /// `[color:*]`. Each value is either the message to show, or an object with
    /// an optional `message` and an optional `use` replacement.
    #[deserializable(validate = "valid_class_keys")]
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub classes: Option<TailwindRestrictions>,

    /// Variants to restrict.
    ///
    /// Each key is a variant, such as `dark` or `group-hover`. A `*` can stand
    /// for the parts at the end, as in `group-*`, or at the start, as in
    /// `*-hover`, and `[*]` matches any arbitrary variant. Each value is either
    /// the message to show, or an object with an optional `message` and an
    /// optional `use` replacement.
    #[deserializable(validate = "valid_variant_keys")]
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub variants: Option<TailwindRestrictions>,
}

impl biome_deserialize::Merge for NoTailwindRestrictedClassesOptions {
    fn merge_with(&mut self, other: Self) {
        if let Some(classes) = other.classes {
            self.classes = Some(classes);
        }
        if let Some(variants) = other.variants {
            self.variants = Some(variants);
        }
    }
}

/// Rejects class keys that use `*` for something other than a whole base,
/// a whole value, or the value of an arbitrary property.
fn valid_class_keys<S>(
    ctx: &mut dyn DeserializationContext<State = S>,
    value: &Option<TailwindRestrictions>,
    _name: &str,
    range: TextRange,
) -> bool {
    let mut is_valid = true;
    for key in value.iter().flat_map(|restrictions| restrictions.keys()) {
        if !is_valid_class_key(key) {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    "The class "<Emphasis>{key.as_ref()}</Emphasis>" uses "<Emphasis>"*"</Emphasis>" in a way that isn't supported. "
                    "A "<Emphasis>"*"</Emphasis>" can only replace the whole utility name, as in "<Emphasis>"*-md"</Emphasis>", "
                    "the whole value, as in "<Emphasis>"bg-*"</Emphasis>", "
                    "or the value of an arbitrary property, as in "<Emphasis>"[color:*]"</Emphasis>"."
                })
                .with_range(range),
            );
            is_valid = false;
        }
    }
    is_valid
}

fn is_valid_class_key(key: &str) -> bool {
    let pattern = key.strip_prefix('-').unwrap_or(key);
    if !pattern.contains('*') {
        return true;
    }
    if let Some(value) = pattern.strip_prefix("*-") {
        return !value.is_empty() && !value.contains('*');
    }
    if let Some(base) = pattern.strip_suffix("-*") {
        return !base.is_empty() && !base.contains('*');
    }
    pattern
        .strip_prefix('[')
        .and_then(|pattern| pattern.strip_suffix(":*]"))
        .is_some_and(|property| !property.is_empty() && !property.contains(['*', ':', '[', ']']))
}

/// Rejects variant keys that use `*` for something other than leading or
/// trailing parts, except `[*]`.
fn valid_variant_keys<S>(
    ctx: &mut dyn DeserializationContext<State = S>,
    value: &Option<TailwindRestrictions>,
    _name: &str,
    range: TextRange,
) -> bool {
    let mut is_valid = true;
    for key in value.iter().flat_map(|restrictions| restrictions.keys()) {
        if !is_valid_variant_key(key) {
            ctx.report(
                DeserializationDiagnostic::new(markup! {
                    "The variant "<Emphasis>{key.as_ref()}</Emphasis>" uses "<Emphasis>"*"</Emphasis>" in a way that isn't supported. "
                    "A "<Emphasis>"*"</Emphasis>" can only replace the parts at the end, as in "<Emphasis>"group-*"</Emphasis>", "
                    "or at the start, as in "<Emphasis>"*-hover"</Emphasis>". "
                    "Use "<Emphasis>"[*]"</Emphasis>" to match any arbitrary variant."
                })
                .with_range(range),
            );
            is_valid = false;
        }
    }
    is_valid
}

fn is_valid_variant_key(key: &str) -> bool {
    let pattern = key.strip_suffix(':').unwrap_or(key);
    if !pattern.contains('*') || pattern == "[*]" {
        return true;
    }
    pattern
        .strip_prefix("*-")
        .or_else(|| pattern.strip_suffix("-*"))
        .is_some_and(|rest| !rest.is_empty() && !rest.contains('*'))
}

impl NoTailwindRestrictedClassesOptions {
    /// Returns [`Self::classes`] if it is set.
    /// Otherwise, returns `None`.
    pub fn classes(&self) -> Option<&TailwindRestrictions> {
        self.classes.as_ref()
    }

    /// Returns [`Self::variants`] if it is set.
    /// Otherwise, returns `None`.
    pub fn variants(&self) -> Option<&TailwindRestrictions> {
        self.variants.as_ref()
    }
}

/// The message and replacement of a restricted class or variant.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(untagged)]
pub enum TailwindRestriction {
    Message(Box<str>),
    WithOptions(TailwindRestrictionOptions),
}

impl TailwindRestriction {
    /// Returns the message to show, if one is configured.
    pub fn message(&self) -> Option<&str> {
        match self {
            Self::Message(message) => Some(message),
            Self::WithOptions(options) => options.message.as_deref(),
        }
    }

    /// Returns the class or variant to use instead, if one is configured.
    pub fn use_instead(&self) -> Option<&str> {
        match self {
            Self::Message(_) => None,
            Self::WithOptions(options) => options.use_instead.as_deref(),
        }
    }
}

impl Deserializable for TailwindRestriction {
    fn deserialize<V: DeserializableValue>(
        ctx: &mut dyn DeserializationContext<State = V::State>,
        value: &V,
        name: &str,
    ) -> Option<Self> {
        if value.visitable_type(ctx)? == DeserializableType::Str {
            Deserializable::deserialize(ctx, value, name).map(Self::Message)
        } else {
            Deserializable::deserialize(ctx, value, name).map(Self::WithOptions)
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Deserializable, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", deny_unknown_fields, default)]
pub struct TailwindRestrictionOptions {
    /// The message to show.
    #[serde(skip_serializing_if = "Option::<_>::is_none")]
    pub message: Option<Box<str>>,

    /// The class or variant to use instead. Biome suggests a fix that uses it
    /// when the key has no `*`.
    #[serde(rename = "use", skip_serializing_if = "Option::<_>::is_none")]
    pub use_instead: Option<Box<str>>,
}

#[cfg(test)]
mod tests {
    use super::NoTailwindRestrictedClassesOptions;
    use biome_deserialize::json::deserialize_from_json_str;
    use biome_json_parser::JsonParserOptions;

    fn diagnostics(source: &str) -> usize {
        deserialize_from_json_str::<NoTailwindRestrictedClassesOptions>(
            source,
            JsonParserOptions::default(),
            "",
        )
        .into_diagnostics()
        .len()
    }

    #[test]
    fn accepts_supported_class_keys() {
        for key in [
            "flex",
            "bg-*",
            "*-md",
            "-*-4",
            "-mt-*",
            "[color:*]",
            "w-1/2",
            "*-1/2",
        ] {
            assert_eq!(
                diagnostics(&format!(r#"{{ "classes": {{ "{key}": "" }} }}"#)),
                0,
                "{key}"
            );
        }
    }

    #[test]
    fn rejects_unsupported_class_keys() {
        for key in [
            "*", "*-*", "bg-*-500", "w-[*]", "*bg", "bg*", "[*:red]", "[*]", "-*",
        ] {
            assert_eq!(
                diagnostics(&format!(r#"{{ "classes": {{ "{key}": "" }} }}"#)),
                1,
                "{key}"
            );
        }
    }

    #[test]
    fn accepts_supported_variant_keys() {
        for key in [
            "dark", "dark:", "group-*", "*-hover", "group-*:", "[*]", "[&_p]",
        ] {
            assert_eq!(
                diagnostics(&format!(r#"{{ "variants": {{ "{key}": "" }} }}"#)),
                0,
                "{key}"
            );
        }
    }

    #[test]
    fn rejects_unsupported_variant_keys() {
        for key in ["*", "*-*", "group-*-hover", "group*", "[*]-x"] {
            assert_eq!(
                diagnostics(&format!(r#"{{ "variants": {{ "{key}": "" }} }}"#)),
                1,
                "{key}"
            );
        }
    }
}
