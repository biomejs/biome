# `biome_deserialize`

## v1.0.0


### Replace `diagnostics` with `context`

This is a **breaking change**.
You have to update all manual implementations of the `Deserializable` trait.

All deserialization methods take a `name` parameter.
This parameter is used to name the deserialized value.
For instance, in the case of deserialized value from an object field, the name is set to the field name.
However, this parameter has been abused to pass data such as file names between deserializers.

When we introduced macros to automatically implement `Deserializable`, users request a way of passing the name parameter between deserializers.
Then, we added the `passthrough_name` attribute to satisfy the request.
As said before, this was never intended for this usage.
There are some inherent limitations, notably some deserializers such as the deserializers of `Vec` set `name` to the empty string for all deserialized items.

We now introduce **deserialization context** to provide a proper way to pass a filename or any string identifier to all deserializers.
All deserializers now take a context parameter that stores this identifier.
We take the opportunity to integrate diagnostic reporting directly in this deserialization context.

Let's take the deserializer of `Day` presented in the `biome_deserialize` [README](https://github.com/biomejs/biome/tree/main/crates/biome_deserialize) as an example for migrating to the new paradigm.
You have to remove the `diagnostics` parameter and to add a new `ctx` parameter.
Note that the diagnostics are now reported using `ctx.report()`.

```diff
  impl Deserializable for Day {
      fn deserialize(
+         ctx: &mut impl DeserializationContext,
          value: &impl DeserializableValue,
          name: &str,
-         diagnostics: &mut Vec<DeserializationDiagnostic>,
      ) -> Option<Self> {
          // We deserialize the value into a number represented as a string.
          let value_text = TextNumber::deserialize(ctx, value, name)?;
          // We attempt to convert the string into a `Day`.
          value_text.parse::<Day>().map_err(|error| {
              // If the conversion failed, then we report the error.
-             diagnostics.push(DeserializationDiagnostic::new(error).with_range(value.range()));
+             ctx.report(DeserializationDiagnostic::new(error).with_range(value.range()));
          }).ok()
      }
  }
```

Biome provides `DefaultDeserializationContext` as a default implementation of `DeserializationContext`.
It is the implementation used by `biome_deserialize::deserialize_from_json_ast`, `biome_deserialize::deserialize_from_json_str`, or `biome_deserialize::::deserialize_from_str`.
The name (renamed `id`) passed as last parameter to `deserialize_from_json_ast` and `deserialize_from_json_str` now corresponds to the identifier that you can retrieve using `ctx.id()`.
This no longer sets the name of the deserialized root value.
The name of the deserialized root value is now the empty string.

Also, the `passthrough_name` macro attribute was removed because we now have a way of retrieving an identfiier from the context.

### Add the `indexmap` Cargo feature

This is a **breaking change**.

Previously, `biome_deserialize` required `serde` and `indexmap` as dependencies.
Its dependencies are now optional.

If you need the implementation of `Deserializable` for `indexmap::IndexMap` and `indexmap::IndexSet`, then you have to use the `indexmap` feature.
Update your `Cargo.toml` as follows:

```diff
  [dependencies]
- biome_deserialize = { version: "<version>" }
+ biome_deserialize = { version: "<version>", features = ["indexmap"] }
```

### Remove `biome_deserialize::StringSet`

This is a **breaking change**.

`biome_deserialize::StringSet` is now removed.
Use `indexmap::IndexSet<String>` instead.

As a consequence the cargo feature `schema` has been removed.
