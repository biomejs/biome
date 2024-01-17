# `biome_deserialize`

`biome_deserialize` consists of data structures that know how to deserialize themselves
along with data formats that know how to deserialize data.
It provides a framework by which these two groups interact with each other,
allowing any supported data structure to be deserialized using any supported data format.

This crate inspired by [serde](https://serde.rs/).
0ne of the main difference is the fault-tolerant behavior of `biome_deserialize`.
_Serde_ uses a fast-fail strategy, while `biome_deserialize` deserialize as much as possible
and report several diagnostics (errors, warning, deprecation messages, ...).
Also, `biome_deserialize` is intended to deserialize textual data formats.

`biome_deserialize` assumes that every supported data formats supports the following types:

- null-like values;
- boolean;
- number -- integers and floats;
- string;
- array;
- maps of key-value pairs (covers objects).

It currently supports the JSON data format.

## Design overview

The two most important traits are `Deserializable`, `DeserializableValue`.

- A type that implements `Deserializable` is a data structure that can be
  deserialized from any supported data format;
- A type that implements `DeserializableValue` is a data format that can
  deserialize any supported data structure.

Simple implementations of `Deserializable` can reuse other deserializable data structures.
For instance, an enumeration that corresponds to a string among A, B, and C, can first deserialize a string and then check that the string is one of its values.

Data structures that cannot directly use another deserializable data structures, use a visitor.
A visitor is generally a zero-sized data structure that implements the `DeserializationVisitor` trait.
A [visitor](https://en.wikipedia.org/wiki/Visitor_pattern) is a well-known design pattern.
It allows selecting an implementation based on the deserialized type without bothering of data format details.

## Usage examples

### Deserializing common types

`biome_deserialize` implements `Deserializable` for common Rust data structure.

In the following example, we deserialize a boolean, an array of integers, and an unordered map of string-integer pairs.

```rust
use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Deserialized;
use biome_json_parser::JsonParserOptions;

let json = "false";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<bool>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(false));
assert!(diagnostics.is_empty());

let json = "[0, 1]";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<Vec<u8>>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(vec![0, 1]));
assert!(diagnostics.is_empty());

use std::collections::HashMap;
let json = r#"{ "a": 0, "b": 1 }"#;
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<HashMap<String, u8>>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(HashMap::from([("a".to_string(), 0), ("b".to_string(), 1)])));
assert!(diagnostics.is_empty());
```

### Deserializable derive macro

For more complex types, such as structs and enums, `biome_deserialize_macros` offers a derive macro
which will generate an implementation of the `Deserializable` trait for you.

For example:

```rs
#[derive(Clone, Debug, Default, Deserializable)]
pub struct MyRuleOptions {
    behavior: Behavior,
    threshold: u8,
    behavior_exceptions: Vec<String>
}
```

Extensive documentation for the macro is also available if you hover over it in your IDE.

Note that the macro has two main limitations:

- The current implementation only supports enums where none of the variants have custom fields,
  structs with named fields, and newtypes.
- Every field must also implement `Deserializable`.

If you want to implement `Deserializable` for a new primitive type, or a complex type that falls
outside the limitations above, you'll need to implement it manually. See below for the section about
[Implementing a custom deserializer](#implementing-a-custom-deserializer).

## Deserializing map and array collections

`biome_deserialize` requires that every data format supports arrays and maps.
A map and an array can be deserialized into several types in Rust.

`biome_deserialize` is able to deserialize an array into a `Vec`, a `HashSet`, or a `IndexSet`.

- `Vec` preserves the insertion order and allows the repetition of values;
- `HashSet` **doesn't preserve the insertion order** and disallows the repetition of values;
- `IndexSet` preserves the insertion order and disallows the repetition of values.

`biome_deserialize` is able to deserialize a map into a `HashMap`, a `BTreeMap`, or a `IndexMap`.

- `HashMap` and `BTreeMap` **don't preserve the insertion order**;
- `IndexMap` preserves the insertion order.

If you hesitate between a collection that preserves the insertion order and one that doesn't,
chooses the collection that preserves the insertion order.
This often outputs less surprising behavior.

## Implementing a custom deserializer

For most enums and structs, an implementation can be generated by the `Deserializable` macro.
However, for primitives, and certain complex types, you may need to implement
`biome_deserialize::Deserializable` yourself.

> [!NOTE]
> If you are curious about the code generated by the macro, *Rust Analyzer* offers a great feature
> from the command palette called "Expand macro recursively at caret". Just put your cursor on the
> word `Deserializable` in the `#[derive(...)]` statement and invoke the command. A panel should
> open with the expanded macro code.

We provide a few examples for custom implementations below.

### Custom integer range

Sometimes you want to deserialize an integer and ensure that it is between two given integers.

For instance, let's assume we want to deserialize a day represented by an integer between 1 and 365.
We can use the [new-type idiom](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) in Rust:

```rust
use std::str::FromStr;
use biome_deserialize::{Deserializable, DeserializableValue, DeserializationDiagnostic, TextNumber};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct Day(u16);

impl Day {
    pub const MIN: Day = Day(1);
    pub const MAX: Day = Day(365);
    pub fn get(&self) -> u16 {
        self.0
    }
}

impl Default for Day {
    fn default() -> Self {
        Self::MIN
    }
}

impl TryFrom<u16> for Day {
    type Error = &'static str;
    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if (Self::MIN.get()..=Self::MAX.get()).contains(&value) {
            Ok(Self(value))
        } else {
            Err("A day must be between 1 and 365")
        }
    }
}

impl FromStr for Day {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u16>()
            .map_err(|_error| "A day must be an integer between 1 and 365")
            .and_then(|value| Day::try_from(value))
    }
}

impl Deserializable for Day {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        // We deserialize the value into a number represented as a string.
        let value_text = TextNumber::deserialize(value, name, diagnostics)?;
        // We attempt to convert the string into a `Day`.
        value_text.parse::<Day>().map_err(|error| {
            // If the conversion failed, then we report the error.
            diagnostics.push(DeserializationDiagnostic::new(error).with_range(value.range()));
        }).ok()
    }
}

use biome_deserialize::json::deserialize_from_json_str;
use biome_deserialize::Deserialized;
use biome_json_parser::JsonParserOptions;

let json = "42";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<Day>(&source, JsonParserOptions::default());
assert_eq!(deserialized, Some(Day(42)));
assert!(diagnostics.is_empty());

let json = "999";
let Deserialized {
    deserialized,
    diagnostics,
} = deserialize_from_json_str::<Day>(&source, JsonParserOptions::default());
assert_eq!(deserialized, None);
assert_eq!(diagnostics..len(), 1);
```

### Deserializing a union

Sometimes we want to allow several types for a single value.
For instance let's assume that we cant to accept a JSON value that is either a string or a boolean.

In this case, we'll need to inspect the type of the `DeserializableValue` to know which deserializer
to use:

```rust
use biome_deserialize::{DeserializationDiagnostic, Deserializable, DeserializableValue, DeserializationVisitor, Text, VisitableType};
use biome_rowan::TextRange;

#[derive(Debug, Eq, PartialEq)]
enum Union {
    Bool(bool),
    Str(String),
}

impl Deserializable for Union {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        if value.is_type(VisitableType::BOOL) {
            biome_deserialize::Deserializable::deserialize(value, rule_name, diagnostics)
                .map(Self::Bool)
        } else {
            biome_deserialize::Deserializable::deserialize(value, rule_name, diagnostics)
                .map(Self::Str)
        }
    }
}

use biome_deserialize::json::deserialize_from_json_str;
use biome_json_parser::JsonParserOptions;

let source = r#" "string" "#;
let deserialized = deserialize_from_json_str::<Union>(&source, JsonParserOptions::default());
assert!(!deserialized.has_errors());
assert_eq!(deserialized.into_deserialized(), Some(Union::Str("string".to_string())));

let source = "true";
let deserialized = deserialize_from_json_str::<Union>(&source, JsonParserOptions::default());
assert!(!deserialized.has_errors());
assert_eq!(deserialized.into_deserialized(), Some(Union::Bool(true)));
```

Alternatively, we could implement the above using a custom visitor. Note that this approach is much
more involved and should only be used as a last resort. Here, we will reimplement `Deserializable`
for our `Union` type with a visitor for demonstration purposes.

To create your own visitor, start with a struct named after your type with a `Visitor` suffix. We'll
call ours `UnionVisitor`.

The visitor struct doesn't need any fields, but we do need to implement `DeserializationVisitor` on
it. A `DeserializationVisitor` provides several `visit_` methods and you must implement the `visit_`
methods of the type(s) you expect. Here we expect either a boolean or a string, so we'll implement
`visit_bool()` and `visit_str()`.

We also have to set the associated type `Output` to be a union of the types we expect:
`VisitableType::BOOL.union(VisitableType::STR)`.

The full example:

```rust
impl Deserializable for Union {
    fn deserialize(
        value: &impl DeserializableValue,
        name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self> {
        // Delegate deserialization to `UnionVisitor`
        value.deserialize(UnionVisitor, name, diagnostics)
    }
}

struct UnionVisitor;
impl DeserializationVisitor for UnionVisitor {
    type Output = Union;

    // We expect a `bool` or a `str` as data type.
    const EXPECTED_TYPE: VisitableType = VisitableType::BOOL.union(VisitableType::STR);

    // Because we expect a `bool` or a `str`, we have to implement the associated method `visit_bool`.
    fn visit_bool(
        self,
        value: bool,
        range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        Some(Union::Bool(value))
    }

    // Because we expect a `bool` or a `str`, we have to implement the associated method `visit_str`.
    fn visit_str(
        self,
        value: Text,
        range: TextRange,
        _name: &str,
        diagnostics: &mut Vec<DeserializationDiagnostic>,
    ) -> Option<Self::Output> {
        Some(Union::Str(value.text().to_string()))
    }
}
```
